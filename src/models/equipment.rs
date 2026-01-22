//! Equipment resource models.

use super::common::{CommonFields, CustomFields, ResourceRef};
use serde::{Deserialize, Serialize};

/// Equipment item in Rentman.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Equipment name
    pub name: Option<String>,
    /// Display name
    pub displayname: Option<String>,
    /// Folder reference
    pub folder: Option<ResourceRef>,
    /// External remark (visible to customers)
    pub external_remark: Option<String>,
    /// Internal remark
    pub remark: Option<String>,
    /// Is rental item
    pub is_rental: Option<bool>,
    /// Is sale item
    pub is_sale: Option<bool>,
    /// Is case (container)
    pub is_case: Option<bool>,
    /// Is serialized (tracked individually)
    pub is_serialized: Option<bool>,
    /// Rental price
    pub price: Option<f64>,
    /// Sale price
    pub price_sale: Option<f64>,
    /// Rental rate type
    pub price_type: Option<String>,
    /// In stock quantity
    pub in_stock: Option<i32>,
    /// Minimum stock level
    pub min_stock: Option<i32>,
    /// Surface area in m²
    pub surface: Option<f64>,
    /// Height in meters
    pub height: Option<f64>,
    /// Weight in kg
    pub weight: Option<f64>,
    /// Volume in m³
    pub volume: Option<f64>,
    /// Power consumption in watts
    pub power: Option<i32>,
    /// Tax class reference
    pub taxclass: Option<ResourceRef>,
    /// Ledger code reference
    pub ledger: Option<ResourceRef>,
    /// Tags
    pub tags: Option<String>,
    /// QR codes
    pub qrcodes: Option<String>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}

/// Serial number for serialized equipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialNumber {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Parent equipment reference
    pub equipment: ResourceRef,
    /// Serial number
    pub serial: String,
    /// Purchase date
    pub purchasedate: Option<String>,
    /// Purchase costs
    pub purchase_costs: Option<f64>,
    /// Is active
    pub active: Option<bool>,
    /// Remark
    pub remark: Option<String>,
    /// Reference number
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    /// Asset location reference
    pub asset_location: Option<ResourceRef>,
    /// Tags
    pub tags: Option<String>,
    /// QR codes
    pub qrcodes: Option<String>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}

/// Equipment planned in a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEquipment {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Equipment reference
    pub equipment: ResourceRef,
    /// Subproject reference
    pub subproject: ResourceRef,
    /// Quantity
    pub quantity: Option<i32>,
    /// Rental price
    pub price: Option<f64>,
    /// Sale price
    pub price_sale: Option<f64>,
    /// Is packed
    pub is_packed: Option<bool>,
    /// Order
    pub order: Option<i32>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}
