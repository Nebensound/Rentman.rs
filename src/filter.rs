//! Typed filters and sorting for resource lists.
//!
//! The generated module [`fields`](crate::fields) has one constant per
//! queryable field of every resource, e.g. `fields::contact::COUNTRY`. A
//! constant knows its resource and value type, so a filter for another
//! resource or with a value of the wrong type does not compile. The methods
//! are named like Rust's comparison methods ([`PartialOrd::le`] and so on);
//! the client translates them into Rentman's query syntax:
//!
//! | Method | Matches items whose field | Sent as |
//! | --- | --- | --- |
//! | [`eq(v)`](Field::eq) | equals `v` | `field=v` |
//! | [`ne(v)`](Field::ne) | does not equal `v` | `field[neq]=v` |
//! | [`lt(v)`](Field::lt) / [`le(v)`](Field::le) | is less than / at most `v` | `field[lt]=v` / `field[lte]=v` |
//! | [`gt(v)`](Field::gt) / [`ge(v)`](Field::ge) | is greater than / at least `v` | `field[gt]=v` / `field[gte]=v` |
//! | [`in_range(a..b)`](Field::in_range) | lies in a Rust range | the matching lower and upper bounds |
//! | [`is_none()`](Field::is_none) / [`is_some()`](Field::is_some) | is null / is not null | `field[isnull]=1` / `0` |
//!
//! Values keep their Rust type: booleans are sent as `1`/`0`, IDs and links
//! as the numeric ID, enums as their documented text, decimals exactly, and
//! timestamps in RFC 3339. Several [`filter`](crate::resource::ListRequest::filter)
//! calls must all match.
//!
//! ```
//! # use rentman_client::{RentmanClient, fields::contact, model};
//! # async fn example(client: RentmanClient) -> rentman_client::Result<()> {
//! let contacts: Vec<model::Contact> = client
//!     .contacts()
//!     .list()
//!     .filter(contact::COUNTRY.eq(model::ContactCountry::Gb))
//!     .filter(contact::FOLDER.is_some())
//!     .sort(contact::NAME.asc())
//!     .all()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Fields Rentman marks as `GENERATED FIELD` can neither be filtered nor
//! sorted on and have no constant. Filter on the field's value type, not on
//! `Option`: nullability is tested with `is_none`/`is_some`.
//!
//! A value of the wrong type does not compile:
//!
//! ```compile_fail
//! # use rentman_client::{RentmanClient, fields::contact};
//! # fn example(client: RentmanClient) {
//! let _ = client.contacts().list().filter(contact::DISTANCE.le("300".to_string()));
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
use std::{
    fmt,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

/// A queryable field of resource `R` with values of type `V`.
///
/// Take one from [`fields`](crate::fields), e.g. `fields::crew::LASTNAME`,
/// and turn it into a [`Filter`] or [`Sort`]:
///
/// ```
/// use rentman_client::fields::crew;
///
/// assert_eq!(crew::LASTNAME.name(), "lastname");
/// let active = crew::ACTIVE.eq(true);
/// let by_name = crew::LASTNAME.asc();
/// # let _ = (active, by_name);
/// ```
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
    ///
    /// ```
    /// assert_eq!(rentman_client::fields::contact::COUNTRY.name(), "country");
    /// ```
    pub fn name(self) -> &'static str {
        self.name
    }

    /// Items whose field equals `value`.
    ///
    /// ```
    /// use rentman_client::fields::crew;
    ///
    /// let filter = crew::ACTIVE.eq(true);
    /// assert_eq!(format!("{filter:?}"), "Filter(active=1)");
    /// ```
    pub fn eq(self, value: V) -> Filter<R> {
        self.filter("", &value)
    }

    /// Items whose field does not equal `value`.
    ///
    /// ```
    /// use rentman_client::fields::task_status;
    ///
    /// let filter = task_status::COLOR.ne("red".to_string());
    /// assert_eq!(format!("{filter:?}"), "Filter(color[neq]=red)");
    /// ```
    pub fn ne(self, value: V) -> Filter<R> {
        self.filter("[neq]", &value)
    }

    /// Items whose field is less than `value`.
    ///
    /// ```
    /// use rentman_client::fields::task_status;
    ///
    /// let filter = task_status::ORDER.lt(10);
    /// assert_eq!(format!("{filter:?}"), "Filter(order[lt]=10)");
    /// ```
    pub fn lt(self, value: V) -> Filter<R> {
        self.filter("[lt]", &value)
    }

    /// Items whose field is at most `value`.
    ///
    /// ```
    /// use rentman_client::{Decimal, fields::contact};
    ///
    /// let filter = contact::DISTANCE.le(Decimal::new(12505, 2));
    /// assert_eq!(format!("{filter:?}"), "Filter(distance[lte]=125.05)");
    /// ```
    pub fn le(self, value: V) -> Filter<R> {
        self.filter("[lte]", &value)
    }

    /// Items whose field is greater than `value`.
    ///
    /// ```
    /// use chrono::DateTime;
    /// use rentman_client::fields::crew;
    ///
    /// let since = DateTime::parse_from_rfc3339("2026-10-01T00:00:00+02:00").unwrap();
    /// let filter = crew::MODIFIED.gt(since);
    /// assert_eq!(format!("{filter:?}"), "Filter(modified[gt]=2026-10-01T00:00:00+02:00)");
    /// ```
    pub fn gt(self, value: V) -> Filter<R> {
        self.filter("[gt]", &value)
    }

    /// Items whose field is at least `value`.
    ///
    /// ```
    /// use rentman_client::fields::task_status;
    ///
    /// let filter = task_status::ORDER.ge(2);
    /// assert_eq!(format!("{filter:?}"), "Filter(order[gte]=2)");
    /// ```
    pub fn ge(self, value: V) -> Filter<R> {
        self.filter("[gte]", &value)
    }

    /// Items whose field lies in `range`, written as a Rust range.
    ///
    /// Each bound becomes one condition: an inclusive start becomes `ge`, an
    /// exclusive end `lt`, an inclusive end `le`. An unbounded side adds no
    /// condition, so `..` matches every item.
    ///
    /// ```
    /// use rentman_client::fields::task_status;
    ///
    /// let filter = task_status::ORDER.in_range(2..5);
    /// assert_eq!(format!("{filter:?}"), "Filter(order[gte]=2&order[lt]=5)");
    ///
    /// let filter = task_status::ORDER.in_range(..=9);
    /// assert_eq!(format!("{filter:?}"), "Filter(order[lte]=9)");
    /// ```
    pub fn in_range(self, range: impl RangeBounds<V>) -> Filter<R> {
        Filter::new(range_conditions(
            self.name,
            erase(range.start_bound()),
            erase(range.end_bound()),
        ))
    }

    /// Items whose field is null, the `None` of an `Option` field.
    ///
    /// ```
    /// use rentman_client::fields::contact;
    ///
    /// let filter = contact::FOLDER.is_none();
    /// assert_eq!(format!("{filter:?}"), "Filter(folder[isnull]=1)");
    /// ```
    pub fn is_none(self) -> Filter<R> {
        self.filter("[isnull]", &true)
    }

    /// Items whose field is not null, the `Some` of an `Option` field.
    ///
    /// ```
    /// use rentman_client::fields::contact;
    ///
    /// let filter = contact::FOLDER.is_some();
    /// assert_eq!(format!("{filter:?}"), "Filter(folder[isnull]=0)");
    /// ```
    pub fn is_some(self) -> Filter<R> {
        self.filter("[isnull]", &false)
    }

    /// Sorts by this field, ascending.
    ///
    /// ```
    /// use rentman_client::fields::crew;
    ///
    /// assert_eq!(format!("{:?}", crew::LASTNAME.asc()), "Sort(+lastname)");
    /// ```
    pub fn asc(self) -> Sort<R> {
        Sort {
            term: format!("+{}", self.name),
            resource: PhantomData,
        }
    }

    /// Sorts by this field, descending.
    ///
    /// ```
    /// use rentman_client::fields::crew;
    ///
    /// assert_eq!(format!("{:?}", crew::MODIFIED.desc()), "Sort(-modified)");
    /// ```
    pub fn desc(self) -> Sort<R> {
        Sort {
            term: format!("-{}", self.name),
            resource: PhantomData,
        }
    }

    fn filter(self, operator: &str, value: &dyn FilterValue) -> Filter<R> {
        Filter::new(vec![self.condition(operator, value)])
    }

    fn condition(self, operator: &str, value: &dyn FilterValue) -> (String, String) {
        condition(self.name, operator, value)
    }
}

