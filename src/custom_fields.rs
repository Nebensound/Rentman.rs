//! Runtime definitions for account-specific Rentman custom fields.

use crate::{
    error::{Error, ErrorKind, Result},
    id::Resource,
    model::{CustomFieldValue, CustomFields, ExtraInputFieldType},
};
use rust_decimal::Decimal;
use std::collections::HashSet;

#[doc(hidden)]
pub trait CustomFieldsResource: Resource {
    #[doc(hidden)]
    fn custom_fields_mut(&mut self) -> Option<&mut CustomFields>;
}

/// Definition of one account-specific custom field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFieldDefinition {
    collection: &'static str,
    id: u64,
    field_type: ExtraInputFieldType,
}

impl CustomFieldDefinition {
    /// Declares a custom field ID and type for resource `R`.
    ///
    /// Obtain `id`, `field_type`, and the resource association from Rentman's
    /// `extra_input_fields()` collection.
    pub fn new<R: Resource>(id: u64, field_type: ExtraInputFieldType) -> Self {
        Self {
            collection: R::COLLECTION,
            id,
            field_type,
        }
    }

    /// Numeric ID from the `ExtraInputField` resource.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Resource collection this custom field belongs to.
    pub fn collection(&self) -> &'static str {
        self.collection
    }

    /// Rentman custom-field value type.
    pub fn field_type(&self) -> &ExtraInputFieldType {
        &self.field_type
    }

    fn key(&self) -> String {
        format!("custom_{}", self.id)
    }
}

/// Invalid custom-field registry configuration or response value.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CustomFieldError {
    /// A resource and custom-field ID were registered more than once.
    #[error("custom field {collection}/custom_{id} is registered more than once")]
    DuplicateDefinition {
        /// Collection owning the custom field.
        collection: &'static str,
        /// Numeric custom-field ID.
        id: u64,
    },
    /// A registered custom field was not present in the returned `custom` map.
    #[error("response for /{collection} is missing registered custom field custom_{id}")]
    MissingValue {
        /// Collection owning the custom field.
        collection: &'static str,
        /// Numeric custom-field ID.
        id: u64,
    },
    /// A custom-field value did not match the type declared by Rentman.
    #[error("custom field /{collection}/custom_{id} has JSON type {actual}, expected {expected}")]
    TypeMismatch {
        /// Collection owning the custom field.
        collection: &'static str,
        /// Numeric custom-field ID.
        id: u64,
        /// Expected Rentman custom-field type.
        expected: &'static str,
        /// JSON value category received.
        actual: &'static str,
    },
}

/// Failed to read or write a dynamically typed custom-field value.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CustomFieldValueError {
    /// The requested `custom_<id>` key is absent.
    #[error("custom field custom_{id} is missing")]
    Missing {
        /// Numeric custom-field ID.
        id: u64,
    },
    /// A custom value could not be serialized or deserialized as the requested Rust type.
    #[error("custom-field value conversion failed: {0}")]
    Json(#[from] serde_json::Error),
}

/// Account-specific custom-field definitions supplied when configuring a client.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CustomFieldRegistry {
    definitions: Vec<CustomFieldDefinition>,
}

impl CustomFieldRegistry {
    /// Builds a registry and rejects duplicate `(resource, field ID)` pairs.
    pub fn new(
        definitions: impl IntoIterator<Item = CustomFieldDefinition>,
    ) -> std::result::Result<Self, CustomFieldError> {
        let mut seen = HashSet::new();
        let mut fields = Vec::new();
        for definition in definitions {
            if !seen.insert((definition.collection, definition.id)) {
                return Err(CustomFieldError::DuplicateDefinition {
                    collection: definition.collection,
                    id: definition.id,
                });
            }
            fields.push(definition);
        }
        Ok(Self {
            definitions: fields,
        })
    }

    pub(crate) fn has_fields(&self, collection: &str) -> bool {
        self.definitions
            .iter()
            .any(|definition| definition.collection == collection)
    }

    pub(crate) fn contains_all(&self, collection: &str, fields: Option<&CustomFields>) -> bool {
        self.definitions
            .iter()
            .filter(|definition| definition.collection == collection)
            .all(|definition| fields.is_some_and(|fields| fields.0.contains_key(&definition.key())))
    }

