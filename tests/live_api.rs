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

use rentman_client::{RentmanApiToken, RentmanClient};
use std::sync::OnceLock;

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
