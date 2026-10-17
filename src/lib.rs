#![doc = include_str!("../crates-io.md")]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod endpoint;
pub mod model;

mod amount;
mod client;
mod config;
mod endpoint_request;
mod equipment;
mod error;
mod ids;
mod invoices;
#[cfg(test)]
// Test helpers are not production code and do not count toward coverage.
#[cfg_attr(coverage_nightly, coverage(off))]
mod test_support;
mod transport;

pub use amount::RentmanAmount;
pub use client::RentmanClient;
pub use config::{
    ConfigError, MAX_ATTEMPTS_VAR, MIN_REQUEST_INTERVAL_VAR, RETRY_BASE_DELAY_VAR,
    RentmanClientConfig,
};
pub use endpoint_request::RentmanEndpointRequest;
pub use equipment::{EquipmentPricePayload, RentmanEquipment};
pub use error::{Error, ErrorKind, InvalidRequest, RequestOutcome, Resource, Result};
pub use ids::{EquipmentId, InvoiceId, InvoiceNumber, PaymentId, RentmanApiToken};
pub use invoices::{
    InvoicePaymentStatus, PaymentImportSource, RentmanInvoice, RentmanPayment,
    RentmanPaymentPayload,
};
/// Re-export of the URL type used for client configuration.
pub use reqwest::Url;
