//! Typed Rentman API request and response models.
//!
//! Resource items such as [`Crew`] are returned directly by the client;
//! every full resource item is a read snapshot containing both its typed `id`
//! and its typed `revision`, not just its data fields. The ID identifies an
//! item; the revision identifies the observed version of that item. Store both
//! to detect updates and reuse unchanged data. All full resources implement
//! [`Revisioned`](crate::Revisioned); see the [revision guide](crate#change-detection).
//!
//! Resource models are read snapshots;
//! their `Payload` counterparts describe writes. Payloads with required fields
//! have a `new` constructor; other payloads implement `Default`. Optional
//! payload fields left as `None` are not sent.
//!
//! See the crate's [type guide](crate#api-values-and-rust-types) for ID, decimal,
//! enum, and null handling, and [`projection!`](crate::projection) for smaller
//! response structs. Each field describes its Rust value and presence rules;
//! Rentman's representation and original schema name follow as background.
//! Enum references list all supported variants, including their transmitted
//! values. Descriptions supplied by Rentman are retained; a technical fallback
//! does not imply additional business semantics.
#![doc = include_str!("model/resource_names.md")]

mod generated;

pub use generated::*;