fn condition(name: &str, operator: &str, value: &dyn FilterValue) -> (String, String) {
    (format!("{name}{operator}"), value.filter_value())
}

/// Erases the value type so the range translation below is not instantiated
/// once per range type.
fn erase<V: FilterValue>(bound: Bound<&V>) -> Bound<&dyn FilterValue> {
    match bound {
        Bound::Included(value) => Bound::Included(value),
        Bound::Excluded(value) => Bound::Excluded(value),
        Bound::Unbounded => Bound::Unbounded,
    }
}

/// Rentman conditions for a range: a lower bound becomes `[gte]` or `[gt]`,
/// an upper bound `[lte]` or `[lt]`, and an unbounded side nothing.
fn range_conditions(
    name: &str,
    start: Bound<&dyn FilterValue>,
    end: Bound<&dyn FilterValue>,
) -> Vec<(String, String)> {
    let lower = match start {
        Bound::Included(value) => Some(condition(name, "[gte]", value)),
        Bound::Excluded(value) => Some(condition(name, "[gt]", value)),
        Bound::Unbounded => None,
    };
    let upper = match end {
        Bound::Included(value) => Some(condition(name, "[lte]", value)),
        Bound::Excluded(value) => Some(condition(name, "[lt]", value)),
        Bound::Unbounded => None,
    };
    lower.into_iter().chain(upper).collect()
}

