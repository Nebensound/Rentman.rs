//! Contact resource models.

use super::common::{CommonFields, CustomFields, ResourceRef};
use serde::{Deserialize, Serialize};

/// A contact (customer/supplier) in Rentman.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Contact name
    pub name: Option<String>,
    /// Folder reference
    pub folder: Option<ResourceRef>,
    /// Account manager reference
    pub account_manager: Option<ResourceRef>,
    /// Is customer
    pub is_customer: Option<bool>,
    /// Is supplier
    pub is_supplier: Option<bool>,
    /// Is location
    pub is_location: Option<bool>,
    /// Mailing street
    pub mailing_street: Option<String>,
    /// Mailing number
    pub mailing_number: Option<String>,
    /// Mailing city
    pub mailing_city: Option<String>,
    /// Mailing postal code
    pub mailing_postalcode: Option<String>,
    /// Mailing country
    pub mailing_country: Option<String>,
    /// Phone 1
    pub phone_1: Option<String>,
    /// Phone 2
    pub phone_2: Option<String>,
    /// Email 1
    pub email_1: Option<String>,
    /// Email 2
    pub email_2: Option<String>,
    /// Website
    pub website: Option<String>,
    /// VAT code
    #[serde(rename = "VAT_code")]
    pub vat_code: Option<String>,
    /// Remark/notes
    pub remark: Option<String>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}

/// A contact person within a contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub common: CommonFields,
    /// Parent contact reference
    pub contact: ResourceRef,
    /// First name
    pub firstname: Option<String>,
    /// Middle name
    pub middle_name: Option<String>,
    /// Last name
    pub lastname: Option<String>,
    /// Function/title
    pub function: Option<String>,
    /// Phone number
    pub phone: Option<String>,
    /// Mobile phone
    pub mobilephone: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Street
    pub street: Option<String>,
    /// House number
    pub number: Option<String>,
    /// City
    pub city: Option<String>,
    /// Postal code
    pub postalcode: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Tags
    pub tags: Option<String>,
    /// Custom fields
    pub custom: Option<CustomFields>,
}
