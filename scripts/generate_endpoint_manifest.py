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
RESPONSE_ENUM_TESTS_OUTPUT = ROOT / "tests" / "contract" / "generated_response_enums.rs"
PAYLOAD_TESTS_OUTPUT = ROOT / "tests" / "contract" / "generated_payload_constructors.rs"

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
# Integer fields documented as resource IDs other than an item's own `id`, with
# their Rust type.
ID_FIELDS = {
    ("ProjectsCreateFromTemplateRequest", "template"): "Id<ProjectTemplate>",
    ("ProjectsSetClientRequest", "client"): "Id<Contact>",
    ("ProjectsSetClientRequest", "contact_person"): "Id<ContactPerson>",
    ("ProjectsSetLocationRequest", "location"): "Id<Contact>",
    ("ProjectsSetLocationRequest", "contact_person"): "Id<ContactPerson>",
    ("SubprojectsSetLocationRequest", "location"): "Id<Contact>",
    ("SubprojectsSetLocationRequest", "contact_person"): "Id<ContactPerson>",
    ("ProjectsSetTypeRequest", "type"): "Id<ProjectType>",
    # Polymorphic link: `itemtype` names the resource, `item` holds its ID.
    **{
        (f"{resource}{direction}", "item"): "ItemId"
        for resource in ("File", "FileFolder", "InvoiceLine", "Task")
        for direction in ("Request", "Response")
    },
}

# Collection path segment -> Rust item type, filled from the documented
# `GET /{collection}/{id}` operations by `collect_resources`.
RESOURCE_COLLECTIONS: dict[str, str] = {}
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
    "project_services_price",
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


# Rust type names for schemas whose name is not "resource + Response/Request".
# Every other schema drops `Response` (`CrewResponse` -> `Crew`) or replaces
# `Request` with `Payload` (`CrewRequest` -> `CrewPayload`). Names follow the
# collection the schema belongs to.
RESOURCE_NAMES = {
    "Factuur": "Invoice",  # /invoices
    "Invitations": "Invitation",  # /invitations
    "Factors": "Factor",  # /factors
    "FactorGroups": "FactorGroup",  # /factorgroups
    "LeaveTypes": "LeaveType",  # /leavetypes
    "LeaveMutations": "LeaveMutation",  # /leavemutation
    "EquipmentAssignedSerials": "EquipmentAssignedSerial",  # /equipmentassignedserials
    "CrewRate": "Rate",  # /rates
    "CrewRates": "CrewRate",  # /crewrates
    "CrewRateFactor": "RateFactor",  # /ratefactors
    "Quotation": "Quote",  # /quotes
    "Ledger": "LedgerCode",  # /ledgercodes
}
# Schemas that are not a resource or a payload keep their own name.
TYPE_NAME_OVERRIDES = {
    # Paging metadata of every response envelope, not a resource.
    "ApiResponse": "ApiResponse",
}


def rust_type_name(schema_name_value: str) -> str:
    """Rust type for an OpenAPI schema: the resource name for responses and
    the resource name plus `Payload` for request bodies."""
    if schema_name_value in TYPE_NAME_OVERRIDES:
        return TYPE_NAME_OVERRIDES[schema_name_value]
    for suffix, replacement in (("Response", ""), ("Request", "Payload")):
        if schema_name_value.endswith(suffix):
            resource = schema_name_value[: -len(suffix)]
            return RESOURCE_NAMES.get(resource, resource) + replacement
    raise ValueError(f"schema {schema_name_value} is neither a Response nor a Request")


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
    return f"{rust_type_name(schema)}{rust_ident(field)}"


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


def collect_resources(spec: dict) -> None:
    for path, methods in spec["paths"].items():
        match = re.fullmatch(r"/([a-z_]+)/\{id\}", path)
        if match is None or "get" not in methods:
            continue
        kind, schema = response_shape(methods["get"])
        if kind == "Item":
            RESOURCE_COLLECTIONS[match.group(1)] = rust_type_name(schema)
    types = list(RESOURCE_COLLECTIONS.values())
    if len(types) != len(set(types)):
        raise ValueError(f"a type serves several collections: {RESOURCE_COLLECTIONS}")


def resource_for_collection(collection: str, context: str) -> str:
    try:
        return RESOURCE_COLLECTIONS[collection]
    except KeyError:
        raise ValueError(f"{context} links to undocumented collection /{collection}") from None


