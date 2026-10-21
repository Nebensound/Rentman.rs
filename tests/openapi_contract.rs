//! Contract tests against the checked-in Rentman OpenAPI document.

use chrono::NaiveDate;
use rentman_client::model::{
    Crew, CrewId, EmailAddress, Equipment, EquipmentIsPhysical, EquipmentRentalSales,
    EquipmentStockManagement, EquipmentStrictContainerContent, EquipmentType, Invoice, LINK_FIELDS,
    Payment, ProjectsCreateFromTemplatePayload, RESOURCE_COLLECTIONS, Rate, SCHEMA_TYPES, TaxClass,
    TaxClassType,
};
use rentman_client::model::{InvoiceId, PaymentId};
use rentman_client::{
    CustomFieldsResource, Id, LinkError, ProjectionField, Resource, Revision, Revisioned,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fmt::Debug, fs};

#[path = "contract/generated_payload_constructors.rs"]
mod generated_payload_constructors;
#[path = "contract/generated_response_enums.rs"]
mod generated_response_enums;
#[path = "contract/generated_revisions.rs"]
mod generated_revisions;

#[test]
fn wire_models_cover_every_documented_schema() {
    let documented = openapi_spec()["components"]["schemas"]
        .as_object()
        .expect("OpenAPI schemas are an object")
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let covered = SCHEMA_TYPES
        .iter()
        .map(|(schema, _)| schema.to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        documented, covered,
        "wire model manifest drifted from openapi/rentman-oas.json; run `cargo xtask generate-openapi` and review the schema surface"
    );
}

#[test]
fn every_schema_maps_to_exactly_one_resource_named_type() {
    let schemas = SCHEMA_TYPES
        .iter()
        .map(|(schema, _)| *schema)
        .collect::<BTreeSet<_>>();
    let types = SCHEMA_TYPES
        .iter()
        .map(|(_, rust_type)| *rust_type)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        schemas.len(),
        SCHEMA_TYPES.len(),
        "a schema is listed twice"
    );
    assert_eq!(
        types.len(),
        SCHEMA_TYPES.len(),
        "two schemas share a Rust type"
    );

    for (schema, rust_type) in SCHEMA_TYPES {
        if *schema == "ApiResponse" {
            // Paging metadata of the response envelope, not a resource.
            assert_eq!(*rust_type, "ApiResponse");
        } else if schema.ends_with("Request") {
            assert!(rust_type.ends_with("Payload"), "{schema} -> {rust_type}");
        } else {
            assert!(schema.ends_with("Response"), "{schema} is neither");
            assert!(
                !rust_type.ends_with("Response") && !rust_type.ends_with("Payload"),
                "{schema} -> {rust_type} is not a resource name"
            );
        }
    }

    let renamed = |schema: &str| {
        SCHEMA_TYPES
            .iter()
            .find(|(name, _)| *name == schema)
            .map(|(_, rust_type)| *rust_type)
    };
    assert_eq!(renamed("CrewResponse"), Some("Crew"));
    assert_eq!(renamed("CrewRequest"), Some("CrewPayload"));
    assert_eq!(renamed("FactuurResponse"), Some("Invoice"));
    assert_eq!(renamed("FactuurRequest"), Some("InvoicePayload"));
    assert_eq!(renamed("InvitationsResponse"), Some("Invitation"));
    assert_eq!(renamed("CrewRateResponse"), Some("Rate"));
    assert_eq!(renamed("CrewRatesResponse"), Some("CrewRate"));
    assert_eq!(renamed("CrewRateFactorResponse"), Some("RateFactor"));
    assert_eq!(renamed("QuotationResponse"), Some("Quote"));
    assert_eq!(renamed("LedgerResponse"), Some("LedgerCode"));
}

#[test]
fn every_documented_operation_is_reachable_through_a_resource() {
    let mut documented = openapi_spec()["paths"]
        .as_object()
        .expect("OpenAPI paths are an object")
        .values()
        .flat_map(|methods| methods.as_object().into_iter().flatten())
        .filter_map(|(_, operation)| operation["operationId"].as_str())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let mut reachable = rentman_client::resource::REACHABLE_OPERATIONS
        .iter()
        .map(|operation| operation.to_string())
        .collect::<Vec<_>>();
    documented.sort();
    reachable.sort();
    assert_eq!(
        documented, reachable,
        "resources drifted from openapi/rentman-oas.json; run `cargo xtask generate-openapi --only resources`"
    );
}

