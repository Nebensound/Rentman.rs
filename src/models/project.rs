//! Project resource models.

use super::common::{CommonFields, CustomFields, ResourceRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A project in Rentman.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Project name
    pub name: Option<String>,
    /// Project number (auto-generated)
    pub number: Option<i32>,
    /// Project reference
    pub reference: Option<String>,
    /// Customer contact reference
    pub customer: Option<ResourceRef>,
    /// Location contact reference
    pub location: Option<ResourceRef>,
    /// Customer contact person reference
    pub cust_contact: Option<ResourceRef>,
    /// Location contact person reference
    pub loc_contact: Option<ResourceRef>,
    /// Project type reference
    #[serde(rename = "project_type")]
    pub project_type: Option<ResourceRef>,
    /// Account manager reference
    pub account_manager: Option<ResourceRef>,
    /// Project color (hex without #)
    pub color: Option<String>,
    /// Usage period start
    pub usageperiod_start: Option<DateTime<Utc>>,
    /// Usage period end
    pub usageperiod_end: Option<DateTime<Utc>>,
    /// Plan period start
    pub planperiod_start: Option<DateTime<Utc>>,
    /// Plan period end
    pub planperiod_end: Option<DateTime<Utc>>,
    /// Equipment period from
    pub equipment_period_from: Option<DateTime<Utc>>,
    /// Equipment period to
    pub equipment_period_to: Option<DateTime<Utc>>,
    /// Terms and conditions
    pub conditions: Option<String>,
    /// Tags
    pub tags: Option<String>,
    /// Refundable deposit amount
    pub refundabledeposit: Option<f64>,
    /// Deposit status
    pub deposit_status: Option<String>,
    /// Already invoiced amount
    pub already_invoiced: Option<f64>,
    /// Total weight
    pub weight: Option<f64>,
    /// Total power consumption
    pub power: Option<f64>,
    /// Total current
    pub current: Option<f64>,
    /// Total volume
    pub volume: Option<f64>,
    /// Purchase costs
    pub purchasecosts: Option<f64>,
    /// Update hash for optimistic locking
    #[serde(rename = "updateHash")]
    pub update_hash: Option<String>,
    /// Remark/notes
    pub remark: Option<String>,
    /// Is archived
    pub archived: Option<bool>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}

/// Data for creating a new project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    /// Project name
    pub name: String,
    /// Project reference (optional)
    pub reference: Option<String>,
    /// Project number (optional, usually auto-generated)
    pub number: Option<i32>,
    /// Custom fields (optional)
    pub custom: Option<CustomFields>,
}

/// Data for updating a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProject {
    /// Project name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Project reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Project number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Customer contact reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<ResourceRef>,
    /// Project type reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projecttype: Option<ResourceRef>,
    /// Custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomFields>,
}

/// A subproject within a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subproject {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Parent project reference
    pub project: ResourceRef,
    /// Subproject name
    pub name: Option<String>,
    /// Location contact reference
    pub location: Option<ResourceRef>,
    /// Usage period start
    pub usageperiod_start: Option<DateTime<Utc>>,
    /// Usage period end
    pub usageperiod_end: Option<DateTime<Utc>>,
    /// In time
    #[serde(rename = "in")]
    pub in_time: Option<DateTime<Utc>>,
    /// Out time
    pub out: Option<DateTime<Utc>>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}