def link_target(schema_name_value: str, field: str, schema: dict) -> str:
    """Rust item type a link field points to, from its documented example
    such as `/crew/0`."""
    example = schema.get("example")
    match = re.fullmatch(r"/([a-z_]+)/\d+", example) if isinstance(example, str) else None
    if match is None:
        raise ValueError(
            f"link field {schema_name_value}.{field} has no /{{collection}}/{{id}} example: {example!r}"
        )
    return resource_for_collection(match.group(1), f"{schema_name_value}.{field}")


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
        if (schema_name_value, field) in ID_FIELDS:
            return ID_FIELDS[(schema_name_value, field)]
        if field == "id":
            own = rust_type_name(schema_name_value)
            if own not in RESOURCE_COLLECTIONS.values():
                raise ValueError(f"{schema_name_value}.id belongs to no documented collection")
            return f"Id<{own}>"
        # Integer IDs other than `id` carry no example that names their
        # collection, so each one needs a deliberate ID_FIELDS entry.
        if field.endswith("_id") or re.match(r"(Numeric )?ID of", schema.get("description") or ""):
            raise ValueError(f"{schema_name_value}.{field} needs an ID_FIELDS entry")
        return "i64"
    if kind == "number":
        return "Decimal"
    if kind == "boolean":
        return "bool"
    if kind == "object":
        return "CustomFields"
    if kind == "string":
        if schema.get("format") == "date-time" or field in DATE_TIME_FIELDS:
            return "DateTime<FixedOffset>"
        if schema.get("format") == "date":
            return "NaiveDate"
        if schema.get("format") == "email":
            return "EmailAddress"
        if is_reference_field(field, schema):
            return f"Id<{link_target(schema_name_value, field, schema)}>"
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
        "use crate::id::{Id, ItemId, Resource};",
        "use chrono::{DateTime, FixedOffset, NaiveDate};",
        "use rust_decimal::Decimal;",
        "use serde::{Deserialize, Serialize};",
        "use std::collections::BTreeMap;",
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
            # Response enums are open: a value Rentman adds later must not fail
            # the whole collection. Request enums stay closed, because the
            # client only sends documented values.
            is_open = schema_name_value.endswith("Response")
            used_variants: set[str] = {"Unknown"} if is_open else set()
            derives = "Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize"
            if not is_open:
                derives = "Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize"
            enum_definitions.extend(
                [
                    f"/// Values documented for `{schema_name_value}.{field}`.",
                    f"#[derive({derives})]",
                    f"pub enum {enum_name} {{",
                ]
            )
            for value in field_schema["enum"]:
                variant = enum_variant_name(value, used_variants)
                enum_definitions.append(f"    /// `{escape(str(value))}`.")
                enum_definitions.append(f"    #[serde(rename = \"{escape(str(value))}\")]")
                enum_definitions.append(f"    {variant},")
            if is_open:
                enum_definitions.extend(
                    [
                        "    /// A value Rentman does not document, kept exactly as sent.",
                        "    #[serde(untagged)]",
                        "    Unknown(String),",
                    ]
                )
            enum_definitions.append("}")
            enum_definitions.append("")

    lines.extend(enum_definitions)

    for schema_name_value, schema in schemas.items():
        description = rust_doc(
            schema.get("description"),
            f"Rentman API schema `{schema_name_value}`.",
        )
        required = effective_required(schema_name_value, schema)
        is_payload = schema_name_value.endswith("Request")
        derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
        if is_payload and not required:
            # Every field is optional: start from `Default` and set what changes.
            derives += ", Default"
        lines.append(f"/// {description}")
        lines.append(f"#[derive({derives})]")
        lines.append(f"pub struct {rust_type_name(schema_name_value)} {{")
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
            is_link = openapi_type(field_schema)[0] == "string" and "Id<" in field_type
            if field not in required:
                serde_attributes.append("default")
                serde_attributes.append("skip_serializing_if = \"Option::is_none\"")
            elif is_link and field_type.startswith("Option<"):
                # Like a plain `Option`, a nullable link may also be missing.
                serde_attributes.append("default")
            if is_link:
                adapter = "optional_link" if field_type.startswith("Option<") else "link"
                serde_attributes.append(f'with = "crate::id::{adapter}"')
            if serde_attributes:
                lines.append(f"    #[serde({', '.join(serde_attributes)})]")
            lines.append(f"    pub {rust_name}: {field_type},")
        lines.append("}")
        lines.append("")
        if is_payload and required:
            lines.extend(payload_constructor(schema_name_value, schema, required))

    type_names = [rust_type_name(name) for name in schemas]
    enum_names = [
        enum_type_name(name, field)
        for name, schema in schemas.items()
        for field, field_schema in (schema.get("properties") or {}).items()
        if "enum" in field_schema
    ]
    fixed_names = [
        "NoRequest", "NoContent", "CollectionResponse", "ItemResponse", "Id", "ItemId",
        "Resource", "EmailAddress", "CustomFields", "CustomFieldValue",
    ]
    alias_names = [f"{name}Id" for name in RESOURCE_COLLECTIONS.values()]
    all_names = type_names + enum_names + fixed_names + alias_names
    duplicates = sorted({name for name in all_names if all_names.count(name) > 1})
    if duplicates:
        raise ValueError(f"generated type names collide: {duplicates}")

    for collection, rust_type in sorted(RESOURCE_COLLECTIONS.items()):
        lines.extend(
            [
                f"impl Resource for {rust_type} {{",
                f'    const COLLECTION: &\'static str = "{collection}";',
                "}",
                "",
                f"/// ID of a [`{rust_type}`] item (`/{collection}/{{id}}`).",
                f"pub type {rust_type}Id = Id<{rust_type}>;",
                "",
            ]
        )

    lines.append("/// Every resource type and its collection path segment.")
    lines.append("pub const RESOURCE_COLLECTIONS: &[(&str, &str)] = &[")
    for collection, rust_type in sorted(RESOURCE_COLLECTIONS.items(), key=lambda item: item[1]):
        lines.append(f'    ("{rust_type}", "{collection}"),')
    lines.append("];")
    lines.append("")
    lines.append("/// Every link field (`schema`, `field`, linked collection).")
    lines.append("pub const LINK_FIELDS: &[(&str, &str, &str)] = &[")
    for name in sorted(schemas):
        for field, field_schema in (schemas[name].get("properties") or {}).items():
            field_type = base_rust_type(name, field, field_schema)
            if openapi_type(field_schema)[0] == "string" and field_type.startswith("Id<"):
                target = field_type[3:-1]
                collection = next(c for c, t in RESOURCE_COLLECTIONS.items() if t == target)
                lines.append(f'    ("{name}", "{escape(field)}", "{collection}"),')
    lines.append("];")
    lines.append("")
    lines.append("/// Every Rentman API component schema and the Rust type that represents it.")
    lines.append("pub const SCHEMA_TYPES: &[(&str, &str)] = &[")
    for name in sorted(schemas):
        lines.append(f'    ("{escape(name)}", "{rust_type_name(name)}"),')
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

        request_type = rust_type_name(operation["request"]) if operation["request"] else "NoRequest"
        if operation["response_kind"] == "NoContent":
            response_type = "NoContent"
            response_spec = "EndpointResponseSpec::NoContent"
        elif operation["response_kind"] == "Collection":
            response_type = f"CollectionResponse<{rust_type_name(operation['response_schema'])}>"
            response_spec = (
                f'EndpointResponseSpec::Collection("{escape(operation["response_schema"])}")'
            )
        else:
            response_type = f"ItemResponse<{rust_type_name(operation['response_schema'])}>"
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
            collection = operation["path"].split("/")[1]
            resource = resource_for_collection(collection, operation["operation_id"])
            lines.append(f"impl EndpointWithId for {operation['endpoint_name']} {{")
            lines.append(f"    type Resource = {resource};")
            lines.append("}")
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


