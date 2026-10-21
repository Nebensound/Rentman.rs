//! Projections: own structs that select exactly the fields they declare.

use crate::id::{Id, Resource};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::value::RawValue;
use std::collections::HashMap;

/// A struct declared with [`projection!`](crate::projection), whose fields
/// are the `fields=` selection of its resource.
pub trait Projection: Sized {
    /// Resource the projection selects from, e.g. `model::Crew`.
    type Resource;
    /// Rentman names of the selected fields.
    #[doc(hidden)]
    const FIELDS: &'static [&'static str];
    /// Builds the projection from one item of the response.
    #[doc(hidden)]
    fn decode(item: &mut ProjectedItem) -> Result<Self, serde_json::Error>;
}

/// A field of a resource that a projection can select; generated per field in
/// [`projection_fields`](crate::projection_fields).
pub trait ProjectionField {
    /// Resource the field belongs to.
    type Resource;
    /// Rust type of the field, as in the generated model.
    type Value;
    /// Wire form of the value, e.g. a link `/crew/12` for an `Id<Crew>`.
    #[doc(hidden)]
    type Wire: DeserializeOwned + IntoValue<Self::Value>;
    /// Rentman name of the field, e.g. `type`.
    const NAME: &'static str;
}

/// Turns the wire form of a field into its value.
#[doc(hidden)]
pub trait IntoValue<V> {
    fn into_value(self) -> V;
}

impl<T> IntoValue<T> for T {
    fn into_value(self) -> T {
        self
    }
}

/// A link field on the wire, e.g. `"/crew/12"`.
#[doc(hidden)]
pub struct Link<R>(Id<R>);

impl<'de, R: Resource> Deserialize<'de> for Link<R> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        crate::id::link::deserialize(deserializer).map(Self)
    }
}

impl<R> IntoValue<Id<R>> for Link<R> {
    fn into_value(self) -> Id<R> {
        self.0
    }
}

impl<R> IntoValue<Option<Id<R>>> for Option<Link<R>> {
    fn into_value(self) -> Option<Id<R>> {
        self.map(|link| link.0)
    }
}

/// One item of a projected response, from which the projection takes its
/// fields. Each field keeps its raw JSON text, so amounts are read exactly.
#[doc(hidden)]
pub struct ProjectedItem(HashMap<String, Box<RawValue>>);

impl ProjectedItem {
    /// Takes the field `F` out of the item; a missing field reads as `null`,
    /// so only `Option` fields may be missing.
    pub fn take<F: ProjectionField>(&mut self) -> Result<F::Value, serde_json::Error> {
        self.take_named::<F::Wire>(F::NAME)
            .map(IntoValue::into_value)
    }

    /// Takes the field `name` out of the item as its wire form `T`.
    pub(crate) fn take_named<T: DeserializeOwned>(
        &mut self,
        name: &str,
    ) -> Result<T, serde_json::Error> {
        let raw = self.0.remove(name);
        serde_json::from_str::<T>(raw.as_deref().map_or("null", RawValue::get))
            .map_err(|error| serde::de::Error::custom(format!("field `{name}`: {error}")))
    }
}

impl<'de> Deserialize<'de> for ProjectedItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        HashMap::deserialize(deserializer).map(Self)
    }
}

/// Decodes an item of a projected response into the projection.
pub(crate) fn decode<P: Projection>(mut item: ProjectedItem) -> crate::Result<P> {
    P::decode(&mut item).map_err(|error| crate::Error::new(crate::ErrorKind::Decode(error)))
}

/// Fields Rentman adds to every selection by itself but does not accept in
/// `fields=` (`RESOURCE_EXTRA_FIELDS` in the generator): next to other fields
/// it drops them silently, and named alone it ignores the whole selection and
/// returns full items (verified live on 2026-10-21).
const ALWAYS_SENT: &[&str] = &["updateHash"];

/// The `fields=` value of a projection, e.g. `id,firstname`.
///
/// Fields Rentman always sends are not requested; a projection of only such
/// fields selects `id`, which Rentman accepts and adds them to.
pub(crate) fn fields<P: Projection>() -> String {
    let selected = P::FIELDS
        .iter()
        .copied()
        .filter(|field| !ALWAYS_SENT.contains(field))
        .collect::<Vec<_>>();
    if selected.is_empty() {
        return "id".to_string();
    }
    selected.join(",")
}

