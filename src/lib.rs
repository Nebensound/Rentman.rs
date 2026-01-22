//! # Rentman API Client
//!
//! Rust client library for the Rentman API.
//!
//! ## Example
//!
//! ```no_run
//! use rentman::RentmanClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RentmanClient::builder()
//!         .token("your-api-token")
//!         .build()?;
//!     
//!     let projects = client.projects().list().await?;
//!     println!("Found {} projects", projects.len());
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs, rust_2018_idioms)]

pub mod client;
pub mod error;
pub mod models;
pub mod endpoints;
pub mod prelude;

// Pagination support is experimental
#[doc(hidden)]
pub mod pagination;

pub use client::{RentmanClient, RentmanClientBuilder};
pub use error::{Error, Result};