#[test]
fn field_constants_cover_every_queryable_field() {
    let spec = openapi_spec();
    let resources = RESOURCE_COLLECTIONS
        .iter()
        .map(|(rust_type, _)| *rust_type)
        .collect::<BTreeSet<_>>();
    let documented = SCHEMA_TYPES
        .iter()
        .filter(|(schema, rust_type)| schema.ends_with("Response") && resources.contains(rust_type))
        .flat_map(|(schema_name, rust_type)| {
            schema(&spec, schema_name)["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(_, property)| {
                    !property["description"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("GENERATED FIELD")
                        && property["type"] != "object"
                })
                .map(move |(field, _)| (rust_type.to_string(), field.clone()))
        })
        .collect::<BTreeSet<_>>();
    let generated = rentman_client::fields::QUERYABLE_FIELDS
        .iter()
        .map(|(rust_type, field)| (rust_type.to_string(), field.to_string()))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        documented, generated,
        "field constants drifted from openapi/rentman-oas.json; run `cargo xtask generate-openapi --only fields`"
    );
}

#[test]
fn every_link_targets_its_documented_collection() {
    let spec = openapi_spec();
    let documented = spec["components"]["schemas"]
        .as_object()
        .expect("OpenAPI schemas are an object")
        .iter()
        .flat_map(|(name, schema)| {
            schema["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter_map(move |(field, property)| {
                    let example = property["example"].as_str()?;
                    let (collection, id) = example.strip_prefix('/')?.split_once('/')?;
                    id.parse::<u64>().ok()?;
                    Some((name.clone(), field.clone(), collection.to_string()))
                })
        })
        // Documented as a link, but Rentman sends the project name instead.
        .filter(|(name, field, _)| {
            (name.as_str(), field.as_str()) != ("PurchaseOrderCostResponse", "project")
        })
        .collect::<BTreeSet<_>>();
    let generated = LINK_FIELDS
        .iter()
        .map(|(schema, field, collection)| {
            (
                schema.to_string(),
                field.to_string(),
                collection.to_string(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        documented, generated,
        "link fields drifted from openapi/rentman-oas.json; run `cargo xtask generate-openapi --only models`"
    );

    let collections = RESOURCE_COLLECTIONS
        .iter()
        .map(|(_, collection)| *collection)
        .collect::<BTreeSet<_>>();
    for (schema, field, collection) in LINK_FIELDS {
        assert!(
            collections.contains(collection),
            "{schema}.{field} links to /{collection}, which is no resource"
        );
    }
}

#[test]
fn every_resource_is_the_item_type_of_its_collection() {
    let spec = openapi_spec();
    let documented = spec["paths"]
        .as_object()
        .expect("OpenAPI paths are an object")
        .iter()
        .filter_map(|(path, methods)| {
            let collection = path.strip_prefix('/')?.strip_suffix("/{id}")?;
            if collection.contains('/') {
                return None;
            }
            let schema = methods["get"]["responses"]["200"]["content"]["application/json"]
                ["schema"]["allOf"]
                .as_array()?
                .iter()
                .find_map(|part| part["properties"]["data"]["$ref"].as_str())?;
            let schema = schema.rsplit('/').next()?;
            let rust_type = SCHEMA_TYPES
                .iter()
                .find(|(name, _)| *name == schema)
                .map(|(_, rust_type)| *rust_type)?;
            Some((rust_type.to_string(), collection.to_string()))
        })
        .collect::<BTreeSet<_>>();
    let generated = RESOURCE_COLLECTIONS
        .iter()
        .map(|(rust_type, collection)| (rust_type.to_string(), collection.to_string()))
        .collect::<BTreeSet<_>>();
    assert_eq!(documented, generated);

    assert_eq!(<Crew as Resource>::COLLECTION, "crew");
    assert_eq!(<Invoice as Resource>::COLLECTION, "invoices");
    assert_eq!(<Rate as Resource>::COLLECTION, "rates");
}

#[test]
fn every_resource_model_carries_the_documented_revision() {
    let spec = openapi_spec();
    let introduction = spec["info"]["description"]
        .as_str()
        .expect("OpenAPI introduction is a string");
    assert!(
        introduction.contains("## updateHash")
            && introduction.contains("computed with the `id` and `modified` field"),
        "the API introduction no longer documents updateHash"
    );

    for (name, schema) in spec["components"]["schemas"]
        .as_object()
        .expect("OpenAPI schemas are an object")
    {
        assert!(
            schema["properties"].get("updateHash").is_none(),
            "{name} now documents updateHash; review the synthetic revision mapping"
        );
    }

    let resources = RESOURCE_COLLECTIONS
        .iter()
        .map(|(resource, _)| *resource)
        .collect::<BTreeSet<_>>();
    let generated = generated_revisions::RESOURCES_WITH_REVISION
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(resources, generated);
    assert!(
        rentman_client::fields::QUERYABLE_FIELDS
            .iter()
            .all(|(_, field)| *field != "updateHash")
    );
}

#[test]
fn generated_models_read_and_write_typed_links() {
    let payment: Payment = serde_json::from_value(json!({
        "id": 99, "updateHash": "payment-99", "created": null, "modified": null,
        "creator": "/crew/3",
        "displayname": "Payment 99", "invoice": "/invoices/7",
        "moment": "2026-07-05T00:00:00.000+02:00", "amount": 42.5,
        "description": "Manual booking"
    }))
    .unwrap();
    assert_eq!(payment.id, PaymentId::new(99));
    assert_eq!(payment.invoice, Some(InvoiceId::new(7)));
    assert_eq!(payment.creator, Some(CrewId::new(3)));
    let wire = serde_json::to_value(&payment).unwrap();
    assert_eq!(wire["id"], json!(99));
    assert_eq!(wire["invoice"], json!("/invoices/7"));
    assert_eq!(wire["creator"], json!("/crew/3"));

    // A nullable link is present even when it has no value.
    let payment: Payment = serde_json::from_value(json!({
        "id": 99, "updateHash": "payment-99", "created": null, "modified": null,
        "creator": null,
        "displayname": "Payment 99",
        "invoice": "/invoices/7", "moment": "2026-07-05T00:00:00.000+02:00",
        "amount": 1, "description": ""
    }))
    .unwrap();
    assert_eq!(payment.creator, None);

    let error = serde_json::from_value::<Payment>(json!({
        "id": 99, "updateHash": "payment-99", "created": null, "modified": null,
        "creator": null,
        "displayname": "Payment 99", "invoice": "/projects/7",
        "moment": "2026-07-05T00:00:00.000+02:00", "amount": 1, "description": ""
    }))
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("link to /projects where a link to /invoices was expected"),
        "{error}"
    );

    let link = LinkError::WrongCollection {
        expected: "invoices",
        found: "projects".to_string(),
    };
    assert_eq!(InvoiceId::from_link("/projects/7"), Err(link));
}

#[test]
fn wire_email_addresses_are_validated() {
    let email: EmailAddress = serde_json::from_str(r#""support@example.test""#).unwrap();
    assert_eq!(email.as_str(), "support@example.test");

    let empty: EmailAddress = serde_json::from_str(r#""""#).unwrap();
    assert_eq!(empty.as_str(), "");

    let error = serde_json::from_str::<EmailAddress>(r#""not-an-email""#).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Rentman email address must contain '@'")
    );

    assert!(serde_json::from_str::<EmailAddress>("1").is_err());
}

#[test]
fn custom_fields_serialize_and_deserialize_application_types() {
    #[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
    enum Choice {
        #[serde(rename = "one")]
        One,
        #[serde(rename = "two")]
        Two,
    }

    let mut fields = rentman_client::model::CustomFields::default();
    fields.set(8, &Choice::Two).unwrap();
    assert_eq!(fields.get::<Choice>(8).unwrap(), Choice::Two);
    assert_eq!(
        fields.0.get("custom_8"),
        Some(&rentman_client::model::CustomFieldValue::Text(
            "two".to_string()
        ))
    );
    assert!(matches!(
        fields.get::<Choice>(99),
        Err(rentman_client::CustomFieldValueError::Missing { id: 99 })
    ));
    assert!(matches!(
        fields.get::<String>(99),
        Err(rentman_client::CustomFieldValueError::Missing { id: 99 })
    ));
    assert!(matches!(
        fields.get::<bool>(8),
        Err(rentman_client::CustomFieldValueError::Json(_))
    ));

    let raw = serde_json::value::RawValue::from_string("1e9999".to_string()).unwrap();
    assert!(matches!(
        fields.set(44, &raw),
        Err(rentman_client::CustomFieldValueError::Json(_))
    ));
}

#[test]
fn wire_date_fields_serialize_as_date_only() {
    let spec = openapi_spec();
    let request_schema = schema(&spec, "ProjectsCreateFromTemplateRequest");
    assert_type(property(request_schema, "template"), "integer");
    assert_eq!(property(request_schema, "start_date")["format"], "date");

    let request = ProjectsCreateFromTemplatePayload {
        template: Id::new(7),
        start_date: NaiveDate::from_ymd_opt(2026, 10, 20).expect("valid date"),
    };
    let json = serde_json::to_value(&request).expect("request serializes");
    assert_eq!(
        json,
        serde_json::json!({"template": 7, "start_date": "2026-10-20"})
    );
    assert_eq!(
        serde_json::from_value::<ProjectsCreateFromTemplatePayload>(json).expect("request parses"),
        request
    );
    assert!(
        serde_json::from_value::<ProjectsCreateFromTemplatePayload>(
            serde_json::json!({"template": 7, "start_date": "2026-10-20T00:00:00Z"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<ProjectsCreateFromTemplatePayload>(
            serde_json::json!({"template": -1, "start_date": "2026-10-20"})
        )
        .is_err()
    );
}

#[test]
fn resource_responses_allow_unselected_fields_and_ignore_unknown_fields() {
    let equipment: Equipment = serde_json::from_value(json!({
        "id": 5463,
        "created": "2024-02-12T11:15:30+01:00",
        "modified": "2026-10-21T16:02:33+02:00",
        "creator": "/crew/244",
        "updateHash": "opaque",
        "futureField": "ignored"
    }))
    .unwrap();

    assert_eq!(equipment.id, Id::new(5463));
    assert_eq!(equipment.creator, Some(CrewId::new(244)));
    assert!(equipment.price.is_none());
    assert!(equipment.custom.is_none());

    let serialized = serde_json::to_value(equipment).unwrap();
    assert!(serialized.get("futureField").is_none());
    assert_eq!(serialized["updateHash"], "opaque");
    assert!(serialized.get("price").is_none());

    let nullable_core = json!({
        "id": 5463,
        "updateHash": "opaque",
        "created": null,
        "modified": null,
        "creator": null
    });
    let equipment: Equipment = serde_json::from_value(nullable_core.clone()).unwrap();
    assert!(equipment.created.is_none());
    assert!(equipment.modified.is_none());
    assert!(equipment.creator.is_none());

    for field in ["id", "updateHash", "created", "modified", "creator"] {
        let mut response = nullable_core.clone();
        response.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<Equipment>(response).is_err(),
            "missing core field {field} must fail"
        );
    }
}

#[test]
fn equipment_decodes_the_full_live_collection_response() {
    let response = r#"{"data":{"id":5463,"created":"2024-02-12T11:15:30+01:00","modified":"2026-10-21T18:08:26+02:00","creator":"/crew/244","displayname":"Testartikel","folder":null,"code":"string","factor_group":"/factorgroups/1","name":"Testartikel","internal_remark":"string","external_remark":"string","unit":"string","in_shop":false,"surface_article":false,"shop_description_short":"string","shop_description_long":"string","shop_seo_title":"string","shop_seo_keyword":"string","shop_seo_description":"string","shop_featured":false,"price":111303.8521365961,"subrental_costs":4889.1308024085,"critical_stock_level":5528,"type":"item","rental_sales":"Rental","temporary":false,"in_planner":false,"in_archive":false,"stock_management":"Exclude from stock tracking","taxclass":"/taxclasses/1","list_price":7709.0760923012,"volume":167.55486115617,"packed_per":5707,"height":2808.3098759169,"width":2885.372883073,"length":3910.4114034759,"weight":5198.3339072367,"empty_weight":8671.1863504942,"power":936.669092379,"current":6159.1688126785,"country_of_origin":"gh","image":null,"ledger":"/ledgercodes/1","ledger_debit":"/ledgercodes/1","defaultgroup":"string","is_combination":false,"is_physical":"Physical equipment","can_edit_content_during_planning":false,"strict_container_content":"Unrestricted","current_quantity_excl_cases":1,"updateHash":"73e663812b40a811fafeafe330b5c270","custom":{"custom_8":null,"custom_9":10,"custom_17":false,"custom_19":false,"custom_23":0,"custom_24":0,"custom_25":0,"custom_26":0,"custom_27":0,"custom_30":1,"custom_37":5,"custom_48":null,"custom_49":null}},"itemCount":1,"limit":1,"offset":0}"#;
    let response: Value = serde_json::from_str(response).unwrap();
    assert_eq!(response["itemCount"], json!(1));
    assert_eq!(response["limit"], json!(1));
    assert_eq!(response["offset"], json!(0));

    let equipment: Equipment = serde_json::from_value(response["data"].clone()).unwrap();
    assert_eq!(equipment.id, Id::new(5463));
    assert_eq!(equipment.creator, Some(CrewId::new(244)));
    assert_eq!(equipment.factor_group, Some(Id::new(1)));
    assert_eq!(equipment.folder, None);
    assert_eq!(equipment.type_, Some(EquipmentType::Item));
    assert_eq!(equipment.rental_sales, Some(EquipmentRentalSales::Rental));
    assert_eq!(
        equipment.stock_management,
        Some(EquipmentStockManagement::ExcludeFromStockTracking)
    );
    assert_eq!(
        equipment.is_physical,
        Some(EquipmentIsPhysical::PhysicalEquipment)
    );
    assert_eq!(
        equipment.strict_container_content,
        Some(EquipmentStrictContainerContent::Unrestricted)
    );
    assert_eq!(
        equipment.price.unwrap().as_decimal(),
        "111303.8521365961".parse().unwrap()
    );
    let custom = equipment.custom.unwrap();
    assert_eq!(
        custom.0.get("custom_8"),
        Some(&rentman_client::model::CustomFieldValue::Null)
    );
    assert_eq!(
        custom.0.get("custom_9"),
        Some(&rentman_client::model::CustomFieldValue::Number(10.into()))
    );
    assert_eq!(
        custom.0.get("custom_17"),
        Some(&rentman_client::model::CustomFieldValue::Bool(false))
    );
}

#[test]
fn every_documented_response_enum_is_open() {
    let spec = openapi_spec();
    let documented = spec["components"]["schemas"]
        .as_object()
        .expect("OpenAPI schemas are an object")
        .iter()
        .filter(|(name, _)| name.ends_with("Response"))
        .flat_map(|(name, schema)| {
            schema["properties"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(_, property)| property.get("enum").is_some())
                .map(move |(field, _)| (name.clone(), field.clone()))
        })
        .collect::<BTreeSet<_>>();
    let covered = generated_response_enums::RESPONSE_ENUM_FIELDS
        .iter()
        .map(|(schema, field)| (schema.to_string(), field.to_string()))
        .collect::<BTreeSet<_>>();

    assert_eq!(
        documented, covered,
        "response enums drifted from openapi/rentman-oas.json; run `cargo xtask generate-openapi --only tests`"
    );
}

#[test]
fn undocumented_response_enum_values_are_kept_as_unknown() {
    let documented: TaxClassType = serde_json::from_value(json!("vat")).unwrap();
    assert_eq!(documented, TaxClassType::Vat);
    assert_eq!(serde_json::to_value(&documented).unwrap(), json!("vat"));

    let unknown: TaxClassType = serde_json::from_value(json!("reverse_charge")).unwrap();
    assert_eq!(unknown, TaxClassType::Unknown("reverse_charge".to_string()));
    assert_eq!(
        serde_json::to_value(&unknown).unwrap(),
        json!("reverse_charge")
    );

    // Values are matched exactly; a different spelling is not documented.
    let unknown: TaxClassType = serde_json::from_value(json!("VAT")).unwrap();
    assert_eq!(unknown, TaxClassType::Unknown("VAT".to_string()));

    // Only strings are enum values; other JSON types still fail.
    assert!(serde_json::from_value::<TaxClassType>(json!(1)).is_err());
}

#[test]
fn an_undocumented_enum_value_does_not_fail_the_collection() {
    let items: Vec<TaxClass> = serde_json::from_value(json!(
        [
            {"id": 1, "updateHash": "tax-1", "created": null, "modified": null, "creator": null,
             "displayname": "VAT 19%", "name": "VAT 19%", "code": "19", "type": "vat"},
            {"id": 2, "updateHash": "tax-2", "created": null, "modified": null, "creator": null,
             "displayname": "New", "name": "New", "code": "N", "type": "reverse_charge"}
        ]
    ))
    .unwrap();

    assert_eq!(items[0].type_, Some(TaxClassType::Vat));
    assert_eq!(
        items[1].type_,
        Some(TaxClassType::Unknown("reverse_charge".to_string()))
    );
    assert_eq!(
        serde_json::to_value(&items[1]).unwrap()["type"],
        json!("reverse_charge")
    );
}

/// Asserts that every value the OpenAPI document lists for `schema.field`
/// deserializes to a named variant and serializes back unchanged.
fn assert_documented_values_are_named<E>(schema_name: &str, field: &str, is_unknown: fn(&E) -> bool)
where
    E: DeserializeOwned + Serialize + Debug,
{
    let spec = openapi_spec();
    let values = property(schema(&spec, schema_name), field)["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("{schema_name}.{field} is no longer an enum in OpenAPI"));
    for value in values {
        // The generator models every enum value as its string form.
        let text = value
            .as_str()
            .map_or_else(|| value.to_string(), str::to_string);
        let parsed: E = serde_json::from_value(json!(text))
            .unwrap_or_else(|error| panic!("{schema_name}.{field} = {text:?}: {error}"));
        assert!(
            !is_unknown(&parsed),
            "{schema_name}.{field} = {text:?} is documented but has no named variant; run `cargo xtask generate-openapi --only models`"
        );
        assert_eq!(serde_json::to_value(&parsed).unwrap(), json!(text));
    }
}

/// Asserts that a payload sends exactly the given fields, which are the
/// required fields of the documented request schema.
fn assert_sends_exactly<T: Serialize>(payload: &T, schema_name: &str, fields: &[&str]) {
    let spec = openapi_spec();
    let required = schema(&spec, schema_name)["required"]
        .as_array()
        .expect("payload schema has required fields")
        .iter()
        .map(|field| field.as_str().expect("required field name"))
        .collect::<BTreeSet<_>>();
    assert_eq!(fields.iter().copied().collect::<BTreeSet<_>>(), required);

    let sent = serde_json::to_value(payload).expect("payload serializes");
    let sent = sent
        .as_object()
        .expect("payload is a JSON object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(sent, required, "{schema_name} sends other fields");
}

fn assert_carries_revision<R, F>(sample: &str, has_custom: bool)
where
    R: Revisioned + CustomFieldsResource + DeserializeOwned + Serialize + Debug,
    F: ProjectionField<Resource = R, Value = Revision<R>>,
{
    assert_eq!(F::NAME, "updateHash");
    let mut wire: Value = serde_json::from_str(sample).expect("sample item is JSON");
    let missing_revision = wire
        .as_object_mut()
        .expect("sample item is object")
        .remove("updateHash");
    let item = serde_json::from_value::<R>(wire.clone());
    assert!(
        item.is_err(),
        "{} accepts an item without updateHash",
        R::COLLECTION
    );
    wire["updateHash"] = json!("d21b3fd9d594582874d23f8d26234732");
    let mut item: R = serde_json::from_value(wire.clone())
        .unwrap_or_else(|error| panic!("{}: {error}", R::COLLECTION));
    assert_eq!(item.id(), Id::new(1));
    assert_eq!(
        item.custom_fields_mut().is_some(),
        has_custom,
        "{}",
        R::COLLECTION
    );
    assert_eq!(
        item.revision(),
        &Revision::new("d21b3fd9d594582874d23f8d26234732").unwrap()
    );
    assert_eq!(
        serde_json::to_value(&item).unwrap()["updateHash"],
        wire["updateHash"]
    );
    assert!(missing_revision.is_none());

    wire["updateHash"] = json!("");
    assert!(serde_json::from_value::<R>(wire).is_err());
}

fn openapi_spec() -> Value {
    let spec = fs::read_to_string("openapi/rentman-oas.json").expect("OpenAPI fixture exists");
    serde_json::from_str(&spec).expect("OpenAPI fixture is valid JSON")
}

fn schema<'a>(spec: &'a Value, name: &str) -> &'a Value {
    &spec["components"]["schemas"][name]
}

fn property<'a>(schema: &'a Value, name: &str) -> &'a Value {
    &schema["properties"][name]
}

fn assert_type(schema: &Value, expected: &str) {
    assert_eq!(schema["type"], expected);
}
