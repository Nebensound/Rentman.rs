//! Live tests against the real Rentman API.
//!
//! These tests are ignored by default so `cargo test` never needs real
//! credentials. Run them explicitly with a real API token:
//!
//! ```bash
//! RENTMAN_API_TOKEN=... cargo test --test live_api -- --ignored
//! ```

#[path = "live/generated_sweep.rs"]
mod generated_sweep;

use rentman_client::{RentmanApiToken, RentmanClient};

fn live_client() -> RentmanClient {
    let token = std::env::var("RENTMAN_API_TOKEN")
        .expect("RENTMAN_API_TOKEN must be set to run live Rentman API tests");
    RentmanClient::new(RentmanApiToken::new(token))
}

#[tokio::test]
#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]
async fn every_get_collection_endpoint_deserializes_live_data() {
    let failures = generated_sweep::sweep_collections(&live_client()).await;

    assert!(
        failures.is_empty(),
        "live responses no longer match the typed models for {} operations:\n{}",
        failures.len(),
        failures
            .iter()
            .map(|(operation, error)| format!("- {operation}: {error}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
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
