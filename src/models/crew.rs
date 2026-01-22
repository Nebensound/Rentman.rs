//! Crew member resource models.

use super::common::{CommonFields, CustomFields, ResourceRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A crew member in Rentman.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crew {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Folder reference
    pub folder: Option<ResourceRef>,
    /// Street address
    pub street: Option<String>,
    /// House number
    pub housenumber: Option<String>,
    /// City
    pub city: Option<String>,
    /// Postal code
    pub postal_code: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Birth date
    pub birthdate: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Mobile phone
    pub mobile: Option<String>,
    /// Phone
    pub phone: Option<String>,
    /// Emergency contact
    pub emergency_contact: Option<String>,
    /// Remark/notes
    pub remark: Option<String>,
    /// Is active
    pub active: Option<bool>,
    /// Is visible in planning
    pub is_visible_planner: Option<bool>,
    /// Is freelancer
    pub is_freelancer: Option<bool>,
    /// Is management
    pub is_management: Option<bool>,
    /// Is driver
    pub is_driver: Option<bool>,
    /// Tags
    pub tags: Option<String>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}

/// Crew availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewAvailability {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Crew member reference
    pub crew: ResourceRef,
    /// Start time
    pub start: DateTime<Utc>,
    /// End time
    pub end: DateTime<Utc>,
    /// Remark
    pub remark: Option<String>,
}

/// Crew member planned for a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCrew {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Function reference
    pub function: ResourceRef,
    /// Crew member reference
    pub crewmember: ResourceRef,
    /// Cost rate reference
    pub cost_rate: Option<ResourceRef>,
    /// Is visible
    pub visible: Option<bool>,
    /// Plan period start
    pub planperiod_start: Option<DateTime<Utc>>,
    /// Plan period end
    pub planperiod_end: Option<DateTime<Utc>>,
    /// Transport type
    pub transport: Option<String>,
    /// Remark
    pub remark: Option<String>,
    /// Planner remark
    pub remark_planner: Option<String>,
    /// Is project leader
    pub project_leader: Option<bool>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}
