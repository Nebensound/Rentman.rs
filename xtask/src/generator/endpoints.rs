use crate::openapi::{Context, Operation, Response, escape};
use std::fmt::Write as _;

pub fn generate(context: &Context<'_>, operations: &[Operation]) -> String {
    let mut output = String::from(
        "use super::{Endpoint, EndpointErrorSpec, EndpointParameterLocation, EndpointParameterSpec, EndpointParameterValueSpec, EndpointRequestSpec, EndpointResponseSpec, EndpointSpec, ExecutableEndpoint};\nuse crate::model::*;\nuse crate::model::{CollectionResponse, ItemResponse, NoContent, NoRequest};\n\n",
    );
    for operation in operations {
        let parameter_name = format!("{}_PARAMETERS", rust_const(&operation.endpoint_name));
        writeln!(
            output,
            "const {parameter_name}: &[EndpointParameterSpec] = &["
        )
        .unwrap();
        for parameter in &operation.parameters {
            writeln!(output, "    EndpointParameterSpec {{\n        name: \"{}\",\n        location: EndpointParameterLocation::{},\n        required: {},\n        value: EndpointParameterValueSpec::{},\n    }},",
                escape(&parameter.name), parameter.location, parameter.required, parameter.value).unwrap();
        }
        output.push_str("];\n\n");
        let error_name = format!("{}_ERRORS", rust_const(&operation.endpoint_name));
        writeln!(output, "const {error_name}: &[EndpointErrorSpec] = &[").unwrap();
        for (status, description) in &operation.errors {
            writeln!(
                output,
                "    EndpointErrorSpec {{ status: {status}, description: \"{}\" }},",
                escape(description)
            )
            .unwrap();
        }
        output.push_str("];\n\n");
        let request_type = operation
            .request
            .as_deref()
            .map_or("NoRequest", |schema| context.type_name(schema));
        let request_spec = operation.request.as_deref().map_or_else(
            || "EndpointRequestSpec::None".to_owned(),
            |name| format!("EndpointRequestSpec::Json(\"{}\")", escape(name)),
        );
        let (response_type, response_spec) = match &operation.response {
            Response::NoContent => (
                "NoContent".to_owned(),
                "EndpointResponseSpec::NoContent".to_owned(),
            ),
            Response::Collection(schema) => (
                format!("CollectionResponse<{}>", context.type_name(schema)),
                format!("EndpointResponseSpec::Collection(\"{}\")", escape(schema)),
            ),
            Response::Item(schema) => (
                format!("ItemResponse<{}>", context.type_name(schema)),
                format!("EndpointResponseSpec::Item(\"{}\")", escape(schema)),
            ),
        };
        writeln!(output, "/// Typed endpoint for `{} {}`.\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) struct {};\n\nimpl Endpoint for {} {{\n    type Request = {request_type};\n    type Response = {response_type};\n\n    const SPEC: EndpointSpec = EndpointSpec {{\n        method: \"{}\",\n        path: \"{}\",\n        operation_id: \"{}\",\n        tag: \"{}\",\n        parameters: {parameter_name},\n        request: {request_spec},\n        response: {response_spec},\n        errors: {error_name},\n    }};\n}}\n",
            operation.method, operation.path, operation.endpoint_name, operation.endpoint_name,
            escape(&operation.method), escape(&operation.path), escape(&operation.operation_id), escape(&operation.tag)).unwrap();
    }
    output.push_str("/// All operations in the checked-in Rentman API document.\npub(crate) const ALL_ENDPOINTS: &[EndpointSpec] = &[\n");
    for operation in operations {
        writeln!(
            output,
            "    <{} as Endpoint>::SPEC,",
            operation.endpoint_name
        )
        .unwrap();
    }
    output.push_str("];\n\n#[allow(dead_code)]\n// Coverage is disabled because this compile-time assertion has no runtime\n// behavior; contract tests cover the generated endpoint inventory.\n#[cfg_attr(coverage_nightly, coverage(off))]\npub(crate) fn assert_all_endpoints_are_executable()\nwhere\n");
    for operation in operations {
        writeln!(
            output,
            "    {}: ExecutableEndpoint,",
            operation.endpoint_name
        )
        .unwrap();
    }
    output.push_str("{\n}\n");
    output
}

/// Endpoint names come from `rust_ident`, so they are ASCII alphanumeric.
fn rust_const(name: &str) -> String {
    name.to_ascii_uppercase()
}
