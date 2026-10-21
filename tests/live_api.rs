//! Live tests against the real Rentman API.
//!
//! These tests are ignored by default so `cargo test` never needs real
//! credentials. Run them explicitly with a real API token, either from the
//! environment or from a git-ignored `.env` file in the crate root (see
//! `.env.example`):
//!
//! ```bash
//! cp .env.example .env   # then fill in RENTMAN_API_TOKEN
//! cargo test --test live_api -- --ignored
//! ```
//!
//! A `RENTMAN_API_TOKEN` already set in the environment takes precedence over
//! the `.env` file.
//!
//! Every documented `GET` collection endpoint has its own generated test in
//! `live/generated_sweep.rs`, so a drifted endpoint shows up as an individual
//! test failure.

#[path = "live/generated_sweep.rs"]
mod generated_sweep;

use bigdecimal::BigDecimal;
use rentman_client::{
    PageSize, RentmanApiToken, RentmanClient,
    fields::equipment,
    model::{self, EquipmentId},
};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{collections::BTreeSet, sync::OnceLock};

/// Shared client so all parallel tests go through one rate limiter.
fn live_client() -> RentmanClient {
    static CLIENT: OnceLock<RentmanClient> = OnceLock::new();
    CLIENT
        .get_or_init(|| {
            // A missing `.env` is fine: CI provides the token via the environment.
            dotenvy::dotenv().ok();
            let token = std::env::var("RENTMAN_API_TOKEN").expect(
                "RENTMAN_API_TOKEN must be set (environment or .env) to run live Rentman API tests",
            );
            RentmanClient::new(RentmanApiToken::new(token))
        })
        .clone()
}

rentman_client::projection! {
    /// An invoice, reduced to its ID.
    struct InvoiceRef from Invoice { id }
}