/// Declares a struct whose fields are a selection of a resource's fields.
///
/// The field types come from the generated model, and the struct is the
/// `fields=` list Rentman is asked for, so the two cannot drift apart. An
/// unknown field name does not compile. Use the Rentman field name; Rust
/// keywords are written as raw identifiers (`r#type` selects `type`).
/// Account-specific custom fields are selected as a whole through `custom`.
///
/// ```
/// rentman_client::projection! {
///     /// Name and email of a crew member.
///     pub struct CrewName from Crew { id, firstname, lastname, email }
/// }
///
/// # async fn example(client: rentman_client::RentmanClient) -> rentman_client::Result<()> {
/// let names: Vec<CrewName> = client.crew().list().all_as::<CrewName>().await?;
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// rentman_client::projection! {
///     pub struct Broken from Crew { id, not_a_field }
/// }
/// ```
#[macro_export]
macro_rules! projection {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident from $resource:ident { $($field:ident),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        #[allow(non_snake_case)]
        $vis struct $name {
            $(
                #[allow(missing_docs)]
                pub $field: <$crate::projection_fields::$resource::$field as $crate::ProjectionField>::Value,
            )+
        }

        impl $crate::Projection for $name {
            type Resource = $crate::model::$resource;
            const FIELDS: &'static [&'static str] = &[
                $(<$crate::projection_fields::$resource::$field as $crate::ProjectionField>::NAME),+
            ];

            fn decode(
                item: &mut $crate::__private::ProjectedItem,
            ) -> ::std::result::Result<Self, $crate::__private::serde_json::Error> {
                ::std::result::Result::Ok(Self {
                    $($field: item.take::<$crate::projection_fields::$resource::$field>()?,)+
                })
            }
        }
    };
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::model::{CrewId, TaskStatusId, TaskStatusType};
    use crate::{ErrorKind, Projection};
    use serde_json::json;

    crate::projection! {
        /// Some fields of a task status.
        struct StatusName from TaskStatus { id, name, r#type, creator }
    }

    fn decode_json(value: serde_json::Value) -> crate::Result<StatusName> {
        decode(serde_json::from_str::<ProjectedItem>(&value.to_string()).unwrap())
    }

    #[test]
    fn selects_exactly_the_declared_fields() {
        assert_eq!(StatusName::FIELDS, ["id", "name", "type", "creator"]);
        assert_eq!(fields::<StatusName>(), "id,name,type,creator");
    }

    #[test]
    fn decodes_items_with_links_and_ignores_extra_fields() {
        let status = decode_json(json!({
            "id": 5, "name": "Done", "type": "done", "creator": "/crew/3",
            "created": "2026-01-01T00:00:00+00:00", "updateHash": "x", "custom": {}
        }))
        .unwrap();
        assert_eq!(
            status,
            StatusName {
                id: TaskStatusId::new(5),
                name: "Done".to_string(),
                r#type: TaskStatusType::Done,
                creator: Some(CrewId::new(3)),
            }
        );

        // A missing optional field reads as `None`.
        let status = decode_json(json!({"id": 5, "name": "Done", "type": "done"})).unwrap();
        assert_eq!(status.creator, None);
        let status =
            decode_json(json!({"id": 5, "name": "Done", "type": "done", "creator": null})).unwrap();
        assert_eq!(status.creator, None);
    }

    #[test]
    fn names_the_field_that_does_not_decode() {
        for (item, message) in [
            (
                json!({"id": 5, "type": "done"}),
                "field `name`: invalid type: null",
            ),
            (
                json!({"id": 5, "name": "Done", "type": "done", "creator": "/projects/3"}),
                "field `creator`: link to /projects where a link to /crew was expected",
            ),
        ] {
            let error = decode_json(item).unwrap_err();
            let ErrorKind::Decode(cause) = error.kind() else {
                panic!("expected a decode error, got {error:?}");
            };
            assert!(cause.to_string().starts_with(message), "{cause}");
        }
        assert!(serde_json::from_str::<ProjectedItem>("[1]").is_err());
    }

    crate::projection! {
        struct StatusRevision from TaskStatus { id, updateHash }
    }

    crate::projection! {
        struct OnlyRevision from TaskStatus { updateHash }
    }

    #[test]
    fn reads_the_revision_without_requesting_it() {
        assert_eq!(StatusRevision::FIELDS, ["id", "updateHash"]);
        assert_eq!(fields::<StatusRevision>(), "id");
        assert_eq!(OnlyRevision::FIELDS, ["updateHash"]);
        assert_eq!(fields::<OnlyRevision>(), "id");

        let item = serde_json::from_str::<ProjectedItem>(
            r#"{"id": 5, "modified": null, "updateHash": "d21b3fd9"}"#,
        )
        .unwrap();
        let status: StatusRevision = decode(item).unwrap();
        assert_eq!(status.id, TaskStatusId::new(5));
        assert_eq!(status.updateHash.as_str(), "d21b3fd9");

        for wire in [r#"{"id": 5}"#, r#"{"updateHash": ""}"#] {
            let item = serde_json::from_str::<ProjectedItem>(wire).unwrap();
            let error = decode::<OnlyRevision>(item).unwrap_err();
            let ErrorKind::Decode(cause) = error.kind() else {
                panic!("expected a decode error, got {error:?}");
            };
            assert!(
                cause.to_string().starts_with("field `updateHash`"),
                "{cause}"
            );
        }
    }

    crate::projection! {
        struct ContactDistance from Contact { id, distance }
    }

    #[test]
    fn reads_amounts_exactly() {
        let mut item: ProjectedItem =
            serde_json::from_str(r#"{"id":1,"distance":123.45678901234567890123}"#).unwrap();
        let contact = ContactDistance::decode(&mut item).unwrap();
        assert_eq!(
            contact.distance.as_decimal().to_string(),
            "123.45678901234567890123"
        );
    }

    #[test]
    fn converts_wire_links_into_ids() {
        let link: Link<crate::model::Crew> = serde_json::from_value(json!("/crew/4")).unwrap();
        assert_eq!(IntoValue::<CrewId>::into_value(link), CrewId::new(4));
        let missing: Option<Link<crate::model::Crew>> = None;
        assert_eq!(IntoValue::<Option<CrewId>>::into_value(missing), None);
        assert_eq!(IntoValue::<i64>::into_value(7_i64), 7);
    }
}
