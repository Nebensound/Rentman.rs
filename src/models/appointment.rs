//! Appointment resource models.

use super::common::{CommonFields, ResourceRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An appointment in Rentman.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Appointment name
    pub name: Option<String>,
    /// Start time
    pub start: DateTime<Utc>,
    /// End time
    pub end: DateTime<Utc>,
    /// Color
    pub color: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Remark
    pub remark: Option<String>,
    /// Is public
    pub is_public: Option<bool>,
    /// Is plannable (employees can be scheduled during this appointment)
    pub is_plannable: Option<bool>,
}

/// Crew member assigned to an appointment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentCrew {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Appointment reference
    pub appointment: ResourceRef,
    /// Crew member reference
    pub crew: ResourceRef,
}
