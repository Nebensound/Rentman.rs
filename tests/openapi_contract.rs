//! Contract tests against the checked-in Rentman OpenAPI document.

use rentman_client::endpoint::{
    ALL_ENDPOINTS, EndpointErrorSpec, EndpointParameterLocation, EndpointParameterSpec,
    EndpointParameterValueSpec, EndpointRequestSpec, EndpointResponseSpec, EndpointSpec,
};
use rentman_client::model::{ALL_SCHEMAS, EmailAddress, RentmanId, ResourceReference};
use serde_json::Value;
use std::{collections::BTreeSet, fs};

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
    let covered = ALL_SCHEMAS
        .iter()
        .map(|schema| schema.to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        documented, covered,
        "wire model manifest drifted from openapi/rentman-oas.json; run scripts/generate_endpoint_manifest.py and review the schema surface"
    );
}

#[test]
fn wire_resource_references_are_validated_paths() {
    let reference: ResourceReference = serde_json::from_str(r#""/invoices/14""#).unwrap();
    assert_eq!(reference.as_str(), "/invoices/14");

    let error = serde_json::from_str::<ResourceReference>(r#""invoices/14""#).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Rentman resource reference must start with '/'")
    );
}

#[test]
fn wire_email_addresses_are_validated() {
    let email: EmailAddress = serde_json::from_str(r#""support@example.test""#).unwrap();
    assert_eq!(email.as_str(), "support@example.test");

    let error = serde_json::from_str::<EmailAddress>(r#""not-an-email""#).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Rentman email address must contain '@'")
    );
}

#[test]
fn wire_rentman_id_displays_plain_number() {
    assert_eq!(RentmanId(42).to_string(), "42");
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