rentman_client::projection! {
    /// A payment and the invoice it belongs to.
    struct PaymentRef from Payment { id, invoice }
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn invoice_and_payment_listing_works_live() {
    let client = live_client();

    let invoices = client
        .invoices()
        .list()
        .page_size(PageSize::MAX)
        .all_as::<InvoiceRef>()
        .await
        .expect("live invoice listing succeeds")
        .into_iter()
        .map(|invoice| invoice.id)
        .collect::<BTreeSet<_>>();
    let payments = client
        .payments()
        .list()
        .page_size(PageSize::MAX)
        .all_as::<PaymentRef>()
        .await
        .expect("live payment listing succeeds");

    for payment in payments {
        assert!(
            invoices.contains(&payment.invoice),
            "payment {} references invoice {} that the invoice listing does not contain",
            payment.id,
            payment.invoice
        );
    }
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn in_archive_filter_is_applied_live() {
    let client = live_client();
    let page = |in_archive| {
        client
            .equipment()
            .list()
            .filter(equipment::IN_ARCHIVE.eq(in_archive))
            .first(500)
    };

    // Rentman applies the filter (sent as `1`/`0`): every row matches. A
    // filter Rentman ignored would mix both kinds and fail one check.
    let active = page(false)
        .await
        .expect("active equipment listing succeeds");
    assert!(
        active.iter().all(|item| !item.in_archive),
        "in_archive=0 returned archived equipment"
    );
    let archived = page(true)
        .await
        .expect("archived equipment listing succeeds");
    assert!(
        archived.iter().all(|item| item.in_archive),
        "in_archive=1 returned active equipment"
    );
}

rentman_client::projection! {
    /// The price fields of an equipment item.
    struct EquipmentPrices from Equipment { id, price, list_price, subrental_costs }
}

#[tokio::test]
#[ignore = "writes to the real Rentman API; requires RENTMAN_API_TOKEN and RENTMAN_LIVE_WRITE_EQUIPMENT_ID"]
async fn equipment_price_write_is_a_no_op_live() {
    let client = live_client();
    // Opt-in: without a designated test item this test writes nothing. The
    // live workflow always exports the variable, so an unset repository
    // variable arrives as an empty string; `.env.example` leaves it empty too.
    let raw_id = std::env::var("RENTMAN_LIVE_WRITE_EQUIPMENT_ID").unwrap_or_default();
    let raw_id = raw_id.trim();
    if raw_id.is_empty() {
        eprintln!("RENTMAN_LIVE_WRITE_EQUIPMENT_ID is not set; skipping the live write test");
        return;
    }
    let id: u64 = raw_id
        .parse()
        .expect("RENTMAN_LIVE_WRITE_EQUIPMENT_ID must be a numeric equipment ID");
    let equipment_id = EquipmentId::new(id);

    let before: EquipmentPrices = client
        .equipment()
        .get_as(equipment_id)
        .await
        .expect("live equipment lookup succeeds");
    // A partial `PUT` that sends only the current price. Amounts are read
    // exactly, but first prove against the stored JSON text that exactly the
    // stored value would be written.
    let payload = model::EquipmentPayload {
        price: Some(before.price),
        ..model::EquipmentPayload::default()
    };
    let stored = stored_prices(id).await;
    let sent: BigDecimal = before
        .price
        .as_decimal()
        .to_string()
        .parse()
        .expect("a decimal is a BigDecimal");
    assert_eq!(
        Some(&sent),
        stored.price.as_ref(),
        "decoding changed the stored price; refusing the live write"
    );

    let updated = client
        .equipment()
        .update(equipment_id, &payload)
        .await
        .expect("live no-op equipment update succeeds");
    assert_eq!(updated.price, before.price, "price changed");
    assert_eq!(updated.list_price, before.list_price, "list_price changed");
    assert_eq!(
        updated.subrental_costs, before.subrental_costs,
        "subrental_costs changed"
    );
    assert_eq!(
        stored_prices(id).await,
        stored,
        "the stored price JSON changed"
    );
}

/// The stored price fields of one equipment item as exact decimals, read from
/// the raw JSON text without going through the client.
#[derive(Debug, PartialEq)]
struct StoredPrices {
    price: Option<BigDecimal>,
    list_price: Option<BigDecimal>,
    subrental_costs: Option<BigDecimal>,
}

async fn stored_prices(id: u64) -> StoredPrices {
    #[derive(Deserialize)]
    struct Envelope<'a> {
        #[serde(borrow)]
        data: RawPrices<'a>,
    }
    #[derive(Deserialize)]
    struct RawPrices<'a> {
        #[serde(borrow, default)]
        price: Option<&'a RawValue>,
        #[serde(borrow, default)]
        list_price: Option<&'a RawValue>,
        #[serde(borrow, default)]
        subrental_costs: Option<&'a RawValue>,
    }

    // Same API and token as `live_client()`, which has loaded `.env`.
    let token = std::env::var("RENTMAN_API_TOKEN").expect("RENTMAN_API_TOKEN is set");
    let body = reqwest::Client::new()
        .get(format!(
            "https://api.rentman.net/equipment/{id}?fields=price,list_price,subrental_costs"
        ))
        .bearer_auth(token)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .expect("raw equipment lookup succeeds")
        .text()
        .await
        .expect("raw equipment response is text");
    let raw: Envelope<'_> = serde_json::from_str(&body).expect("raw equipment response is JSON");
    let exact = |value: Option<&RawValue>| {
        value.map(|value| {
            value
                .get()
                .parse::<BigDecimal>()
                .expect("stored price is a JSON number")
        })
    };
    StoredPrices {
        price: exact(raw.data.price),
        list_price: exact(raw.data.list_price),
        subrental_costs: exact(raw.data.subrental_costs),
    }
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn filters_survive_the_cursor_live() {
    let client = live_client();
    let archived = || {
        client
            .equipment()
            .list()
            .filter(equipment::IN_ARCHIVE.eq(true))
    };
    // Small pages make the filter travel through several cursors.
    let paged = archived()
        .page_size(PageSize::new(100).unwrap())
        .all()
        .await
        .unwrap();
    let single = archived().first(1500).await.unwrap();

    assert!(
        paged.iter().all(|item| item.in_archive),
        "an active item leaked through a cursor"
    );
    if single.len() < 1500 {
        let ids = |items: &[rentman_client::model::Equipment]| {
            let mut ids = items.iter().map(|item| item.id).collect::<Vec<_>>();
            ids.sort();
            ids
        };
        assert_eq!(ids(&paged), ids(&single));
    }
}

rentman_client::projection! {
    /// The fields of an equipment item this test selects.
    struct EquipmentName from Equipment { id, name, in_archive, creator }
}

/// Runs without credentials: the projection used by the live test below
/// expands and decodes from outside the crate.
#[test]
fn equipment_projection_decodes_outside_the_crate() {
    use rentman_client::{__private::ProjectedItem, Projection};

    assert_eq!(
        EquipmentName::FIELDS,
        ["id", "name", "in_archive", "creator"]
    );
    let mut item: ProjectedItem = serde_json::from_str(
        r#"{"id": 3, "name": "Mixer", "in_archive": true, "creator": "/crew/2", "custom": {}}"#,
    )
    .unwrap();
    let equipment = EquipmentName::decode(&mut item).unwrap();
    assert_eq!(equipment.id, EquipmentId::new(3));
    assert_eq!(equipment.name, "Mixer");
    assert!(equipment.in_archive);
    assert_eq!(
        equipment.creator,
        Some(rentman_client::model::CrewId::new(2))
    );

    // Every field that does not decode fails with its name.
    let valid =
        serde_json::json!({"id": 3, "name": "Mixer", "in_archive": true, "creator": "/crew/2"});
    for (field, broken) in [
        ("id", serde_json::json!("three")),
        ("name", serde_json::Value::Null),
        ("in_archive", serde_json::json!("yes")),
        ("creator", serde_json::json!("/projects/2")),
    ] {
        let mut wire = valid.clone();
        wire[field] = broken;
        let mut item: ProjectedItem = serde_json::from_str(&wire.to_string()).unwrap();
        let error = EquipmentName::decode(&mut item).unwrap_err();
        assert!(
            error.to_string().starts_with(&format!("field `{field}`")),
            "{error}"
        );
    }
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn projections_keep_their_field_selection_across_the_cursor_live() {
    let client = live_client();
    let projected = client
        .equipment()
        .list()
        .filter(equipment::IN_ARCHIVE.eq(true))
        .page_size(PageSize::new(100).unwrap())
        .all_as::<EquipmentName>()
        .await
        .unwrap();
    let full = client
        .equipment()
        .list()
        .filter(equipment::IN_ARCHIVE.eq(true))
        .page_size(PageSize::MAX)
        .all()
        .await
        .unwrap();

    assert_eq!(projected.len(), full.len());
    for (projected, full) in projected.iter().zip(&full) {
        assert_eq!(projected.id, full.id);
        assert_eq!(projected.name, full.name);
        assert!(projected.in_archive);
        assert_eq!(projected.creator, full.creator);
    }
}