    pub(crate) fn apply(
        &self,
        collection: &'static str,
        fields: Option<&mut CustomFields>,
    ) -> Result<()> {
        let definitions = self
            .definitions
            .iter()
            .filter(|definition| definition.collection == collection)
            .collect::<Vec<_>>();
        let Some(fields) = fields else {
            if let Some(definition) = definitions.first() {
                return Err(custom_field_error(CustomFieldError::MissingValue {
                    collection,
                    id: definition.id,
                }));
            }
            return Ok(());
        };
        for definition in &definitions {
            let Some(value) = fields.0.get(&definition.key()) else {
                return Err(custom_field_error(CustomFieldError::MissingValue {
                    collection,
                    id: definition.id,
                }));
            };
            let (expected, matches) = expected_value(&definition.field_type, value);
            if !matches {
                return Err(custom_field_error(CustomFieldError::TypeMismatch {
                    collection,
                    id: definition.id,
                    expected,
                    actual: json_type(value),
                }));
            }
        }
        let allowed = definitions
            .iter()
            .map(|definition| definition.key())
            .collect::<HashSet<_>>();
        fields.0.retain(|key, _| allowed.contains(key));
        Ok(())
    }
}

fn custom_field_error(error: CustomFieldError) -> Error {
    Error::new(ErrorKind::CustomField(error))
}

fn json_type(value: &CustomFieldValue) -> &'static str {
    match value {
        CustomFieldValue::Null => "null",
        CustomFieldValue::Bool(_) => "boolean",
        CustomFieldValue::Number(_) => "number",
        CustomFieldValue::Text(_) => "string",
        CustomFieldValue::List(_) => "array",
        CustomFieldValue::Object(_) => "object",
    }
}

fn expected_value(
    field_type: &ExtraInputFieldType,
    value: &CustomFieldValue,
) -> (&'static str, bool) {
    if matches!(value, CustomFieldValue::Null) {
        return (field_type_name(field_type), true);
    }
    let matches = match field_type {
        ExtraInputFieldType::Bool => matches!(value, CustomFieldValue::Bool(_)),
        ExtraInputFieldType::Double | ExtraInputFieldType::Price => {
            matches!(value, CustomFieldValue::Number(_))
        }
        ExtraInputFieldType::Int => {
            matches!(value, CustomFieldValue::Number(number) if number.fract() == Decimal::ZERO)
        }
        ExtraInputFieldType::Color
        | ExtraInputFieldType::Date
        | ExtraInputFieldType::Datetime
        | ExtraInputFieldType::Enum
        | ExtraInputFieldType::Longtext
        | ExtraInputFieldType::Text
        | ExtraInputFieldType::Html
        | ExtraInputFieldType::Url
        | ExtraInputFieldType::Phone => matches!(value, CustomFieldValue::Text(_)),
        // Rentman does not document a stable JSON shape for linked custom values.
        ExtraInputFieldType::Item | ExtraInputFieldType::Unknown(_) => true,
    };
    (field_type_name(field_type), matches)
}

