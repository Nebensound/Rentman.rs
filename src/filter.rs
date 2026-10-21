//! Typed filters and sorting for resource lists.
//!
//! The generated module [`fields`](crate::fields) has one constant per
//! queryable field of every resource, e.g. `fields::contact::COUNTRY`. A
//! constant knows its resource and value type, so a filter for another
//! resource or with a value of the wrong type does not compile:
//!
//! ```
//! # use rentman_client::{RentmanApiToken, RentmanClient, fields::contact, model};
//! # async fn example(client: RentmanClient) -> rentman_client::Result<()> {
//! let contacts: Vec<model::Contact> = client
//!     .contacts()
//!     .list()
//!     .filter(contact::COUNTRY.eq(model::ContactCountry::Gb))
//!     .filter(contact::FOLDER.is_null(false))
//!     .sort(contact::NAME.asc())
//!     .all()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Fields Rentman marks as `GENERATED FIELD` can neither be filtered nor
//! sorted on and have no constant.
//!
//! A value of the wrong type does not compile:
//!
//! ```compile_fail
//! # use rentman_client::{RentmanClient, fields::contact};
//! # fn example(client: RentmanClient) {
//! let _ = client.contacts().list().filter(contact::DISTANCE.lte("300".to_string()));
//! # }
//! ```
//!
//! Neither does a field of another resource:
//!
//! ```compile_fail
//! # use rentman_client::{RentmanClient, fields::crew};
//! # fn example(client: RentmanClient) {
//! let _ = client.contacts().list().filter(crew::FIRSTNAME.eq("Ada".to_string()));
//! # }
//! ```

use crate::{
    Decimal,
    id::{Id, ItemId},
    model::EmailAddress,
};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use std::{fmt, marker::PhantomData};

/// A queryable field of resource `R` with values of type `V`.
pub struct Field<R, V> {
    pub(crate) name: &'static str,
    pub(crate) types: PhantomData<fn() -> (R, V)>,
}

impl<R, V> Clone for Field<R, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R, V> Copy for Field<R, V> {}

impl<R, V> fmt::Debug for Field<R, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field({})", self.name)
    }
}

impl<R, V: FilterValue> Field<R, V> {
    /// Rentman's name of the field, e.g. `country`.
    pub fn name(self) -> &'static str {
        self.name
    }

    /// Items whose field equals `value` (`field=value`).
    pub fn eq(self, value: V) -> Filter<R> {
        self.filter("", &value)
    }

    /// Items whose field does not equal `value` (`field[neq]=value`).
    pub fn neq(self, value: V) -> Filter<R> {
        self.filter("[neq]", &value)
    }

    /// Items whose field is less than `value` (`field[lt]=value`).
    pub fn lt(self, value: V) -> Filter<R> {
        self.filter("[lt]", &value)
    }

    /// Items whose field is greater than `value` (`field[gt]=value`).
    pub fn gt(self, value: V) -> Filter<R> {
        self.filter("[gt]", &value)
    }

    /// Items whose field is at most `value` (`field[lte]=value`).
    pub fn lte(self, value: V) -> Filter<R> {
        self.filter("[lte]", &value)
    }

    /// Items whose field is at least `value` (`field[gte]=value`).
    pub fn gte(self, value: V) -> Filter<R> {
        self.filter("[gte]", &value)
    }

    /// Items whose field is (`true`) or is not (`false`) null
    /// (`field[isnull]=1` or `0`).
    pub fn is_null(self, is_null: bool) -> Filter<R> {
        self.filter("[isnull]", &is_null)
    }

    /// Sorts by this field, ascending (`sort=+field`).
    pub fn asc(self) -> Sort<R> {
        Sort {
            term: format!("+{}", self.name),
            resource: PhantomData,
        }
    }

    /// Sorts by this field, descending (`sort=-field`).
    pub fn desc(self) -> Sort<R> {
        Sort {
            term: format!("-{}", self.name),
            resource: PhantomData,
        }
    }

    fn filter(self, operator: &str, value: &dyn FilterValue) -> Filter<R> {
        Filter {
            key: format!("{}{operator}", self.name),
            value: value.filter_value(),
            resource: PhantomData,
        }
    }
}

/// A filter on a list of resource `R`, e.g. `country=gb`.
pub struct Filter<R> {
    pub(crate) key: String,
    pub(crate) value: String,
    resource: PhantomData<fn() -> R>,
}

impl<R> fmt::Debug for Filter<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Filter({}={})", self.key, self.value)
    }
}

/// A sort term for a list of resource `R`, e.g. `+name`.
pub struct Sort<R> {
    pub(crate) term: String,
    resource: PhantomData<fn() -> R>,
}

impl<R> fmt::Debug for Sort<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sort({})", self.term)
    }
}

/// A value that can be compared in a filter.
pub trait FilterValue {
    /// The value as Rentman expects it in the query string.
    #[doc(hidden)]
    fn filter_value(&self) -> String;
}

/// Generated enums are filter values through their documented text.
#[doc(hidden)]
pub trait EnumFilterValue: Serialize {}

impl<T: EnumFilterValue> FilterValue for T {
    fn filter_value(&self) -> String {
        // Generated enums serialize to their documented (or unknown) text.
        serde_json::to_value(self)
            .ok()
            .as_ref()
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string()
    }
}

/// Rentman only understands `1` and `0` as booleans in filters; it ignores
/// `true` and `false`.
impl FilterValue for bool {
    fn filter_value(&self) -> String {
        if *self { "1" } else { "0" }.to_string()
    }
}

impl FilterValue for i64 {
    fn filter_value(&self) -> String {
        self.to_string()
    }
}

