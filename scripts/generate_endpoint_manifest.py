#!/usr/bin/env python3
"""Regenerate the checked-in typed Rentman OpenAPI contract."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OPENAPI = ROOT / "openapi" / "rentman-oas.json"
ENDPOINT_OUTPUT = ROOT / "src" / "endpoint" / "generated.rs"
WIRE_OUTPUT = ROOT / "src" / "model" / "generated.rs"
LIVE_SWEEP_OUTPUT = ROOT / "tests" / "live" / "generated_sweep.rs"

RUST_KEYWORDS = {
    "as",
    "async",
    "await",
    "break",
    "crate",
    "dyn",
    "enum",
    "fn",
    "impl",
    "in",
    "let",
    "match",
    "mod",
    "move",
    "pub",
    "ref",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "type",
    "use",
    "where",
}

DATE_TIME_FIELDS = {"created", "modified"}
URL_FIELDS = {"url", "website", "image", "logo"}

# Deliberate deviations from the documented OpenAPI types, verified against
# live API payloads on 2026-07-21 (see the live sweep in tests/live_api.rs).
# They apply to Response schemas only: request bodies keep the documented
# types because they define what this client sends.

# Fields whose live values contradict the documented type.
RESPONSE_TYPE_OVERRIDES = {
    # quantity is documented as string but returned as an integer.
    ("ActualContentResponse", "quantity"): "i64",
    ("EquipmentSetContentResponse", "quantity"): "i64",
    ("ProjectEquipmentResponse", "quantity"): "i64",
    # factor is documented as string but returned as a number (e.g. 2.8).
    ("FactorsResponse", "factor"): "Decimal",
    ("ProjectEquipmentResponse", "factor"): "Decimal",
    ("ProjectRequestEquipmentResponse", "factor"): "Decimal",
    ("SubrentalEquipmentResponse", "factor"): "Decimal",
    # number is documented as string but returned as an integer here; it is a
    # real string on contracts, invoices, and purchase orders.
    ("ProjectResponse", "number"): "i64",
    ("RepairResponse", "number"): "i64",
    ("SubrentalResponse", "number"): "i64",
    # contract is documented as string but returned as an integer.
    ("CrewResponse", "contract"): "i64",
    # public is documented as the string enum "0"/"1" but returned as an
    # integer.
    ("TaskResponse", "public"): "i64",
    # project is documented as a resource reference but returns the project
    # display name.
    ("PurchaseOrderCostResponse", "project"): "String",
    # website holds user-entered text (often empty, or missing a scheme).
    ("ContactResponse", "website"): "String",
}

# Fields that are documented as non-nullable but return null in live data.
RESPONSE_NULLABLE_FIELDS = {
    ("ContactResponse", "mailing_unit_number"),
    ("ContactResponse", "mailing_district"),
    ("ContactResponse", "mailing_extra_address_line"),
    ("ContactResponse", "visit_unit_number"),
    ("ContactResponse", "visit_district"),
    ("ContactResponse", "visit_extra_address_line"),
    ("ContactResponse", "invoice_unit_number"),
    ("ContactResponse", "invoice_district"),
    ("ContactResponse", "invoice_extra_address_line"),
    ("CrewResponse", "unit_number"),
    ("CrewResponse", "district"),
    ("CrewResponse", "extraaddressline"),
    ("FactuurResponse", "integration_reference_id"),
    ("ProjectFunctionGroupResponse", "remark"),
    ("PurchaseOrderResponse", "export_message"),
    ("TaskResponse", "color"),
}

# Documented-required fields that live collection responses omit (most are
# financial aggregates that only appear when explicitly requested).
_PROJECT_PRICE_FIELDS = (
    "project_total_price",
    "project_total_price_cancelled",
    "project_rental_price",
    "project_sale_price",
    "project_crew_price",
    "project_transport_price",
    "project_other_price",
    "project_insurance_price",
)
_PROJECT_COST_FIELDS = ("estimated_cost", "planned_cost", "actual_cost")
RESPONSE_OPTIONAL_FIELDS = {
    "ContractResponse": set(_PROJECT_PRICE_FIELDS),
    "FactuurResponse": set(_PROJECT_PRICE_FIELDS),
    "QuotationResponse": set(_PROJECT_PRICE_FIELDS),
    "ProjectResponse": set(_PROJECT_PRICE_FIELDS + _PROJECT_COST_FIELDS),
    "SubprojectResponse": set(_PROJECT_PRICE_FIELDS + _PROJECT_COST_FIELDS),
    "EquipmentResponse": {
        "current_quantity",
        "current_quantity_excl_cases",
        "quantity_in_cases",
    },
    "FileFolderResponse": {"parent_api_path"},
    "FileResponse": {"parent_api_path", "itemtype"},
    "InvoiceLineResponse": {"parent_api_path"},
    "TaskResponse": {"parent_api_path"},
    "PaymentResponse": {"payment_import_source"},
    "ExtraInputFieldResponse": {"linkedItemType"},
    "PurchaseOrderResponse": {"previous_status"},
}


def escape(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def schema_name(ref: str) -> str:
    return ref.rsplit("/", 1)[1]


def rust_ident(value: str) -> str:
    parts = re.split(r"[^A-Za-z0-9]+", str(value))
    ident = "".join(part[:1].upper() + part[1:] for part in parts if part)
    if not ident:
        ident = "Value"
    if ident[0].isdigit():
        ident = f"Value{ident}"
    return ident


def rust_field(value: str) -> str:
    field = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    field = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", field)
    field = field.lower()
    field = re.sub(r"[^A-Za-z0-9_]", "_", field)
    field = re.sub(r"_+", "_", field).strip("_")
    if not field:
        field = "value"
    if field[0].isdigit():
        field = f"field_{field}"
    if field in RUST_KEYWORDS:
        field = f"{field}_"
    return field


def rust_doc(value: str | None, fallback: str) -> str:
    text = (value or fallback).strip() or fallback
    text = re.sub(r"\s+", " ", text)
    return escape(text)


def openapi_type(schema: dict) -> tuple[str | None, bool]:
    value = schema.get("type")
    if isinstance(value, list):
        non_null = [item for item in value if item != "null"]
        if len(non_null) != 1:
            raise ValueError(f"unsupported union type: {value}")
        return non_null[0], True
    return value, False


def enum_type_name(schema: str, field: str) -> str:
    return f"{schema}{rust_ident(field)}"


def enum_variant_name(value: object, used: set[str]) -> str:
    text = str(value)
    replacements = {
        "0": "Zero",
        "1": "One",
        "+": "Plus",
        "-": "Minus",
    }
    base = replacements.get(text, rust_ident(text))
    if base in RUST_KEYWORDS:
        base = f"{base}Value"
    variant = base
    suffix = 2
    while variant in used:
        variant = f"{base}{suffix}"
        suffix += 1
    used.add(variant)
    return variant


def is_reference_field(field: str, schema: dict) -> bool:
    example = schema.get("example")
    if isinstance(example, str) and example.startswith("/"):
        return True
    return field in {
        "creator",
        "invoice",
        "customer",
        "account_manager",
        "contact",
        "project",
        "folder",
        "parent",
        "equipment",
        "serialnumber",
        "vehicle",
        "crew",
        "supplier",
        "subproject",
        "task",
    }


def is_url_field(field: str) -> bool:
    return field in URL_FIELDS or field.endswith("_url")


def base_rust_type(schema_name_value: str, field: str, schema: dict) -> str:
    if schema_name_value.endswith("Response"):
        override = RESPONSE_TYPE_OVERRIDES.get((schema_name_value, field))
        if override:
            return override
        # order is documented as string on many response schemas but is an
        # integer in every observed live payload.
        if field == "order" and openapi_type(schema)[0] == "string":
            return "i64"

    if "enum" in schema:
        return enum_type_name(schema_name_value, field)

    kind, _ = openapi_type(schema)
    if kind == "integer":
        return "RentmanId" if field == "id" or field.endswith("_id") else "i64"
    if kind == "number":
        return "Decimal"
    if kind == "boolean":
        return "bool"
    if kind == "object":
        return "CustomFields"
    if kind == "string":
        if schema.get("format") == "date-time" or field in DATE_TIME_FIELDS:
            return "DateTime<FixedOffset>"
        if schema.get("format") == "email":
            return "EmailAddress"
        if is_reference_field(field, schema):
            return "ResourceReference"
        if is_url_field(field):
            return "Url"
        return "String"

    raise ValueError(f"unsupported property type for {schema_name_value}.{field}: {schema}")


def rust_field_type(schema_name_value: str, field: str, schema: dict, required: set[str]) -> str:
    _, nullable = openapi_type(schema)
    if (schema_name_value, field) in RESPONSE_NULLABLE_FIELDS:
        nullable = True
    base = base_rust_type(schema_name_value, field, schema)
    if nullable or field not in required:
        return f"Option<{base}>"
    return base


def effective_required(schema_name_value: str, schema: dict) -> set[str]:
    required = set(schema.get("required") or [])
    return required - RESPONSE_OPTIONAL_FIELDS.get(schema_name_value, set())


def endpoint_type_name(method: str, path: str, operation_id: str, seen: set[str]) -> str:
    base = f"{rust_ident(operation_id)}Endpoint"
    if base not in seen:
        seen.add(base)
        return base

    qualified = f"{rust_ident(method)}{rust_ident(path)}Endpoint"
    if qualified not in seen:
        seen.add(qualified)
        return qualified

    suffix = 2
    while f"{qualified}{suffix}" in seen:
        suffix += 1
    name = f"{qualified}{suffix}"
    seen.add(name)
    return name


def request_schema(operation: dict) -> str | None:
    content = (
        operation.get("requestBody", {})
        .get("content", {})
        .get("application/json", {})
        .get("schema", {})
    )
    ref = content.get("$ref")
    return schema_name(ref) if ref else None


def response_shape(operation: dict) -> tuple[str, str | None]:
    schema = (
        operation.get("responses", {})
        .get("200", {})
        .get("content", {})
        .get("application/json", {})
        .get("schema")
    )
    if schema is None:
        raise ValueError(f"operation {operation.get('operationId')} has no 200 JSON schema")
    if schema.get("type") == "null":
        return ("NoContent", None)

    all_of = schema.get("allOf") or []
    data_schema = None
    for item in all_of:
        data_schema = (item.get("properties") or {}).get("data")
        if data_schema is not None:
            break
    if data_schema is None:
        raise ValueError(f"operation {operation.get('operationId')} has no data schema")

    if data_schema.get("type") == "array":
        return ("Collection", schema_name(data_schema["items"]["$ref"]))

    ref = data_schema.get("$ref")
    if ref:
        return ("Item", schema_name(ref))

    raise ValueError(f"operation {operation.get('operationId')} has unsupported response schema")


def error_responses(operation: dict) -> list[dict]:
    result = []
    for status, response in operation.get("responses", {}).items():
        if status == "200":
            continue
        result.append(
            {
                "status": int(status),
                "description": response.get("description", ""),
            }
        )
    result.sort(key=lambda item: item["status"])
    return result


def parameter_location(parameter: dict) -> str:
    location = parameter.get("in")
    if location == "path":
        return "Path"
    if location == "query":
        return "Query"
    raise ValueError(
        f"operation {parameter.get('operationId')} has unsupported parameter location {location}"
    )


def parameter_value(schema: dict) -> str:
    kind, nullable = openapi_type(schema)
    if nullable:
        raise ValueError(f"nullable endpoint parameters are unsupported: {schema}")
    if kind == "integer":
        return "Integer"
    if kind == "number":
        return "Number"
    if kind == "boolean":
        return "Boolean"
    if kind == "string":
        return "String"
    raise ValueError(f"unsupported endpoint parameter schema: {schema}")


def endpoint_parameters(operation: dict) -> list[dict]:
    result = []
    for parameter in operation.get("parameters") or []:
        result.append(
            {
                "name": parameter["name"],
                "location": parameter_location(parameter),
                "required": bool(parameter.get("required", False)),
                "value": parameter_value(parameter.get("schema", {})),
            }
        )
    return result


def collect_operations(spec: dict) -> list[dict]:
    operations = []
    seen_names = set()
    for path, methods in spec["paths"].items():
        for method, operation in methods.items():
            operation_id = operation.get("operationId")
            if not operation_id:
                continue
            response_kind, response_schema = response_shape(operation)
            request = request_schema(operation)
            operations.append(
                {
                    "method": method.upper(),
                    "path": path,
                    "operation_id": operation_id,
                    "tag": (operation.get("tags") or [""])[0],
                    "endpoint_name": endpoint_type_name(method, path, operation_id, seen_names),
                    "parameters": endpoint_parameters(operation),
                    "request": request,
                    "response_kind": response_kind,
                    "response_schema": response_schema,
                    "errors": error_responses(operation),
                }
            )
    operations.sort(key=lambda item: (item["path"], item["method"], item["operation_id"]))
    return operations


def generate_wire(spec: dict) -> None:
    lines = [
        "//! Strongly typed Rentman API request and response models.",
        "//!",
        "//! This file is generated by `scripts/generate_endpoint_manifest.py`.",
        "",
        "#![allow(clippy::module_name_repetitions)]",
        "",
        "use chrono::{DateTime, FixedOffset};",
        "use rust_decimal::Decimal;",
        "use serde::{Deserialize, Serialize};",
        "use std::{collections::BTreeMap, fmt};",
        "use url::Url;",
        "",
        "/// Marker type for endpoints without a JSON request body.",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]",
        "pub struct NoRequest;",
        "",
        "/// Marker type for endpoints without a JSON response body.",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]",
        "pub struct NoContent;",
        "",
        "/// Rentman collection response wrapper.",
        "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
        "pub struct CollectionResponse<T> {",
        "    /// Returned collection items.",
        "    pub data: Vec<T>,",
        "    /// Number of items in this response.",
        "    #[serde(rename = \"itemCount\")]",
        "    pub item_count: usize,",
        "    /// Maximum number of items in this response.",
        "    pub limit: usize,",
        "    /// Number of skipped items when offset pagination is used.",
        "    #[serde(default)]",
        "    pub offset: Option<usize>,",
        "    /// URL for the next page when cursor pagination is used.",
        "    #[serde(default)]",
        "    pub next_page_url: Option<Url>,",
        "}",
        "",
        "/// Rentman item response wrapper.",
        "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
        "pub struct ItemResponse<T> {",
        "    /// Returned item.",
        "    pub data: T,",
        "    /// Number of items in this response.",
        "    #[serde(rename = \"itemCount\")]",
        "    #[serde(default)]",
        "    pub item_count: Option<usize>,",
        "    /// Maximum number of items in this response.",
        "    #[serde(default)]",
        "    pub limit: Option<usize>,",
        "    /// Number of skipped items when offset pagination is used.",
        "    #[serde(default)]",
        "    pub offset: Option<usize>,",
        "    /// URL for the next page when cursor pagination is used.",
        "    #[serde(default)]",
        "    pub next_page_url: Option<Url>,",
        "}",
        "",
        "/// Rentman integer identifier.",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]",
        "#[serde(transparent)]",
        "pub struct RentmanId(pub u64);",
        "",
        "/// Link to another Rentman resource, for example `/invoices/14`.",
        "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]",
        "#[serde(transparent)]",
        "pub struct ResourceReference(String);",
        "",
        "impl ResourceReference {",
        "    /// Returns the raw Rentman resource path.",
        "    pub fn as_str(&self) -> &str {",
        "        &self.0",
        "    }",
        "}",
        "",
        "impl<'de> Deserialize<'de> for ResourceReference {",
        "    // Coverage is disabled because serde monomorphizes this adapter per",
        "    // deserializer; behavior is covered through model deserialization tests.",
        "    #[cfg_attr(coverage_nightly, coverage(off))]",
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        "        let value = String::deserialize(deserializer)?;",
        "        if !value.starts_with('/') {",
        "            return Err(serde::de::Error::custom(\"Rentman resource reference must start with '/'\"));",
        "        }",
        "        Ok(Self(value))",
        "    }",
        "}",
        "",
        "/// Email address string from Rentman. Empty when the address is not set.",
        "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]",
        "#[serde(transparent)]",
        "pub struct EmailAddress(String);",
        "",
        "impl EmailAddress {",
        "    /// Returns the email address as a string slice.",
        "    pub fn as_str(&self) -> &str {",
        "        &self.0",
        "    }",
        "}",
        "",
        "impl<'de> Deserialize<'de> for EmailAddress {",
        "    // Coverage is disabled because serde monomorphizes this adapter per",
        "    // deserializer; behavior is covered through model deserialization tests.",
        "    #[cfg_attr(coverage_nightly, coverage(off))]",
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        "        let value = String::deserialize(deserializer)?;",
        "        // Live payloads use an empty string when no address is set.",
        "        if !value.is_empty() && !value.contains('@') {",
        "            return Err(serde::de::Error::custom(\"Rentman email address must contain '@'\"));",
        "        }",
        "        Ok(Self(value))",
        "    }",
        "}",
        "",
        "/// Explicitly open custom fields object.",
        "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
        "#[serde(transparent)]",
        "pub struct CustomFields(pub BTreeMap<String, CustomFieldValue>);",
        "",
        "/// Value inside Rentman's custom fields object.",
        "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
        "#[serde(untagged)]",
        "pub enum CustomFieldValue {",
        "    /// Null custom field value.",
        "    Null,",
        "    /// Boolean custom field value.",
        "    Bool(bool),",
        "    /// Numeric custom field value.",
        "    Number(Decimal),",
        "    /// Text custom field value.",
        "    Text(String),",
        "    /// List custom field value.",
        "    List(Vec<CustomFieldValue>),",
        "    /// Object custom field value.",
        "    Object(BTreeMap<String, CustomFieldValue>),",
        "}",
        "",
        "impl fmt::Display for RentmanId {",
        "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {",
        "        write!(f, \"{}\", self.0)",
        "    }",
        "}",
        "",
    ]

    enum_definitions: list[str] = []
    schemas = spec["components"]["schemas"]
    for schema_name_value, schema in schemas.items():
        required = set(schema.get("required") or [])
        for field, field_schema in (schema.get("properties") or {}).items():
            if "enum" not in field_schema:
                continue
            enum_name = enum_type_name(schema_name_value, field)
            used_variants: set[str] = set()
            enum_definitions.extend(
                [
                    f"/// Values documented for `{schema_name_value}.{field}`.",
                    "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]",
                    f"pub enum {enum_name} {{",
                ]
            )
            for value in field_schema["enum"]:
                variant = enum_variant_name(value, used_variants)
                enum_definitions.append(f"    /// `{escape(str(value))}`.")
                enum_definitions.append(f"    #[serde(rename = \"{escape(str(value))}\")]")
                enum_definitions.append(f"    {variant},")
            enum_definitions.append("}")
            enum_definitions.append("")

    lines.extend(enum_definitions)

    for schema_name_value, schema in schemas.items():
        description = rust_doc(
            schema.get("description"),
            f"Rentman API schema `{schema_name_value}`.",
        )
        lines.append(f"/// {description}")
        lines.append("#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]")
        lines.append(f"pub struct {schema_name_value} {{")
        required = effective_required(schema_name_value, schema)
        for field, field_schema in (schema.get("properties") or {}).items():
            rust_name = rust_field(field)
            field_type = rust_field_type(schema_name_value, field, field_schema, required)
            description = rust_doc(
                field_schema.get("description"),
                f"Rentman API field `{schema_name_value}.{field}`.",
            )
            lines.append(f"    /// {description}")
            serde_attributes = []
            if rust_name != field:
                serde_attributes.append(f'rename = "{escape(field)}"')
            if field not in required:
                serde_attributes.append("default")
                serde_attributes.append("skip_serializing_if = \"Option::is_none\"")
            if serde_attributes:
                lines.append(f"    #[serde({', '.join(serde_attributes)})]")
            lines.append(f"    pub {rust_name}: {field_type},")
        lines.append("}")
        lines.append("")

    lines.append("/// All Rentman API component schemas represented by this module.")
    lines.append("pub const ALL_SCHEMAS: &[&str] = &[")
    for name in sorted(schemas):
        lines.append(f'    "{escape(name)}",')
    lines.append("];")
    lines.append("")

    WIRE_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    WIRE_OUTPUT.write_text("\n".join(lines))


def generate_endpoints(operations: list[dict]) -> None:
    lines = [
        "use super::{Endpoint, EndpointErrorSpec, EndpointParameterLocation, EndpointParameterSpec, EndpointParameterValueSpec, EndpointRequestSpec, EndpointResponseSpec, EndpointSpec, EndpointWithId, ExecutableEndpoint};",
        "use crate::model::*;",
        "use crate::model::{CollectionResponse, ItemResponse, NoContent, NoRequest};",
        "",
    ]

    for operation in operations:
        parameter_const_name = (
            f"{re.sub(r'[^A-Za-z0-9]', '_', operation['endpoint_name']).upper()}_PARAMETERS"
        )
        lines.append(f"const {parameter_const_name}: &[EndpointParameterSpec] = &[")
        for parameter in operation["parameters"]:
            lines.append("    EndpointParameterSpec {")
            lines.append(f'        name: "{escape(parameter["name"])}",')
            lines.append(
                f"        location: EndpointParameterLocation::{parameter['location']},"
            )
            lines.append(f"        required: {str(parameter['required']).lower()},")
            lines.append(f"        value: EndpointParameterValueSpec::{parameter['value']},")
            lines.append("    },")
        lines.append("];")
        lines.append("")

        const_name = f"{re.sub(r'[^A-Za-z0-9]', '_', operation['endpoint_name']).upper()}_ERRORS"
        lines.append(f"const {const_name}: &[EndpointErrorSpec] = &[")
        for error in operation["errors"]:
            lines.append(
                "    EndpointErrorSpec { "
                f"status: {error['status']}, "
                f'description: "{escape(error["description"])}" '
                "},"
            )
        lines.append("];")
        lines.append("")

        request_type = operation["request"] or "NoRequest"
        if operation["response_kind"] == "NoContent":
            response_type = "NoContent"
            response_spec = "EndpointResponseSpec::NoContent"
        elif operation["response_kind"] == "Collection":
            response_type = f"CollectionResponse<{operation['response_schema']}>"
            response_spec = (
                f'EndpointResponseSpec::Collection("{escape(operation["response_schema"])}")'
            )
        else:
            response_type = f"ItemResponse<{operation['response_schema']}>"
            response_spec = f'EndpointResponseSpec::Item("{escape(operation["response_schema"])}")'

        request_spec = (
            "EndpointRequestSpec::None"
            if operation["request"] is None
            else f'EndpointRequestSpec::Json("{escape(operation["request"])}")'
        )

        lines.extend(
            [
                f"/// Typed endpoint for `{operation['method']} {operation['path']}`.",
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
                f"pub struct {operation['endpoint_name']};",
                "",
                f"impl Endpoint for {operation['endpoint_name']} {{",
                f"    type Request = {request_type};",
                f"    type Response = {response_type};",
                "",
                "    const SPEC: EndpointSpec = EndpointSpec {",
                f'        method: "{escape(operation["method"])}",',
                f'        path: "{escape(operation["path"])}",',
                f'        operation_id: "{escape(operation["operation_id"])}",',
                f'        tag: "{escape(operation["tag"])}",',
                f"        parameters: {parameter_const_name},",
                f"        request: {request_spec},",
                f"        response: {response_spec},",
                f"        errors: {const_name},",
                "    };",
                "}",
                "",
            ]
        )
        if any(
            parameter["name"] == "id"
            and parameter["location"] == "Path"
            and parameter["value"] == "Integer"
            for parameter in operation["parameters"]
        ):
            lines.append(f"impl EndpointWithId for {operation['endpoint_name']} {{}}")
            lines.append("")

    lines.append("/// All operations in the checked-in Rentman API document.")
    lines.append("pub const ALL_ENDPOINTS: &[EndpointSpec] = &[")
    for operation in operations:
        lines.append(f"    <{operation['endpoint_name']} as Endpoint>::SPEC,")
    lines.append("];")
    lines.append("")
    lines.append("#[allow(dead_code)]")
    lines.append("// Coverage is disabled because this compile-time assertion has no runtime")
    lines.append("// behavior; contract tests cover the generated endpoint inventory.")
    lines.append("#[cfg_attr(coverage_nightly, coverage(off))]")
    lines.append("pub(crate) fn assert_all_endpoints_are_executable()")
    lines.append("where")
    for operation in operations:
        lines.append(f"    {operation['endpoint_name']}: ExecutableEndpoint,")
    lines.append("{")
    lines.append("}")
    lines.append("")

    ENDPOINT_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    ENDPOINT_OUTPUT.write_text("\n".join(lines))


def generate_live_sweep(operations: list[dict]) -> None:
    sweep = [
        operation
        for operation in operations
        if operation["method"] == "GET"
        and operation["response_kind"] == "Collection"
        and not any(
            parameter["location"] == "Path" for parameter in operation["parameters"]
        )
    ]
    lines = [
        "//! One live test per documented `GET` collection endpoint without path",
        "//! parameters: each calls the endpoint with `limit=1` and decodes the",
        "//! response through the typed models.",
        "//!",
        "//! This file is generated by `scripts/generate_endpoint_manifest.py`.",
        "",
        "use super::live_client;",
        "use rentman_client::endpoint::{self, ExecutableEndpoint};",
        "use serde::{Serialize, de::DeserializeOwned};",
        "",
        "async fn check<E>()",
        "where",
        "    E: ExecutableEndpoint,",
        "    E::Request: Serialize,",
        "    E::Response: DeserializeOwned,",
        "{",
        "    if let Err(error) = live_client()",
        "        .endpoint::<E>()",
        '        .query_param("limit", 1)',
        "        .send()",
        "        .await",
        "    {",
        '        panic!("{} failed against the live API: {error:#}", E::SPEC.operation_id);',
        "    }",
        "}",
        "",
    ]
    for operation in sweep:
        test_name = rust_field(operation["operation_id"])
        lines.extend(
            [
                "#[tokio::test]",
                '#[ignore = "calls the real Rentman API; requires RENTMAN_API_TOKEN"]',
                f"async fn {test_name}() {{",
                f"    check::<endpoint::{operation['endpoint_name']}>().await;",
                "}",
                "",
            ]
        )
    LIVE_SWEEP_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    LIVE_SWEEP_OUTPUT.write_text("\n".join(lines))


def main() -> None:
    spec = json.loads(OPENAPI.read_text())
    generate_wire(spec)
    operations = collect_operations(spec)
    generate_endpoints(operations)
    generate_live_sweep(operations)
    subprocess.run(["cargo", "fmt", "--all"], cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
