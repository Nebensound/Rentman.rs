//! Typed Rentman API endpoint definitions: the crate-internal foundation of
//! the resource layer.
//!
//! The metadata records every documented operation, parameter, body, and error
//! status of the OpenAPI document. The resource layer reads only what it needs
//! to send requests; the contract test below reads all of it, so unused parts
//! are allowed here deliberately.
#![allow(dead_code)]

mod generated;

use serde::{Serialize, de::DeserializeOwned};

pub(crate) use generated::*;

/// Documented Rentman API operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointSpec {
    /// HTTP method in uppercase.
    pub method: &'static str,
    /// Rentman API path template.
    pub path: &'static str,
    /// Rentman API operation ID.
    pub operation_id: &'static str,
    /// First Rentman API tag of the operation.
    pub tag: &'static str,
    /// Documented path or query parameters of the operation.
    pub parameters: &'static [EndpointParameterSpec],
    /// JSON request body type of the operation.
    pub request: EndpointRequestSpec,
    /// Successful response body type of the operation.
    pub response: EndpointResponseSpec,
    /// Documented non-success responses of the operation.
    pub errors: &'static [EndpointErrorSpec],
}

/// Documented path or query parameter of an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndpointParameterSpec {
    /// Parameter name as documented by Rentman.
    pub name: &'static str,
    /// Location of the parameter in the HTTP request.
    pub location: EndpointParameterLocation,
    /// Whether the parameter is required by the Rentman API document.
    pub required: bool,
    /// Documented value type of the parameter.
    pub value: EndpointParameterValueSpec,
}

/// HTTP request location of an endpoint parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndpointParameterLocation {
    /// Parameter is rendered into the path template.
    Path,
    /// Parameter is rendered into the URL query string.
    Query,
}

/// Documented primitive value shape of an endpoint parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndpointParameterValueSpec {
    /// Integer parameter.
    Integer,
    /// Number parameter.
    Number,
    /// Boolean parameter.
    Boolean,
    /// String parameter.
    String,
}

/// Documented non-success response of an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndpointErrorSpec {
    /// HTTP status code.
    pub status: u16,
    /// Rentman API response description.
    pub description: &'static str,
}

/// Request-body shape of a documented endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndpointRequestSpec {
    /// The endpoint defines no JSON request body.
    None,
    /// The endpoint accepts a JSON body matching the named Rentman API schema.
    Json(&'static str),
}

/// Successful response-body shape of a documented endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndpointResponseSpec {
    /// The endpoint returns no JSON data.
    NoContent,
    /// The endpoint returns an item of the named Rentman API schema.
    Item(&'static str),
    /// The endpoint returns a collection of the named Rentman API schema.
    Collection(&'static str),
}

/// Typed representation of a documented endpoint.
pub(crate) trait Endpoint {
    /// JSON request body type.
    type Request;
    /// Successful response body type.
    type Response;

    /// Static endpoint metadata from the checked-in Rentman API definition.
    const SPEC: EndpointSpec;
}

/// Endpoint whose request and response types can be sent through `RentmanClient`.
pub(crate) trait ExecutableEndpoint: Endpoint
where
    Self::Request: Serialize,
    Self::Response: DeserializeOwned,
{
}

impl<E> ExecutableEndpoint for E
where
    E: Endpoint,
    E::Request: Serialize,
    E::Response: DeserializeOwned,
{
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod contract_tests {
    //! The endpoint layer is crate-internal, so its contract test against the
    //! OpenAPI document lives next to it instead of in `tests/`.

    use super::*;
    use serde_json::Value;
    use std::{collections::BTreeSet, fs};

    fn openapi_spec() -> Value {
        let spec = fs::read_to_string("openapi/rentman-oas.json").expect("OpenAPI fixture exists");
        serde_json::from_str(&spec).expect("OpenAPI fixture is valid JSON")
    }

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
                reference_name(data["items"]["$ref"].as_str().expect("array item has ref"))
                    .to_string(),
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
}