fn field_type_name(field_type: &ExtraInputFieldType) -> &'static str {
    match field_type {
        ExtraInputFieldType::Bool => "boolean",
        ExtraInputFieldType::Color => "color string",
        ExtraInputFieldType::Date => "date string",
        ExtraInputFieldType::Datetime => "datetime string",
        ExtraInputFieldType::Double => "number",
        ExtraInputFieldType::Enum => "enum string",
        ExtraInputFieldType::Int => "integer number",
        ExtraInputFieldType::Longtext => "string",
        ExtraInputFieldType::Price => "number",
        ExtraInputFieldType::Text => "string",
        ExtraInputFieldType::Html => "string",
        ExtraInputFieldType::Item => "linked item value",
        ExtraInputFieldType::Url => "URL string",
        ExtraInputFieldType::Phone => "phone string",
        ExtraInputFieldType::Unknown(_) => "any JSON value",
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::model::Equipment;
    use proptest::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[test]
    fn constructs_and_describes_a_typed_field() {
        let definition = CustomFieldDefinition::new::<Equipment>(8, ExtraInputFieldType::Text);
        assert_eq!(definition.id(), 8);
        assert_eq!(definition.collection(), "equipment");
        assert_eq!(definition.field_type(), &ExtraInputFieldType::Text);
        assert_eq!(definition.key(), "custom_8");
        assert_eq!(
            CustomFieldRegistry::default(),
            CustomFieldRegistry::new([]).unwrap()
        );
    }

    #[test]
    fn rejects_duplicate_definitions_for_one_resource() {
        let definition = CustomFieldDefinition::new::<Equipment>(8, ExtraInputFieldType::Text);
        let other = CustomFieldDefinition::new::<Equipment>(9, ExtraInputFieldType::Int);
        let registry = CustomFieldRegistry::new([definition.clone(), other]).unwrap();
        assert!(registry.has_fields("equipment"));
        assert_eq!(
            CustomFieldRegistry::new([definition.clone(), definition]),
            Err(CustomFieldError::DuplicateDefinition {
                collection: "equipment",
                id: 8,
            })
        );
    }

    #[test]
    fn validates_registered_field_presence_and_values() {
        let registry = CustomFieldRegistry::new([
            CustomFieldDefinition::new::<Equipment>(8, ExtraInputFieldType::Text),
            CustomFieldDefinition::new::<Equipment>(9, ExtraInputFieldType::Int),
            CustomFieldDefinition::new::<Equipment>(17, ExtraInputFieldType::Bool),
            CustomFieldDefinition::new::<Equipment>(23, ExtraInputFieldType::Price),
            CustomFieldDefinition::new::<Equipment>(41, ExtraInputFieldType::Enum),
            CustomFieldDefinition::new::<Equipment>(49, ExtraInputFieldType::Item),
        ])
        .unwrap();
        let mut fields = CustomFields(BTreeMap::from([
            (
                "custom_8".to_string(),
                CustomFieldValue::Text("ok".to_string()),
            ),
            (
                "custom_9".to_string(),
                CustomFieldValue::Number(Decimal::from(10)),
            ),
            ("custom_17".to_string(), CustomFieldValue::Bool(false)),
            (
                "custom_23".to_string(),
                CustomFieldValue::Number(Decimal::new(1234, 2)),
            ),
            (
                "custom_41".to_string(),
                CustomFieldValue::Text("choice".to_string()),
            ),
            ("custom_49".to_string(), CustomFieldValue::Null),
            (
                "custom_999".to_string(),
                CustomFieldValue::Text("ignored".to_string()),
            ),
        ]));
        assert!(registry.contains_all("equipment", Some(&fields)));
        assert!(!registry.contains_all("equipment", None));
        assert!(!registry.contains_all("equipment", Some(&CustomFields::default())));
        assert!(registry.contains_all("contacts", None));
        registry.apply("equipment", Some(&mut fields)).unwrap();
        assert!(!fields.0.contains_key("custom_999"));
        registry.apply("contacts", None).unwrap();
    }

    #[test]
    fn rejects_missing_fields_and_wrong_json_types() {
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        assert!(matches!(
            registry.apply("equipment", None).unwrap_err().kind(),
            ErrorKind::CustomField(CustomFieldError::MissingValue { id: 8, .. })
        ));
        let mut wrong = CustomFields(BTreeMap::from([(
            "custom_8".to_string(),
            CustomFieldValue::Bool(true),
        )]));
        assert!(matches!(
            registry
                .apply("equipment", Some(&mut wrong))
                .unwrap_err()
                .kind(),
            ErrorKind::CustomField(CustomFieldError::TypeMismatch {
                expected: "string",
                actual: "boolean",
                ..
            })
        ));
    }

    #[test]
    fn validates_all_api_field_types() {
        let examples = [
            (
                ExtraInputFieldType::Color,
                CustomFieldValue::Text("red".into()),
            ),
            (
                ExtraInputFieldType::Date,
                CustomFieldValue::Text("2026-10-21".into()),
            ),
            (
                ExtraInputFieldType::Datetime,
                CustomFieldValue::Text("2026-10-21T00:00:00Z".into()),
            ),
            (
                ExtraInputFieldType::Double,
                CustomFieldValue::Number(Decimal::ONE),
            ),
            (
                ExtraInputFieldType::Enum,
                CustomFieldValue::Text("choice".into()),
            ),
            (
                ExtraInputFieldType::Int,
                CustomFieldValue::Number(Decimal::from(1)),
            ),
            (
                ExtraInputFieldType::Longtext,
                CustomFieldValue::Text("text".into()),
            ),
            (
                ExtraInputFieldType::Price,
                CustomFieldValue::Number(Decimal::ONE),
            ),
            (
                ExtraInputFieldType::Text,
                CustomFieldValue::Text("text".into()),
            ),
            (
                ExtraInputFieldType::Html,
                CustomFieldValue::Text("<p>x</p>".into()),
            ),
            (
                ExtraInputFieldType::Item,
                CustomFieldValue::Object(BTreeMap::new()),
            ),
            (
                ExtraInputFieldType::Url,
                CustomFieldValue::Text("https://example.test".into()),
            ),
            (
                ExtraInputFieldType::Phone,
                CustomFieldValue::Text("+1".into()),
            ),
            (
                ExtraInputFieldType::Unknown("new_type".into()),
                CustomFieldValue::List(vec![]),
            ),
        ];
        for (field_type, value) in examples {
            let (_, valid) = expected_value(&field_type, &value);
            assert!(valid, "{field_type:?} rejected {value:?}");
            assert!(!field_type_name(&field_type).is_empty());
        }
        let (_, fractional_int_valid) = expected_value(
            &ExtraInputFieldType::Int,
            &CustomFieldValue::Number(Decimal::new(15, 1)),
        );
        assert!(!fractional_int_valid);
        for (value, expected) in [
            (CustomFieldValue::Null, "null"),
            (CustomFieldValue::Bool(true), "boolean"),
            (CustomFieldValue::Number(Decimal::ONE), "number"),
            (CustomFieldValue::Text("x".into()), "string"),
            (CustomFieldValue::List(vec![]), "array"),
            (CustomFieldValue::Object(BTreeMap::new()), "object"),
        ] {
            assert_eq!(json_type(&value), expected);
        }
    }

    #[test]
    fn custom_values_round_trip_through_application_types() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        enum Choice {
            #[serde(rename = "one")]
            One,
            #[serde(rename = "two")]
            Two,
        }

        let mut fields = CustomFields::default();
        fields.set(41, &Choice::Two).unwrap();
        assert_eq!(fields.get::<Choice>(41).unwrap(), Choice::Two);
        assert!(matches!(
            fields.get::<Choice>(99),
            Err(CustomFieldValueError::Missing { id: 99 })
        ));
        assert!(matches!(
            fields.get::<bool>(41),
            Err(CustomFieldValueError::Json(_))
        ));
    }

    #[test]
    fn custom_value_deserialization_preserves_decimal_json_numbers() {
        let value: CustomFieldValue = serde_json::from_str("0.1234567890123456789").unwrap();
        assert_eq!(
            value,
            CustomFieldValue::Number("0.1234567890123456789".parse().unwrap())
        );

        let nested: CustomFieldValue =
            serde_json::from_str(r#"{"values":[1.25,"text",true,null]}"#).unwrap();
        assert_eq!(
            nested,
            CustomFieldValue::Object(BTreeMap::from([(
                "values".to_owned(),
                CustomFieldValue::List(vec![
                    CustomFieldValue::Number(Decimal::new(125, 2)),
                    CustomFieldValue::Text("text".to_owned()),
                    CustomFieldValue::Bool(true),
                    CustomFieldValue::Null,
                ]),
            )]))
        );
        // Out-of-range numbers fail at every depth instead of surfacing as
        // serde_json's private arbitrary-precision marker object.
        for text in ["1e9999", "[1e9999]", r#"{"value":1e9999}"#, "{"] {
            assert!(
                serde_json::from_str::<CustomFieldValue>(text).is_err(),
                "{text}"
            );
        }
    }

    proptest! {
        #[test]
        fn custom_value_json_never_panics(text in any::<String>()) {
            if let Ok(value) = serde_json::from_str::<CustomFieldValue>(&text) {
                let encoded = serde_json::to_string(&value).unwrap();
                let decoded: CustomFieldValue = serde_json::from_str(&encoded).unwrap();
                prop_assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn custom_values_report_application_serialization_errors() {
        // One type for every outcome: llvm-cov counts `set` as covered only
        // if a single instantiation runs all of its branches.
        enum Probe {
            Text,
            FailsSerialization,
            OutOfDecimalRange,
        }

        impl Serialize for Probe {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    Self::Text => serializer.serialize_str("ok"),
                    Self::FailsSerialization => Err(serde::ser::Error::custom(
                        "deliberate serialization failure",
                    )),
                    Self::OutOfDecimalRange => serializer.serialize_u128(u128::MAX),
                }
            }
        }

        let mut fields = CustomFields::default();
        fields.set(41, &Probe::Text).unwrap();
        assert_eq!(fields.get::<String>(41).unwrap(), "ok");
        assert!(matches!(
            fields.set(42, &Probe::FailsSerialization),
            Err(CustomFieldValueError::Json(_))
        ));
        assert!(matches!(
            fields.set(43, &Probe::OutOfDecimalRange),
            Err(CustomFieldValueError::Json(_))
        ));
        assert!(!fields.0.contains_key("custom_42"));
        assert!(!fields.0.contains_key("custom_43"));

        let raw = serde_json::value::RawValue::from_string("1e9999".to_string()).unwrap();
        assert!(matches!(
            fields.set(44, &raw),
            Err(CustomFieldValueError::Json(_))
        ));

        fields.set(45, &true).unwrap();
        assert!(matches!(
            fields.get::<String>(45),
            Err(CustomFieldValueError::Json(_))
        ));
        assert!(matches!(
            fields.get::<String>(99),
            Err(CustomFieldValueError::Missing { id: 99 })
        ));
    }
}
