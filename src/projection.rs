//! Projections: own structs that select exactly the fields they declare.

use crate::id::{Id, Resource};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::value::RawValue;
use std::collections::HashMap;

const NEVER_REQUESTED_FIELDS: &[&str] = &["updateHash"];

/// A struct declared with [`projection!`](crate::projection), whose fields
/// are the `fields=` selection of its resource.
pub trait Projection: Sized {
    /// Resource the projection selects from, e.g. `model::Crew`.
    type Resource: Resource;
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
    /// Whether this key must be present in the selected response.
    const REQUIRED: bool = false;
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
    pub(crate) fn custom_fields_complete(
        &self,
        registry: &crate::CustomFieldRegistry,
        collection: &'static str,
    ) -> Result<bool, serde_json::Error> {
        if !registry.has_fields(collection) {
            return Ok(true);
        }
        let Some(raw) = self.0.get("custom") else {
            return Ok(false);
        };
        let fields = serde_json::from_str::<crate::model::CustomFields>(raw.get())?;
        Ok(registry.contains_all(collection, Some(&fields)))
    }

    pub(crate) fn apply_custom_fields(
        &mut self,
        registry: &crate::CustomFieldRegistry,
        collection: &'static str,
    ) -> crate::Result<()> {
        if !registry.has_fields(collection) {
            self.0.remove("custom");
            return Ok(());
        }
        let raw = self.0.remove("custom");
        let Some(raw) = raw else {
            return registry.apply(collection, None);
        };
        let mut fields = serde_json::from_str::<crate::model::CustomFields>(raw.get())
            .map_err(|error| crate::Error::new(crate::ErrorKind::Decode(error)))?;
        registry.apply(collection, Some(&mut fields))?;
        let raw = serde_json::value::to_raw_value(&fields)
            .expect("CustomFields contain only serializable JSON values");
        self.0.insert("custom".to_string(), raw);
        Ok(())
    }

    pub(crate) fn has_fields<P: Projection>(&self) -> bool {
        self.has_field_names(P::FIELDS)
    }

    pub(crate) fn has_field_names(&self, fields: &[&str]) -> bool {
        fields.iter().all(|field| self.0.contains_key(*field))
    }

    pub(crate) fn resource_id<R: Resource>(&self) -> crate::Result<Id<R>> {
        serde_json::from_str::<Id<R>>(self.0.get("id").map_or("null", |raw| raw.get()))
            .map_err(|error| crate::Error::new(crate::ErrorKind::Decode(error)))
    }

    pub(crate) fn deserialize<T: DeserializeOwned>(self) -> crate::Result<T> {
        let json =
            serde_json::to_string(&self.0).expect("ProjectedItem contains only parsed JSON values");
        serde_json::from_str(&json)
            .map_err(|error| crate::Error::new(crate::ErrorKind::Decode(error)))
    }

