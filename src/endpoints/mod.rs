//! Endpoint implementations for Rentman API resources.

pub mod projects;
pub mod contacts;
pub mod equipment;

pub use projects::ProjectsEndpoint;
pub use contacts::ContactsEndpoint;
pub use equipment::EquipmentEndpoint;
