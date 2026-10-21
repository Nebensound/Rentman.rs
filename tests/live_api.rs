//! Live tests against the real Rentman API.
//!
//! These tests are ignored by default unless `RENTMAN_API_TOKEN` is set in the
//! environment or in a git-ignored `.env` file in the crate root (see
//! `.env.example`). When a token is present, a normal `cargo test` runs them:
//!
//! ```bash
//! cp .env.example .env   # then fill in RENTMAN_API_TOKEN
//! cargo test
//! ```
//!
//! A `RENTMAN_API_TOKEN` already set in the environment takes precedence over
//! the `.env` file. Cargo watches both sources and rebuilds the test binary
//! when token availability changes.
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
            // Use the same crate-root file as the build-time test selection.
            match dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")) {
                Ok(()) => {}
                Err(error) if error.not_found() => {}
                Err(_) => panic!("could not load crate-root .env for live API tests"),
            }
            let token = std::env::var("RENTMAN_API_TOKEN").expect(
                "RENTMAN_API_TOKEN must be set (environment or .env) to run live Rentman API tests",
            );
            RentmanClient::new(RentmanApiToken::new(token))
        })
        .clone()
}

#[tokio::test]
#[cfg_attr(
    not(live_api_tests_enabled),
    ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"
)]
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
#[cfg_attr(
    not(live_api_tests_enabled),
    ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"
)]
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
            !active
                .iter()
                .any(|equipment| equipment.id == EquipmentId::new(item.id.0)),
            "all_active_equipment() returned archived equipment {}",
            item.id
        );
    }
}

/// Loads one page of equipment with the given `in_archive` filter value.
///
/// Full rows are requested: the generated `EquipmentResponse` requires its
/// documented fields, so a `fields` selection would not decode.
async fn equipment_page(client: &RentmanClient, in_archive: &str) -> Vec<model::EquipmentResponse> {
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
#[cfg_attr(
    not(live_api_tests_enabled),
    ignore = "writes to the real Rentman API; requires RENTMAN_API_TOKEN and RENTMAN_LIVE_WRITE_EQUIPMENT_ID"
)]
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
    let request: model::EquipmentRequest =
        serde_json::from_value(serde_json::to_value(&payload).expect("payload serializes"))
            .expect("price payload is a valid EquipmentRequest");
    assert_typed_request_is_exact(&request, &payload);
    client
        .endpoint::<endpoint::UpdateEquipmentEndpoint>()
        .id(model::RentmanId(id))
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
    request: &model::EquipmentRequest,
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