    /// Takes the field `F` out of the item; a missing field reads as `null`,
    /// so only `Option` fields may be missing.
    pub fn take<F: ProjectionField>(&mut self) -> Result<F::Value, serde_json::Error> {
        if F::REQUIRED && !self.0.contains_key(F::NAME) {
            return Err(serde::de::Error::missing_field(F::NAME));
        }
        self.take_named::<F::Wire>(F::NAME)
            .map(IntoValue::into_value)
    }

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

/// The `fields=` value of a projection, e.g. `id,firstname`.
pub(crate) fn fields<P: Projection>() -> String {
    fields_named(P::FIELDS)
}

pub(crate) fn fields_named(fields: &[&str]) -> String {
    let requested = fields
        .iter()
        .copied()
        .filter(|field| !NEVER_REQUESTED_FIELDS.contains(field))
        .collect::<Vec<_>>();
    if requested.is_empty() {
        "id".to_string()
    } else {
        requested.join(",")
    }
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
        struct StatusName from TaskStatus { id, updateHash, name, r#type, creator }
    }

    fn decode_json(value: serde_json::Value) -> crate::Result<StatusName> {
        decode(serde_json::from_str::<ProjectedItem>(&value.to_string()).unwrap())
    }

    #[test]
    fn selects_exactly_the_declared_fields() {
        assert_eq!(
            StatusName::FIELDS,
            ["id", "updateHash", "name", "type", "creator"]
        );
        assert_eq!(fields::<StatusName>(), "id,name,type,creator");
        assert_eq!(fields::<OnlyRevision>(), "id");
    }

    crate::projection! {
        struct OnlyRevision from TaskStatus { updateHash }
    }

    crate::projection! {
        struct StatusCore from TaskStatus { id, updateHash, created, modified, creator }
    }

    #[test]
    fn decodes_items_with_links_and_ignores_extra_fields() {
        let status = decode_json(json!({
            "id": 5, "updateHash": "status-5", "name": "Done", "type": "done", "creator": "/crew/3",
            "created": "2026-01-01T00:00:00+00:00", "custom": {}
        }))
        .unwrap();
        assert_eq!(
            status,
            StatusName {
                id: TaskStatusId::new(5),
                updateHash: crate::Revision::new("status-5").unwrap(),
                name: "Done".to_string(),
                r#type: TaskStatusType::Done,
                creator: Some(CrewId::new(3)),
            }
        );

        // Nullable fields must be present, but their value may be null.
        let status = decode_json(json!({
            "id": 5,
            "updateHash": "status-5",
            "name": "Done",
            "type": "done",
            "creator": null
        }))
        .unwrap();
        assert_eq!(status.creator, None);
    }

    #[test]
    fn names_the_field_that_does_not_decode() {
        for (item, message) in [
            (
                json!({"id": 5, "updateHash": "status-5", "type": "done"}),
                "missing field `name`",
            ),
            (
                json!({
                    "id": 5,
                    "updateHash": "status-5",
                    "name": "Done",
                    "type": "done",
                    "creator": "/projects/3"
                }),
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

    #[test]
    fn nullable_core_projection_fields_must_be_present_but_may_be_null() {
        let base = json!({
            "id": 5,
            "updateHash": "status-5",
            "created": null,
            "modified": null,
            "creator": null
        });
        let item: ProjectedItem = serde_json::from_value(base.clone()).unwrap();
        let status = decode::<StatusCore>(item).unwrap();
        assert_eq!(status.created, None);
        assert_eq!(status.modified, None);
        assert_eq!(status.creator, None);

        for field in ["id", "updateHash", "created", "modified", "creator"] {
            let mut wire = base.clone();
            wire.as_object_mut().unwrap().remove(field);
            let item: ProjectedItem = serde_json::from_value(wire).unwrap();
            let error = decode::<StatusCore>(item).unwrap_err();
            let ErrorKind::Decode(cause) = error.kind() else {
                panic!("expected a decode error, got {error:?}");
            };
            assert!(cause.to_string().contains(field), "{cause}");
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

    #[test]
    fn handles_custom_fields_with_and_without_a_registry() {
        use crate::model::ExtraInputFieldType;
        use crate::{CustomFieldDefinition, CustomFieldRegistry};

        let empty = CustomFieldRegistry::default();
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<
            crate::model::Equipment,
        >(8, ExtraInputFieldType::Text)])
        .unwrap();
        let item: ProjectedItem =
            serde_json::from_str(r#"{"id":1,"custom":{"custom_8":"x"}}"#).unwrap();
        assert!(item.custom_fields_complete(&empty, "equipment").unwrap());
        assert!(item.custom_fields_complete(&registry, "equipment").unwrap());

        let missing: ProjectedItem = serde_json::from_str(r#"{"id":1}"#).unwrap();
        assert!(
            !missing
                .custom_fields_complete(&registry, "equipment")
                .unwrap()
        );
        let malformed: ProjectedItem =
            serde_json::from_str(r#"{"id":1,"custom":"invalid"}"#).unwrap();
        assert!(
            malformed
                .custom_fields_complete(&registry, "equipment")
                .is_err()
        );

        let mut without_registry: ProjectedItem =
            serde_json::from_str(r#"{"custom":{"custom_8":"x"}}"#).unwrap();
        without_registry
            .apply_custom_fields(&empty, "equipment")
            .unwrap();
        assert!(!without_registry.0.contains_key("custom"));

        let mut with_unknown: ProjectedItem =
            serde_json::from_str(r#"{"custom":{"custom_8":"x","custom_999":true}}"#).unwrap();
        with_unknown
            .apply_custom_fields(&registry, "equipment")
            .unwrap();
        let kept: crate::model::CustomFields =
            serde_json::from_str(with_unknown.0["custom"].get()).unwrap();
        assert_eq!(kept.0.len(), 1);

        let mut missing: ProjectedItem = serde_json::from_str(r#"{"id":1}"#).unwrap();
        assert!(matches!(
            missing
                .apply_custom_fields(&registry, "equipment")
                .unwrap_err()
                .kind(),
            crate::ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        let mut malformed: ProjectedItem = serde_json::from_str(r#"{"custom":"invalid"}"#).unwrap();
        assert!(matches!(
            malformed
                .apply_custom_fields(&registry, "equipment")
                .unwrap_err()
                .kind(),
            crate::ErrorKind::Decode(_)
        ));
    }

    #[test]
    fn extracts_resource_ids_and_decodes_write_items() {
        let item: ProjectedItem = serde_json::from_str(r#"{"id":12,"updateHash":"h12"}"#).unwrap();
        assert_eq!(
            item.resource_id::<crate::model::Equipment>().unwrap(),
            crate::model::EquipmentId::new(12)
        );
        let item: ProjectedItem = serde_json::from_str(
            r#"{"id":12,"updateHash":"h12","created":null,"modified":null,"creator":null}"#,
        )
        .unwrap();
        let equipment: crate::model::Equipment = item.deserialize().unwrap();
        assert_eq!(equipment.id, crate::model::EquipmentId::new(12));
        assert_eq!(equipment.revision.as_str(), "h12");

        let missing: ProjectedItem = serde_json::from_str(r#"{"updateHash":"h12"}"#).unwrap();
        assert!(matches!(
            missing
                .resource_id::<crate::model::Equipment>()
                .unwrap_err()
                .kind(),
            crate::ErrorKind::Decode(_)
        ));
        let invalid: ProjectedItem =
            serde_json::from_str(r#"{"id":"bad","updateHash":"h12"}"#).unwrap();
        assert!(matches!(
            invalid
                .deserialize::<crate::model::Equipment>()
                .unwrap_err()
                .kind(),
            crate::ErrorKind::Decode(_)
        ));
    }
}
