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

use rentman_client::{
    EquipmentId, EquipmentPricePayload, RentmanApiToken, RentmanClient, RentmanEquipment, endpoint,
    model,
};
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
#[ignore = "writes to the real Rentman API; requires RENTMAN_API_TOKEN and RENTMAN_LIVE_WRITE_EQUIPMENT_ID"]
async fn equipment_price_write_is_a_no_op_live() {
    let client = live_client();
    // Opt-in: without a designated test item this test writes nothing.
    let Ok(raw_id) = std::env::var("RENTMAN_LIVE_WRITE_EQUIPMENT_ID") else {
        eprintln!("RENTMAN_LIVE_WRITE_EQUIPMENT_ID is not set; skipping the live write test");
        return;
    };
    let id: u64 = raw_id
        .parse()
        .expect("RENTMAN_LIVE_WRITE_EQUIPMENT_ID must be a numeric equipment ID");
    let equipment_id = EquipmentId::new(id);

    let before = active_equipment(&client, equipment_id).await;

    // Partial `PUT` through the facade: only the first set price field is sent,
    // with its current value.
    let payload = no_op_payload(&before);
    client
        .update_equipment_prices(equipment_id, &payload)
        .await
        .expect("live no-op equipment price update succeeds");
    assert_prices_unchanged(&before, &active_equipment(&client, equipment_id).await);

    // The same no-op through a typed endpoint sent exactly once.
    let request: model::EquipmentRequest =
        serde_json::from_value(serde_json::to_value(&payload).expect("payload serializes"))
            .expect("price payload is a valid EquipmentRequest");
    client
        .endpoint::<endpoint::UpdateEquipmentEndpoint>()
        .id(model::RentmanId(id))
        .body(request)
        .expect("request body serializes")
        .send_once()
        .await
        .expect("live no-op equipment update with send_once succeeds");
    assert_prices_unchanged(&before, &active_equipment(&client, equipment_id).await);
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

fn assert_prices_unchanged(before: &RentmanEquipment, after: &RentmanEquipment) {
    assert_eq!(after.price, before.price, "price changed");
    assert_eq!(after.list_price, before.list_price, "list_price changed");
    assert_eq!(
        after.subrental_costs, before.subrental_costs,
        "subrental_costs changed"
    );
}