/// Decimal fields filter by their exact decimal text.
impl FilterValue for Decimal {
    fn filter_value(&self) -> String {
        self.to_string()
    }
}

impl FilterValue for String {
    fn filter_value(&self) -> String {
        self.clone()
    }
}

impl FilterValue for DateTime<FixedOffset> {
    fn filter_value(&self) -> String {
        self.to_rfc3339()
    }
}

impl FilterValue for EmailAddress {
    fn filter_value(&self) -> String {
        self.as_str().to_string()
    }
}

/// IDs and links filter by the numeric ID; Rentman accepts it for link
/// fields as well.
impl<R> FilterValue for Id<R> {
    fn filter_value(&self) -> String {
        self.get().to_string()
    }
}

impl FilterValue for ItemId {
    fn filter_value(&self) -> String {
        self.get().to_string()
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::fields::{accessory, contact, contact_person, file};
    use crate::model::{ContactCountry, CrewId, EquipmentPayload};
    use crate::{ItemId, model::EmailAddress};
    use chrono::DateTime;
    use rust_decimal::Decimal;

    fn pair<R>(filter: super::Filter<R>) -> (String, String) {
        (filter.key, filter.value)
    }

    fn owned(key: &str, value: &str) -> (String, String) {
        (key.to_string(), value.to_string())
    }

    #[test]
    fn renders_every_operator_in_the_documented_syntax() {
        let distance = Decimal::new(300, 0);
        assert_eq!(
            pair(contact::DISTANCE.eq(distance)),
            owned("distance", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.neq(distance)),
            owned("distance[neq]", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.lt(distance)),
            owned("distance[lt]", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.gt(distance)),
            owned("distance[gt]", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.lte(distance)),
            owned("distance[lte]", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.gte(distance)),
            owned("distance[gte]", "300")
        );
        assert_eq!(
            pair(contact::FOLDER.is_null(false)),
            owned("folder[isnull]", "0")
        );
        assert_eq!(
            pair(contact::FOLDER.is_null(true)),
            owned("folder[isnull]", "1")
        );
        assert_eq!(contact::NAME.asc().term, "+name");
        assert_eq!(contact::NAME.desc().term, "-name");
        assert_eq!(contact::NAME.name(), "name");
    }

    #[test]
    fn serializes_decimal_model_fields_without_losing_digits() {
        let decimal: Decimal = serde_json::from_str("0.1234567890123456789").unwrap();
        let payload = EquipmentPayload {
            list_price: Some(decimal),
            ..EquipmentPayload::default()
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert_eq!(json, r#"{"list_price":0.1234567890123456789}"#);

        let parsed: EquipmentPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.list_price, Some(decimal));
    }

    #[test]
    fn decimal_model_fields_round_excess_places_and_reject_overflow() {
        // Pins the documented precision limits of `Decimal`'s Serde support.
        let rounded: EquipmentPayload =
            serde_json::from_str(r#"{"list_price":0.12345678901234567890123456789012345}"#)
                .unwrap();
        assert_eq!(
            rounded.list_price.unwrap().to_string(),
            "0.1234567890123456789012345679"
        );
        for text in [
            r#"{"list_price":79228162514264337593543950336}"#,
            r#"{"list_price":1e30}"#,
        ] {
            assert!(
                serde_json::from_str::<EquipmentPayload>(text).is_err(),
                "{text}"
            );
        }
    }

    #[test]
    fn renders_values_as_rentman_expects_them() {
        assert_eq!(
            pair(contact::COUNTRY.eq(ContactCountry::Gb)),
            owned("country", "gb")
        );
        assert_eq!(
            pair(contact::COUNTRY.eq(ContactCountry::Unknown("xx".into()))),
            owned("country", "xx")
        );
        assert_eq!(pair(accessory::AUTOMATIC.eq(true)), owned("automatic", "1"));
        assert_eq!(
            pair(accessory::AUTOMATIC.eq(false)),
            owned("automatic", "0")
        );
        assert_eq!(
            pair(accessory::QUANTITY.gte(-2)),
            owned("quantity[gte]", "-2")
        );
        assert_eq!(
            pair(contact::DISTANCE.lte(Decimal::new(12505, 2))),
            owned("distance[lte]", "125.05")
        );
        assert_eq!(
            pair(contact::NAME.eq("Acme & Co".into())),
            owned("name", "Acme & Co")
        );
        let moment = DateTime::parse_from_rfc3339("2026-10-01T00:00:00+02:00").unwrap();
        assert_eq!(
            pair(accessory::CREATED.gt(moment)),
            owned("created[gt]", "2026-10-01T00:00:00+02:00")
        );
        let email: EmailAddress = serde_json::from_str(r#""a@example.test""#).unwrap();
        assert_eq!(
            pair(contact_person::EMAIL.eq(email)),
            owned("email", "a@example.test")
        );
        assert_eq!(pair(file::ITEM.eq(ItemId::new(4))), owned("item", "4"));
        assert_eq!(
            pair(accessory::CREATOR.eq(CrewId::new(247))),
            owned("creator", "247")
        );
    }

    #[test]
    fn describes_fields_filters_and_sorting() {
        let field = contact::NAME;
        let copy = field;
        assert_eq!(format!("{:?}", field.clone()), "Field(name)");
        assert_eq!(format!("{copy:?}"), "Field(name)");
        assert_eq!(
            format!("{:?}", contact::DISTANCE.lte(Decimal::new(300, 0))),
            "Filter(distance[lte]=300)"
        );
        assert_eq!(format!("{:?}", contact::NAME.desc()), "Sort(-name)");
    }
}
