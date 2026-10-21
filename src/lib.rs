#![doc = include_str!("../crates-io.md")]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod fields;
pub mod model;
pub mod projection_fields;
pub mod resource;

mod amount;
mod client;
mod config;
mod endpoint;
mod error;
mod filter;
mod id;
mod ids;
mod paging;
mod projection;
mod revision;
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
pub use error::{Error, ErrorKind, InvalidRequest, RequestOutcome, ResourceRef, Result};
pub use filter::{Field, Filter, FilterValue, Sort};
pub use id::{Id, ItemId, LinkError, Resource};
pub use ids::RentmanApiToken;
pub use paging::PageSize;
pub use projection::{Projection, ProjectionField};
/// Re-export of the URL type used for client configuration.
pub use reqwest::Url;
pub use revision::{Changes, InvalidRevision, Revision, Revisioned, Revisions};

/// Items used by the code `projection!` expands to; not a public API.
#[doc(hidden)]
pub mod __private {
    pub use crate::projection::{IntoValue, Link, ProjectedItem};
    pub use serde_json;
}
