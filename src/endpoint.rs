//! Typed Rentman API endpoint definitions.

mod generated;

use serde::{Serialize, de::DeserializeOwned};

pub use generated::*;

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
pub trait Endpoint {
    /// JSON request body type.
    type Request;
    /// Successful response body type.
    type Response;

    /// Static endpoint metadata from the checked-in Rentman API definition.
    const SPEC: EndpointSpec;
}

/// Endpoint whose path contains a documented integer `{id}` parameter.
pub trait EndpointWithId: Endpoint {}

/// Endpoint whose request and response types can be sent through `RentmanClient`.
pub trait ExecutableEndpoint: Endpoint
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
