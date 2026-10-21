//! Contract tests against the checked-in Rentman OpenAPI document.

use chrono::NaiveDate;
use rentman_client::endpoint::{
    ALL_ENDPOINTS, EndpointErrorSpec, EndpointParameterLocation, EndpointParameterSpec,
    EndpointParameterValueSpec, EndpointRequestSpec, EndpointResponseSpec, EndpointSpec,
};
use rentman_client::model::{
    CollectionResponse, Crew, CrewId, EmailAddress, Invoice, LINK_FIELDS, Payment,
    ProjectsCreateFromTemplatePayload, RESOURCE_COLLECTIONS, Rate, SCHEMA_TYPES, TaxClass,
    TaxClassType,
};
use rentman_client::{Id, InvoiceId, LinkError, PaymentId, Resource};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fmt::Debug, fs};

#[path = "contract/generated_payload_constructors.rs"]
mod generated_payload_constructors;
#[path = "contract/generated_response_enums.rs"]
mod generated_response_enums;

#[test]
fn endpoint_manifest_covers_every_documented_operation() {
    let documented = documented_operations();
    let covered = ALL_ENDPOINTS
        .iter()
        .copied()
        .map(OwnedEndpointSpec::from)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        documented, covered,
        "endpoint manifest drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py and review the API surface"
    );
}

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
        "wire model manifest drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py and review the schema surface"
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
    let mut documented = documented_operations()
        .into_iter()
        .map(|operation| operation.operation_id)
        .collect::<Vec<_>>();
    let mut reachable = rentman_client::resource::REACHABLE_OPERATIONS
        .iter()
        .map(|operation| operation.to_string())
        .collect::<Vec<_>>();
    documented.sort();
    reachable.sort();
    assert_eq!(
        documented, reachable,
        "resources drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py"
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
        "field constants drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py"
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
        "link fields drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py"
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
fn generated_models_read_and_write_typed_links() {
    let payment: Payment = serde_json::from_value(json!({
        "id": 99, "created": null, "modified": null, "creator": "/crew/3",
        "displayname": "Payment 99", "invoice": "/invoices/7",
        "moment": "2026-07-05T00:00:00.000+02:00", "amount": 42.5,
        "description": "Manual booking"
    }))
    .unwrap();
    assert_eq!(payment.id, PaymentId::new(99));
    assert_eq!(payment.invoice, InvoiceId::new(7));
    assert_eq!(payment.creator, Some(CrewId::new(3)));
    let wire = serde_json::to_value(&payment).unwrap();
    assert_eq!(wire["id"], json!(99));
    assert_eq!(wire["invoice"], json!("/invoices/7"));
    assert_eq!(wire["creator"], json!("/crew/3"));

    // A nullable link may also be missing.
    let payment: Payment = serde_json::from_value(json!({
        "id": 99, "created": null, "modified": null, "displayname": "Payment 99",
        "invoice": "/invoices/7", "moment": "2026-07-05T00:00:00.000+02:00",
        "amount": 1, "description": ""
    }))
    .unwrap();
    assert_eq!(payment.creator, None);

    let error = serde_json::from_value::<Payment>(json!({
        "id": 99, "created": null, "modified": null, "creator": null,
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
fn high_level_invoice_fields_match_openapi_schema() {
    let spec = openapi_spec();
    let invoice = schema(&spec, "FactuurResponse");

    assert_required(invoice, "id");
    assert_required(invoice, "number");
    assert_required(invoice, "is_paid");
    assert_type(property(invoice, "id"), "integer");
    assert_type(property(invoice, "number"), "string");
    assert_type(property(invoice, "is_paid"), "boolean");
    assert_type(property(invoice, "price_invat"), "number");
}

#[test]
fn high_level_payment_fields_match_openapi_schema() {
    let spec = openapi_spec();
    let payment = schema(&spec, "PaymentResponse");

    assert_required(payment, "id");
    assert_required(payment, "invoice");
    assert_required(payment, "moment");
    assert_type(property(payment, "id"), "integer");
    assert_type(property(payment, "invoice"), "string");
    assert_type(property(payment, "moment"), "string");
    assert_eq!(property(payment, "moment")["format"], "date-time");
    assert_type(property(payment, "amount"), "number");
    assert_type(property(payment, "description"), "string");
}

#[test]
fn high_level_equipment_fields_match_openapi_schema() {
    let spec = openapi_spec();
    let equipment = schema(&spec, "EquipmentResponse");

    assert_required(equipment, "id");
    assert_type(property(equipment, "id"), "integer");
    for field in ["price", "list_price", "subrental_costs"] {
        assert_type(property(equipment, field), "number");
    }
    assert_type(property(equipment, "in_archive"), "boolean");
}

#[test]
fn equipment_price_payload_matches_openapi_schema() {
    let spec = openapi_spec();
    let equipment = schema(&spec, "EquipmentRequest");

    for field in ["price", "list_price", "subrental_costs"] {
        assert_type(property(equipment, field), "number");
    }
}

#[test]
fn payment_payload_matches_openapi_schema() {
    let spec = openapi_spec();
    let payment = schema(&spec, "PaymentRequest");

    assert_required(payment, "moment");
    assert_type(property(payment, "moment"), "string");
    assert_eq!(property(payment, "moment")["format"], "date-time");
    assert_type(property(payment, "amount"), "number");
    assert_type(property(payment, "description"), "string");

    let import_source = property(payment, "payment_import_source");
    assert_type(import_source, "string");
    assert_enum_contains(import_source, "none");
    assert_enum_contains(import_source, "publicapi");
}

#[test]
fn high_level_methods_still_point_to_documented_operations() {
    let operations = documented_operations();

    assert!(operations.iter().any(|operation| operation.matches(
        "GET",
        "/invoices",
        "getFactuurCollection",
        "invoices"
    )));
    assert!(operations.iter().any(|operation| operation.matches(
        "GET",
        "/payments",
        "getPaymentCollection",
        "payments"
    )));
    assert!(operations.iter().any(|operation| operation.matches(
        "PUT",
        "/payments/{id}",
        "updatePayment",
        "payments"
    )));
    assert!(operations.iter().any(|operation| operation.matches(
        "POST",
        "/invoices/{id}/payments",
        "createPayment",
        "invoices"
    )));
    assert!(operations.iter().any(|operation| operation.matches(
        "GET",
        "/equipment",
        "getEquipmentCollection",
        "equipment"
    )));
    assert!(operations.iter().any(|operation| operation.matches(
        "PUT",
        "/equipment/{id}",
        "updateEquipment",
        "equipment"
    )));
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OwnedEndpointSpec {
    method: String,
    path: String,
    operation_id: String,
    tag: String,
    parameters: Vec<OwnedEndpointParameterSpec>,
    request: OwnedEndpointRequestSpec,
    response: OwnedEndpointResponseSpec,
    errors: Vec<OwnedEndpointErrorSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OwnedEndpointParameterSpec {
    name: String,
    location: OwnedEndpointParameterLocation,
    required: bool,
    value: OwnedEndpointParameterValueSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OwnedEndpointParameterLocation {
    Path,
    Query,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OwnedEndpointParameterValueSpec {
    Integer,
    Number,
    Boolean,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OwnedEndpointRequestSpec {
    None,
    Json(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OwnedEndpointResponseSpec {
    NoContent,
    Item(String),
    Collection(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OwnedEndpointErrorSpec {
    status: u16,
    description: String,
}

impl From<EndpointSpec> for OwnedEndpointSpec {
    fn from(value: EndpointSpec) -> Self {
        Self {
            method: value.method.to_string(),
            path: value.path.to_string(),
            operation_id: value.operation_id.to_string(),
            tag: value.tag.to_string(),
            parameters: value.parameters.iter().copied().map(Into::into).collect(),
            request: value.request.into(),
            response: value.response.into(),
            errors: value.errors.iter().copied().map(Into::into).collect(),
        }
    }
}

impl OwnedEndpointSpec {
    fn matches(&self, method: &str, path: &str, operation_id: &str, tag: &str) -> bool {
        self.method == method
            && self.path == path
            && self.operation_id == operation_id
            && self.tag == tag
    }
}

impl From<EndpointParameterSpec> for OwnedEndpointParameterSpec {
    fn from(value: EndpointParameterSpec) -> Self {
        Self {
            name: value.name.to_string(),
            location: value.location.into(),
            required: value.required,
            value: value.value.into(),
        }
    }
}

impl From<EndpointParameterLocation> for OwnedEndpointParameterLocation {
    fn from(value: EndpointParameterLocation) -> Self {
        match value {
            EndpointParameterLocation::Path => Self::Path,
            EndpointParameterLocation::Query => Self::Query,
        }
    }
}

impl From<EndpointParameterValueSpec> for OwnedEndpointParameterValueSpec {
    fn from(value: EndpointParameterValueSpec) -> Self {
        match value {
            EndpointParameterValueSpec::Integer => Self::Integer,
            EndpointParameterValueSpec::Number => Self::Number,
            EndpointParameterValueSpec::Boolean => Self::Boolean,
            EndpointParameterValueSpec::String => Self::String,
        }
    }
}

impl From<EndpointRequestSpec> for OwnedEndpointRequestSpec {
    fn from(value: EndpointRequestSpec) -> Self {
        match value {
            EndpointRequestSpec::None => Self::None,
            EndpointRequestSpec::Json(schema) => Self::Json(schema.to_string()),
        }
    }
}

impl From<EndpointResponseSpec> for OwnedEndpointResponseSpec {
    fn from(value: EndpointResponseSpec) -> Self {
        match value {
            EndpointResponseSpec::NoContent => Self::NoContent,
            EndpointResponseSpec::Item(schema) => Self::Item(schema.to_string()),
            EndpointResponseSpec::Collection(schema) => Self::Collection(schema.to_string()),
        }
    }
}

impl From<EndpointErrorSpec> for OwnedEndpointErrorSpec {
    fn from(value: EndpointErrorSpec) -> Self {
        Self {
            status: value.status,
            description: value.description.to_string(),
        }
    }
}

fn documented_operations() -> BTreeSet<OwnedEndpointSpec> {
    let spec = openapi_spec();
    spec["paths"]
        .as_object()
        .expect("OpenAPI paths are an object")
        .iter()
        .flat_map(|(path, methods)| {
            methods
                .as_object()
                .expect("OpenAPI path item is an object")
                .iter()
                .filter_map(move |(method, operation)| {
                    let operation_id = operation["operationId"].as_str()?;
                    let tag = operation["tags"]
                        .as_array()
                        .and_then(|tags| tags.first())
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    Some(OwnedEndpointSpec {
                        method: method.to_uppercase(),
                        path: path.to_string(),
                        operation_id: operation_id.to_string(),
                        tag: tag.to_string(),
                        parameters: documented_parameters(operation),
                        request: documented_request(operation),
                        response: documented_response(operation),
                        errors: documented_errors(operation),
                    })
                })
        })
        .collect()
}

fn documented_parameters(operation: &Value) -> Vec<OwnedEndpointParameterSpec> {
    operation["parameters"]
        .as_array()
        .map(|parameters| {
            parameters
                .iter()
                .map(|parameter| OwnedEndpointParameterSpec {
                    name: parameter["name"]
                        .as_str()
                        .expect("parameter has name")
                        .to_string(),
                    location: documented_parameter_location(parameter),
                    required: parameter["required"].as_bool().unwrap_or(false),
                    value: documented_parameter_value(&parameter["schema"]),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn documented_parameter_location(parameter: &Value) -> OwnedEndpointParameterLocation {
    match parameter["in"].as_str().expect("parameter has location") {
        "path" => OwnedEndpointParameterLocation::Path,
        "query" => OwnedEndpointParameterLocation::Query,
        location => panic!("unsupported OpenAPI parameter location {location}"),
    }
}

fn documented_parameter_value(schema: &Value) -> OwnedEndpointParameterValueSpec {
    match schema["type"].as_str().expect("parameter has schema type") {
        "integer" => OwnedEndpointParameterValueSpec::Integer,
        "number" => OwnedEndpointParameterValueSpec::Number,
        "boolean" => OwnedEndpointParameterValueSpec::Boolean,
        "string" => OwnedEndpointParameterValueSpec::String,
        value => panic!("unsupported OpenAPI parameter value type {value}"),
    }
}

fn documented_request(operation: &Value) -> OwnedEndpointRequestSpec {
    let schema = &operation["requestBody"]["content"]["application/json"]["schema"];
    let Some(reference) = schema["$ref"].as_str() else {
        return OwnedEndpointRequestSpec::None;
    };
    OwnedEndpointRequestSpec::Json(reference_name(reference).to_string())
}

fn documented_response(operation: &Value) -> OwnedEndpointResponseSpec {
    let schema = &operation["responses"]["200"]["content"]["application/json"]["schema"];
    if schema["type"].as_str() == Some("null") {
        return OwnedEndpointResponseSpec::NoContent;
    }

    let data = schema["allOf"]
        .as_array()
        .expect("success response uses allOf")
        .iter()
        .find_map(|item| {
            item["properties"]["data"]
                .as_object()
                .map(|_| &item["properties"]["data"])
        })
        .expect("success response has data property");

    if data["type"].as_str() == Some("array") {
        return OwnedEndpointResponseSpec::Collection(
            reference_name(data["items"]["$ref"].as_str().expect("array item has ref")).to_string(),
        );
    }

    OwnedEndpointResponseSpec::Item(
        reference_name(data["$ref"].as_str().expect("item response has ref")).to_string(),
    )
}

fn documented_errors(operation: &Value) -> Vec<OwnedEndpointErrorSpec> {
    let mut errors = operation["responses"]
        .as_object()
        .expect("operation responses are an object")
        .iter()
        .filter(|(status, _)| status.as_str() != "200")
        .map(|(status, response)| OwnedEndpointErrorSpec {
            status: status
                .parse()
                .expect("documented response status is numeric"),
            description: response["description"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        })
        .collect::<Vec<_>>();
    errors.sort();
    errors
}

fn reference_name(reference: &str) -> &str {
    reference
        .rsplit_once('/')
        .map(|(_, name)| name)
        .expect("OpenAPI reference has a schema name")
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
        "response enums drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py"
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
    let page: CollectionResponse<TaxClass> = serde_json::from_value(json!({
        "itemCount": 2,
        "limit": 300,
        "data": [
            {"id": 1, "created": null, "modified": null, "creator": null,
             "displayname": "VAT 19%", "name": "VAT 19%", "code": "19", "type": "vat"},
            {"id": 2, "created": null, "modified": null, "creator": null,
             "displayname": "New", "name": "New", "code": "N", "type": "reverse_charge"}
        ]
    }))
    .unwrap();

    assert_eq!(page.data[0].type_, TaxClassType::Vat);
    assert_eq!(
        page.data[1].type_,
        TaxClassType::Unknown("reverse_charge".to_string())
    );
    assert_eq!(
        serde_json::to_value(&page.data[1]).unwrap()["type"],
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
            "{schema_name}.{field} = {text:?} is documented but has no named variant; run scripts/generate_endpoint_manifest.py"
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

fn assert_required(schema: &Value, name: &str) {
    let required = schema["required"]
        .as_array()
        .expect("schema has required array");
    assert!(
        required.iter().any(|value| value.as_str() == Some(name)),
        "{name} should still be required in OpenAPI"
    );
}

fn assert_type(schema: &Value, expected: &str) {
    assert_eq!(schema["type"], expected);
}

fn assert_enum_contains(schema: &Value, expected: &str) {
    let variants = schema["enum"].as_array().expect("schema has enum array");
    assert!(
        variants
            .iter()
            .any(|value| value.as_str() == Some(expected)),
        "{expected} should still be allowed by OpenAPI"
    );
}
