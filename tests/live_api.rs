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
    EquipmentId, EquipmentPricePayload, RentmanApiToken, RentmanClient, RentmanEquipment, endpoint,
    model,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::sync::OnceLock;

/// Rows per page for the `in_archive` filter checks: enough to contain both
/// archived and active items in a real account.
const FILTER_CHECK_LIMIT: usize = 500;

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

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn high_level_invoice_and_payment_listing_works_live() {
    let client = live_client();

    let invoices = client
        .all_invoices()
        .await
        .expect("live invoice listing succeeds");
    let payments = client
        .all_payments_by_invoice()
        .await
        .expect("live payment listing succeeds");

    for invoice_id in payments.keys() {
        assert!(
            invoices.iter().any(|invoice| invoice.id == *invoice_id),
            "payment references invoice {invoice_id} that the invoice listing does not contain"
        );
    }
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn active_equipment_listing_works_and_excludes_archived_items_live() {
    let client = live_client();

    // Decodes every active item through `RentmanEquipment`/`RentmanAmount`.
    let active = client
        .all_active_equipment()
        .await
        .expect("live active equipment listing succeeds");

    // Rentman applies `in_archive=0` (the value the facade sends) as a filter:
    // every returned row is active ...
    let unarchived = equipment_page(&client, "0").await;
    assert!(
        unarchived.iter().all(|item| !item.in_archive),
        "in_archive=0 returned archived equipment, so Rentman ignores the filter value"
    );
    // ... and the opposite value returns only archived rows. A filter that is
    // ignored would mix both and fail one of the two checks.
    let archived = equipment_page(&client, "1").await;
    assert!(
        archived.iter().all(|item| item.in_archive),
        "in_archive=1 returned active equipment, so Rentman ignores the filter value"
    );

    for item in &archived {
        assert!(
            !active.iter().any(|equipment| equipment.id == item.id),
            "all_active_equipment() returned archived equipment {}",
            item.id
        );
    }
}

/// Loads one page of equipment with the given `in_archive` filter value.
///
/// Full rows are requested: the generated `Equipment` requires its
/// documented fields, so a `fields` selection would not decode.
async fn equipment_page(client: &RentmanClient, in_archive: &str) -> Vec<model::Equipment> {
    client
        .endpoint::<endpoint::GetEquipmentCollectionEndpoint>()
        .query_param("in_archive", in_archive)
        .query_param("limit", FILTER_CHECK_LIMIT)
        .send()
        .await
        .unwrap_or_else(|error| {
            panic!("equipment listing with in_archive={in_archive} failed: {error:#}")
        })
        .data
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

    let before = active_equipment(&client, equipment_id).await;

    // Partial `PUT` through the facade: only the first set price field is sent,
    // with its current value. `before` went through the amount deserializer,
    // which rounds over-precise values; so first prove against the stored JSON
    // text that exactly the stored value would be written.
    let payload = no_op_payload(&before);
    let stored = stored_prices(id).await;
    assert_payload_matches_stored(&payload, &stored);
    client
        .update_equipment_prices(equipment_id, &payload)
        .await
        .expect("live no-op equipment price update succeeds");
    assert_prices_unchanged(&before, &active_equipment(&client, equipment_id).await);

    // The same no-op through a typed endpoint sent exactly once. The generated
    // request goes through `serde_json::Value` and `Decimal`'s float
    // serialization, so first check that exactly the current prices would be
    // sent; otherwise this write would change the test item.
    let request: model::EquipmentPayload =
        serde_json::from_value(serde_json::to_value(&payload).expect("payload serializes"))
            .expect("price payload is a valid EquipmentPayload");
    assert_typed_request_is_exact(&request, &payload);
    client
        .endpoint::<endpoint::UpdateEquipmentEndpoint>()
        .id(rentman_client::Id::new(id))
        .body(request)
        .expect("request body serializes")
        .send_once()
        .await
        .expect("live no-op equipment update with send_once succeeds");
    assert_prices_unchanged(&before, &active_equipment(&client, equipment_id).await);
    assert_eq!(
        stored_prices(id).await,
        stored,
        "the stored price JSON changed"
    );
}

/// The stored price fields of one equipment item as exact decimals, read from
/// the raw JSON text without going through `RentmanAmount` or `f64`.
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

/// Refuses the write unless every price in the payload equals the stored
/// value exactly.
fn assert_payload_matches_stored(payload: &EquipmentPricePayload, stored: &StoredPrices) {
    for (field, sent, stored) in [
        ("price", payload.price, &stored.price),
        ("list_price", payload.list_price, &stored.list_price),
        (
            "subrental_costs",
            payload.subrental_costs,
            &stored.subrental_costs,
        ),
    ] {
        let Some(sent) = sent else {
            continue;
        };
        let sent: BigDecimal = sent
            .as_decimal()
            .to_string()
            .parse()
            .expect("a decimal is a BigDecimal");
        assert_eq!(
            Some(&sent),
            stored.as_ref(),
            "decoding changed the stored {field}; refusing the live write"
        );
    }
}

async fn active_equipment(client: &RentmanClient, id: EquipmentId) -> RentmanEquipment {
    client
        .all_active_equipment()
        .await
        .expect("live active equipment listing succeeds")
        .into_iter()
        .find(|equipment| equipment.id == id)
        .unwrap_or_else(|| panic!("test equipment {id} must exist and not be archived"))
}

fn no_op_payload(equipment: &RentmanEquipment) -> EquipmentPricePayload {
    let mut payload = EquipmentPricePayload::default();
    if equipment.price.is_some() {
        payload.price = equipment.price;
    } else if equipment.list_price.is_some() {
        payload.list_price = equipment.list_price;
    } else {
        payload.subrental_costs = equipment.subrental_costs;
    }
    assert!(
        !payload.is_empty(),
        "test equipment {} needs at least one price field set",
        equipment.id
    );
    payload
}

/// Refuses the typed write unless the body that `.body()` would build carries
/// exactly the payload's prices.
fn assert_typed_request_is_exact(
    request: &model::EquipmentPayload,
    payload: &EquipmentPricePayload,
) {
    let body = serde_json::to_value(request).expect("typed request serializes");
    for (field, expected) in [
        ("price", payload.price),
        ("list_price", payload.list_price),
        ("subrental_costs", payload.subrental_costs),
    ] {
        let sent = body.get(field).map(|value| {
            Decimal::from_str_exact(&value.to_string()).expect("sent price is a decimal")
        });
        assert_eq!(
            sent,
            expected.map(|amount| amount.as_decimal()),
            "the typed request would not send {field} exactly; refusing the live write"
        );
    }
}

fn assert_prices_unchanged(before: &RentmanEquipment, after: &RentmanEquipment) {
    assert_eq!(after.price, before.price, "price changed");
    assert_eq!(after.list_price, before.list_price, "list_price changed");
    assert_eq!(
        after.subrental_costs, before.subrental_costs,
        "subrental_costs changed"
    );
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn filters_survive_the_cursor_live() {
    use rentman_client::{PageSize, fields::equipment};

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
    let mut item: ProjectedItem = serde_json::from_value(serde_json::json!({
        "id": 3, "name": "Mixer", "in_archive": true, "creator": "/crew/2", "custom": {}
    }))
    .unwrap();
    let equipment = EquipmentName::decode(&mut item).unwrap();
    assert_eq!(equipment.id, rentman_client::EquipmentId::new(3));
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
        let mut item: ProjectedItem = serde_json::from_value(wire).unwrap();
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
    use rentman_client::{PageSize, fields::equipment};

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