def payload_fields(schema_name_value: str, schema: dict, required: set[str]) -> list[tuple[str, str, bool]]:
    """(Rust field name, Rust type, required) of a payload, in schema order."""
    return [
        (rust_field(field), rust_field_type(schema_name_value, field, field_schema, required), field in required)
        for field, field_schema in (schema.get("properties") or {}).items()
    ]


def payload_constructor(schema_name_value: str, schema: dict, required: set[str]) -> list[str]:
    fields = payload_fields(schema_name_value, schema, required)
    parameters = ", ".join(f"{name}: {rust_type}" for name, rust_type, is_required in fields if is_required)
    lines = [
        f"impl {rust_type_name(schema_name_value)} {{",
        "    /// Creates the payload from its required fields; every optional field",
        "    /// is `None` and not sent.",
        f"    pub fn new({parameters}) -> Self {{",
        "        Self {",
    ]
    for name, _, is_required in fields:
        lines.append(f"            {name}," if is_required else f"            {name}: None,")
    lines.extend(["        }", "    }", "}", ""])
    return lines


def sample_value(rust_type: str, schema_name_value: str, field: str, field_schema: dict) -> str:
    """A valid Rust expression of the given field type for generated tests."""
    if rust_type.startswith("Option<"):
        return "None"
    if rust_type.startswith("Id<"):
        return "rentman_client::Id::new(1)"
    if "enum" in field_schema:
        variant = enum_variant_name(field_schema["enum"][0], {"Unknown"} if schema_name_value.endswith("Response") else set())
        return f"model::{rust_type}::{variant}"
    samples = {
        "String": 'String::from("x")',
        "DateTime<FixedOffset>": 'chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00+00:00").unwrap()',
        "NaiveDate": "chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()",
        "i64": "1",
        "bool": "true",
        "Decimal": "rust_decimal::Decimal::ONE",
        "ItemId": "rentman_client::ItemId::new(1)",
    }
    if rust_type not in samples:
        raise ValueError(f"no sample value for {schema_name_value}.{field}: {rust_type}")
    return samples[rust_type]