/// A filter on a list of resource `R`, built from a [`Field`].
///
/// Pass it to [`ListRequest::filter`](crate::resource::ListRequest::filter).
/// Its `Debug` output shows the query it sends:
///
/// ```
/// use rentman_client::fields::crew;
///
/// assert_eq!(format!("{:?}", crew::ACTIVE.eq(false)), "Filter(active=0)");
/// ```
pub struct Filter<R> {
    pub(crate) conditions: Vec<(String, String)>,
    resource: PhantomData<fn() -> R>,
}

impl<R> Filter<R> {
    fn new(conditions: Vec<(String, String)>) -> Self {
        Self {
            conditions,
            resource: PhantomData,
        }
    }
}

impl<R> fmt::Debug for Filter<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let query = self
            .conditions
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");
        write!(f, "Filter({query})")
    }
}

/// A sort term for a list of resource `R`, built with [`Field::asc`] or
/// [`Field::desc`] and passed to [`ListRequest::sort`](crate::resource::ListRequest::sort).
///
/// ```
/// use rentman_client::fields::crew;
///
/// assert_eq!(format!("{:?}", crew::LASTNAME.asc()), "Sort(+lastname)");
/// ```
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
///
/// Implemented for the value types of [`fields`](crate::fields) constants:
/// `bool`, `i64`, `String`, [`Decimal`], timestamps, [`EmailAddress`],
/// [`Id`], [`ItemId`], and generated enums. You do not implement it yourself.
///
/// ```
/// use rentman_client::{fields::contact, model::ContactCountry};
///
/// // A generated enum is sent as its documented text.
/// assert_eq!(format!("{:?}", contact::COUNTRY.eq(ContactCountry::Gb)), "Filter(country=gb)");
/// ```
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
        let [condition] = <[_; 1]>::try_from(filter.conditions).unwrap();
        condition
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
            pair(contact::DISTANCE.ne(distance)),
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
            pair(contact::DISTANCE.le(distance)),
            owned("distance[lte]", "300")
        );
        assert_eq!(
            pair(contact::DISTANCE.ge(distance)),
            owned("distance[gte]", "300")
        );
        assert_eq!(
            pair(contact::FOLDER.is_some()),
            owned("folder[isnull]", "0")
        );
        assert_eq!(
            pair(contact::FOLDER.is_none()),
            owned("folder[isnull]", "1")
        );
        assert_eq!(
            contact::DISTANCE.in_range(distance..=distance).conditions,
            [owned("distance[gte]", "300"), owned("distance[lte]", "300")]
        );
        assert_eq!(
            contact::DISTANCE
                .in_range((
                    std::ops::Bound::Excluded(distance),
                    std::ops::Bound::Excluded(distance)
                ))
                .conditions,
            [owned("distance[gt]", "300"), owned("distance[lt]", "300")]
        );
        assert!(contact::DISTANCE.in_range(..).conditions.is_empty());
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
            pair(accessory::QUANTITY.ge(-2)),
            owned("quantity[gte]", "-2")
        );
        assert_eq!(
            pair(contact::DISTANCE.le(Decimal::new(12505, 2))),
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
            format!("{:?}", contact::DISTANCE.le(Decimal::new(300, 0))),
            "Filter(distance[lte]=300)"
        );
        assert_eq!(format!("{:?}", contact::NAME.desc()), "Sort(-name)");
    }
}