def generate_payload_constructor_tests(spec: dict) -> None:
    lines = [
        "//! One test per payload constructor: `new` sets exactly the required",
        "//! fields, and only they are sent.",
        "//!",
        "//! This file is generated by `scripts/generate_endpoint_manifest.py`.",
        "",
        "use super::assert_sends_exactly;",
        "use rentman_client::model;",
        "",
    ]
    for schema_name_value, schema in spec["components"]["schemas"].items():
        required = effective_required(schema_name_value, schema)
        if not schema_name_value.endswith("Request") or not required:
            continue
        properties = schema.get("properties") or {}
        arguments = ", ".join(
            sample_value(
                rust_field_type(schema_name_value, field, properties[field], required),
                schema_name_value,
                field,
                properties[field],
            )
            for field in properties
            if field in required
        )
        sent = ", ".join(f'"{escape(field)}"' for field in properties if field in required)
        lines.extend(
            [
                "#[test]",
                f"fn {rust_field(rust_type_name(schema_name_value))}() {{",
                f"    let payload = model::{rust_type_name(schema_name_value)}::new({arguments});",
                f'    assert_sends_exactly(&payload, "{schema_name_value}", &[{sent}]);',
                "}",
                "",
            ]
        )
    PAYLOAD_TESTS_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    PAYLOAD_TESTS_OUTPUT.write_text("\n".join(lines))


def generate_response_enum_tests(spec: dict) -> None:
    fields = [
        (schema_name_value, field)
        for schema_name_value, schema in spec["components"]["schemas"].items()
        if schema_name_value.endswith("Response")
        for field, field_schema in (schema.get("properties") or {}).items()
        if "enum" in field_schema
    ]
    lines = [
        "//! One contract test per open response enum: every value the OpenAPI",
        "//! document lists must deserialize to a named variant, not to `Unknown`.",
        "//!",
        "//! This file is generated by `scripts/generate_endpoint_manifest.py`.",
        "",
        "use super::assert_documented_values_are_named;",
        "use rentman_client::model;",
        "",
        "/// Response schema fields that have a generated open enum.",
        "pub(crate) const RESPONSE_ENUM_FIELDS: &[(&str, &str)] = &[",
    ]
    for schema_name_value, field in fields:
        lines.append(f'    ("{escape(schema_name_value)}", "{escape(field)}"),')
    lines.extend(["];", ""])
    for schema_name_value, field in fields:
        enum_name = enum_type_name(schema_name_value, field)
        test_name = rust_field(f"{schema_name_value}_{field}").rstrip("_")
        lines.extend(
            [
                "#[test]",
                f"fn {test_name}() {{",
                f"    assert_documented_values_are_named::<model::{enum_name}>(",
                f'        "{escape(schema_name_value)}",',
                f'        "{escape(field)}",',
                f"        |value| matches!(value, model::{enum_name}::Unknown(_)),",
                "    );",
                "}",
                "",
            ]
        )
    RESPONSE_ENUM_TESTS_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    RESPONSE_ENUM_TESTS_OUTPUT.write_text("\n".join(lines))


def main() -> None:
    spec = json.loads(OPENAPI.read_text())
    collect_resources(spec)
    generate_wire(spec)
    operations = collect_operations(spec)
    generate_endpoints(operations)
    generate_live_sweep(operations)
    generate_response_enum_tests(spec)
    generate_payload_constructor_tests(spec)
    subprocess.run(["cargo", "fmt", "--all"], cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
