use regex::Regex;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct GeneratorConfig {
    pub revision_field: String,
    pub response_core_fields: Vec<String>,
    pub response_string_integer_fields: Vec<String>,
    pub date_time_fields: Vec<String>,
    pub url_fields: Vec<String>,
    pub reference_fields: Vec<String>,
    pub filterable_types: Vec<String>,
    pub uncountable_types: Vec<String>,
    pub resource_names: BTreeMap<String, String>,
    pub schema_type_overrides: BTreeMap<String, String>,
    pub enum_variant_names: BTreeMap<String, String>,
    pub response_type_overrides: Vec<FieldOverride>,
    pub nullable_response_fields: Vec<SchemaField>,
    pub integer_id_fields: Vec<FieldOverride>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaNamesConfig {
    resource_names: BTreeMap<String, String>,
    schema_type_overrides: BTreeMap<String, String>,
    enum_variant_names: BTreeMap<String, String>,
    integer_id_fields: Vec<FieldOverride>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResponseOverridesConfig {
    revision_field: String,
    response_core_fields: Vec<String>,
    response_string_integer_fields: Vec<String>,
    response_type_overrides: Vec<FieldOverride>,
    nullable_response_fields: Vec<SchemaField>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldRulesConfig {
    date_time_fields: Vec<String>,
    url_fields: Vec<String>,
    reference_fields: Vec<String>,
    filterable_types: Vec<String>,
    uncountable_types: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldOverride {
    pub schema: String,
    pub field: String,
    pub rust_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaField {
    pub schema: String,
    pub field: String,
}

/// The generator config embedded from `xtask/*.toml`; a unit test proves it parses.
pub fn config() -> &'static GeneratorConfig {
    static CONFIG: OnceLock<GeneratorConfig> = OnceLock::new();
    CONFIG.get_or_init(|| {
        parse_config(
            include_str!("../schema-names.toml"),
            include_str!("../response-overrides.toml"),
            include_str!("../field-rules.toml"),
        )
        .expect("embedded generator config is valid")
    })
}

pub fn parse_config(
    schema_names: &str,
    response_overrides: &str,
    field_rules: &str,
) -> Result<GeneratorConfig, String> {
    let schemas: SchemaNamesConfig = toml::from_str(schema_names)
        .map_err(|error| format!("invalid xtask/schema-names.toml: {error}"))?;
    let responses: ResponseOverridesConfig = toml::from_str(response_overrides)
        .map_err(|error| format!("invalid xtask/response-overrides.toml: {error}"))?;
    let fields: FieldRulesConfig = toml::from_str(field_rules)
        .map_err(|error| format!("invalid xtask/field-rules.toml: {error}"))?;
    Ok(GeneratorConfig {
        revision_field: responses.revision_field,
        response_core_fields: responses.response_core_fields,
        response_string_integer_fields: responses.response_string_integer_fields,
        date_time_fields: fields.date_time_fields,
        url_fields: fields.url_fields,
        reference_fields: fields.reference_fields,
        filterable_types: fields.filterable_types,
        uncountable_types: fields.uncountable_types,
        resource_names: schemas.resource_names,
        schema_type_overrides: schemas.schema_type_overrides,
        enum_variant_names: schemas.enum_variant_names,
        response_type_overrides: responses.response_type_overrides,
        nullable_response_fields: responses.nullable_response_fields,
        integer_id_fields: schemas.integer_id_fields,
    })
}

pub fn object(value: &Value) -> Result<&Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| "expected a JSON object".to_owned())
}

pub fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("expected string field {key}"))
}

pub fn optional_string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

/// Properties of a component schema. `Context::with_config` rejects schemas
/// whose `properties` is not an object, so a missing one is the only empty case.
pub fn properties(schema: &Value) -> &Map<String, Value> {
    schema
        .get("properties")
        .and_then(Value::as_object)
        .unwrap_or(empty_object())
}

fn empty_object() -> &'static Map<String, Value> {
    static EMPTY: OnceLock<Map<String, Value>> = OnceLock::new();
    EMPTY.get_or_init(Map::new)
}

pub fn required(schema: &Value) -> BTreeSet<String> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

pub fn schema_names(spec: &Value) -> Result<&Map<String, Value>, String> {
    spec.get("components")
        .and_then(|value| value.get("schemas"))
        .ok_or_else(|| "OpenAPI document has no components.schemas".to_owned())
        .and_then(object)
}

pub fn rust_type_name(name: &str) -> Result<String, String> {
    let config = config();
    if let Some(mapped) = config.schema_type_overrides.get(name) {
        return Ok(mapped.clone());
    }
    for (suffix, replacement) in [("Response", ""), ("Request", "Payload")] {
        if let Some(resource) = name.strip_suffix(suffix) {
            let mapped = config
                .resource_names
                .get(resource)
                .map_or(resource, String::as_str);
            return Ok(format!("{mapped}{replacement}"));
        }
    }
    Err(format!("schema {name} is neither a Response nor a Request"))
}

pub fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn rust_ident(value: &str) -> String {
    let mut result = String::new();
    for part in Regex::new(r"[^A-Za-z0-9]+")
        .expect("static regex")
        .split(value)
        .filter(|part| !part.is_empty())
    {
        let mut chars = part.chars();
        result.extend(chars.next().into_iter().flat_map(char::to_uppercase));
        result.extend(chars);
    }
    if result.is_empty() {
        result.push_str("Value");
    }
    if result.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        result.insert_str(0, "Value");
    }
    result
}

fn validate_config(
    schemas: &Map<String, Value>,
    config: &GeneratorConfig,
    resources: &BTreeMap<String, String>,
) -> Result<(), String> {
    for source in config.resource_names.keys() {
        if !schemas.contains_key(&format!("{source}Response")) {
            return Err(format!(
                "schema-names.toml maps unknown resource schema {source}Response"
            ));
        }
    }
    for schema in config.schema_type_overrides.keys() {
        if !schemas.contains_key(schema) {
            return Err(format!(
                "schema-names.toml overrides unknown schema {schema}"
            ));
        }
    }
    for override_ in config
        .response_type_overrides
        .iter()
        .chain(&config.integer_id_fields)
    {
        let fields = schemas
            .get(&override_.schema)
            .ok_or_else(|| {
                format!(
                    "generator config references unknown schema {}",
                    override_.schema
                )
            })
            .map(properties)?;
        if !fields.contains_key(&override_.field) {
            return Err(format!(
                "generator config references unknown field {}.{}",
                override_.schema, override_.field
            ));
        }
    }
    for override_ in &config.nullable_response_fields {
        let fields = schemas
            .get(&override_.schema)
            .ok_or_else(|| {
                format!(
                    "generator config references unknown schema {}",
                    override_.schema
                )
            })
            .map(properties)?;
        if !fields.contains_key(&override_.field) {
            return Err(format!(
                "generator config references unknown nullable field {}.{}",
                override_.schema, override_.field
            ));
        }
    }
    for resource in &config.uncountable_types {
        if !resources.values().any(|known| known == resource) {
            return Err(format!(
                "field-rules.toml marks unknown resource {resource} uncountable"
            ));
        }
    }
    Ok(())
}

pub fn rust_field(value: &str) -> String {
    let mut field = value.to_owned();
    for pattern in [r"(.)([A-Z][a-z]+)", r"([a-z0-9])([A-Z])"] {
        field = Regex::new(pattern)
            .expect("static regex")
            .replace_all(&field, "${1}_${2}")
            .into_owned();
    }
    field = Regex::new(r"[^A-Za-z0-9_]+")
        .expect("static regex")
        .replace_all(&field, "_")
        .to_ascii_lowercase();
    field = Regex::new(r"_+")
        .expect("static regex")
        .replace_all(&field, "_")
        .into_owned();
    field = field.trim_matches('_').to_owned();
    if field.is_empty() {
        field.push_str("value");
    }
    if field.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        field.insert_str(0, "field_");
    }
    if is_keyword(&field) {
        field.push('_');
    }
    field
}

fn is_keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "async"
            | "await"
            | "break"
            | "crate"
            | "dyn"
            | "enum"
            | "fn"
            | "impl"
            | "in"
            | "let"
            | "match"
            | "mod"
            | "move"
            | "pub"
            | "ref"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "type"
            | "use"
            | "where"
    )
}

pub fn rust_doc(value: Option<&str>, fallback: &str) -> String {
    let text = value
        .unwrap_or(fallback)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    escape(if text.is_empty() { fallback } else { &text })
}

pub fn openapi_type(schema: &Value) -> Result<(Option<&str>, bool), String> {
    match schema.get("type") {
        Some(Value::String(kind)) => Ok((Some(kind), false)),
        Some(Value::Array(types)) => {
            let non_null = types
                .iter()
                .filter_map(Value::as_str)
                .filter(|kind| *kind != "null")
                .collect::<Vec<_>>();
            if non_null.len() != 1 {
                return Err(format!("unsupported union type: {types:?}"));
            }
            Ok((Some(non_null[0]), true))
        }
        _ => Ok((None, false)),
    }
}

pub fn enum_variant_name(value: &Value, used: &mut BTreeSet<String>) -> String {
    let text = value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_owned);
    let base = config()
        .enum_variant_names
        .get(&text)
        .cloned()
        .unwrap_or_else(|| rust_ident(&text));
    let base = if is_keyword(&base) {
        format!("{base}Value")
    } else {
        base
    };
    let mut variant = base.clone();
    let mut suffix = 2;
    while !used.insert(variant.clone()) {
        variant = format!("{base}{suffix}");
        suffix += 1;
    }
    variant
}

#[derive(Clone, Debug)]
struct FieldType {
    base: String,
    kind: Option<String>,
    nullable: bool,
}

/// A validated OpenAPI document. Building it checks every component schema
/// and field once, so the generators read names and types without failing.
#[derive(Clone, Debug)]
pub struct Context<'a> {
    pub spec: &'a Value,
    pub paths: &'a Map<String, Value>,
    pub schemas: &'a Map<String, Value>,
    pub resources: BTreeMap<String, String>,
    pub config: &'static GeneratorConfig,
    type_names: BTreeMap<String, String>,
    resource_schemas: BTreeMap<String, String>,
    fields: BTreeMap<(String, String), FieldType>,
}

impl<'a> Context<'a> {
    pub fn with_config(spec: &'a Value, config: &'static GeneratorConfig) -> Result<Self, String> {
        let paths = spec
            .get("paths")
            .ok_or_else(|| "OpenAPI document has no paths".to_owned())
            .and_then(object)?;
        let schemas = schema_names(spec)?;
        let mut type_names = BTreeMap::new();
        for (name, schema) in schemas {
            if schema
                .get("properties")
                .is_some_and(|value| !value.is_object())
            {
                return Err(format!("schema {name} has non-object properties"));
            }
            type_names.insert(name.clone(), rust_type_name(name)?);
        }
        let mut context = Self {
            spec,
            paths,
            schemas,
            resources: BTreeMap::new(),
            config,
            type_names,
            resource_schemas: BTreeMap::new(),
            fields: BTreeMap::new(),
        };
        let path_pattern = Regex::new(r"^/([a-z_]+)/\{id\}$").expect("static regex");
        for (path, methods) in paths {
            let Some(captures) = path_pattern.captures(path) else {
                continue;
            };
            let Some(operation) = methods.get("get") else {
                continue;
            };
            if let Response::Item(schema) = response_shape(operation)? {
                let name = context.referenced_type(&schema, path)?.to_owned();
                if !schema.ends_with("Response") {
                    return Err(format!("{path} returns {schema}, not a Response schema"));
                }
                context.resource_schemas.insert(name.clone(), schema);
                context.resources.insert(captures[1].to_owned(), name);
            }
        }
        let mut names = BTreeSet::new();
        for name in context.resources.values() {
            if !names.insert(name) {
                return Err(format!(
                    "a type serves several collections: {:?}",
                    context.resources
                ));
            }
        }
        validate_config(schemas, config, &context.resources)?;
        for (name, schema) in schemas {
            for (field, field_schema) in properties(schema) {
                let (kind, mut nullable) = openapi_type(field_schema)?;
                if config
                    .nullable_response_fields
                    .iter()
                    .any(|item| &item.schema == name && &item.field == field)
                {
                    nullable = true;
                }
                let field_type = FieldType {
                    base: context.base_rust_type(name, field, field_schema, kind)?,
                    kind: kind.map(str::to_owned),
                    nullable,
                };
                context
                    .fields
                    .insert((name.clone(), field.clone()), field_type);
            }
        }
        Ok(context)
    }

    /// Rust type of a component schema.
    pub fn type_name(&self, schema: &str) -> &str {
        &self.type_names[schema]
    }

    /// Rust type of a schema an operation references, which may be undefined.
    pub fn referenced_type(&self, schema: &str, context: &str) -> Result<&str, String> {
        self.type_names
            .get(schema)
            .map(String::as_str)
            .ok_or_else(|| format!("{context} references undefined schema {schema}"))
    }

    pub fn enum_type_name(&self, schema: &str, field: &str) -> String {
        format!("{}{}", self.type_name(schema), rust_ident(field))
    }

    /// Whether `schema` is the response schema of a documented collection.
    pub fn is_resource_schema(&self, schema: &str) -> bool {
        schema.ends_with("Response")
            && self
                .resources
                .values()
                .any(|resource| resource == self.type_name(schema))
    }

    /// The response schema that defines `resource`.
    pub fn resource_schema(&self, resource: &str) -> (&str, &Value) {
        let name = self.resource_schemas[resource].as_str();
        (name, &self.schemas[name])
    }

    pub fn resource_for_collection(&self, collection: &str, context: &str) -> Result<&str, String> {
        self.resources
            .get(collection)
            .map(String::as_str)
            .ok_or_else(|| format!("{context} links to undocumented collection /{collection}"))
    }

    pub fn link_target(
        &self,
        schema_name: &str,
        field: &str,
        schema: &Value,
    ) -> Result<String, String> {
        let example = schema
            .get("example")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!("link field {schema_name}.{field} has no /{{collection}}/{{id}} example")
            })?;
        let pattern = Regex::new(r"^/([a-z_]+)/\d+$").expect("static regex");
        let captures = pattern.captures(example).ok_or_else(|| {
            format!("link field {schema_name}.{field} has invalid example: {example:?}")
        })?;
        Ok(self
            .resource_for_collection(&captures[1], &format!("{schema_name}.{field}"))?
            .to_owned())
    }

    fn base_rust_type(
        &self,
        schema_name: &str,
        field: &str,
        schema: &Value,
        kind: Option<&str>,
    ) -> Result<String, String> {
        if schema_name.ends_with("Response") {
            if let Some(override_) = self
                .config
                .response_type_overrides
                .iter()
                .find(|item| item.schema == schema_name && item.field == field)
            {
                return Ok(override_.rust_type.clone());
            }
            if self
                .config
                .response_string_integer_fields
                .iter()
                .any(|configured| configured == field)
                && kind == Some("string")
            {
                return Ok("i64".to_owned());
            }
        }
        if schema.get("enum").is_some() {
            return Ok(self.enum_type_name(schema_name, field));
        }
        match kind {
            Some("integer") => {
                if let Some(override_) = self
                    .config
                    .integer_id_fields
                    .iter()
                    .find(|item| item.schema == schema_name && item.field == field)
                {
                    return Ok(override_.rust_type.clone());
                }
                if field == "id" {
                    let own = self.type_name(schema_name);
                    if !self.resources.values().any(|resource| resource == own) {
                        return Err(format!(
                            "{schema_name}.id belongs to no documented collection"
                        ));
                    }
                    return Ok(format!("Id<{own}>"));
                }
                let description = optional_string(schema, "description").unwrap_or_default();
                if field.ends_with("_id")
                    || Regex::new(r"^(Numeric )?ID of")
                        .expect("static regex")
                        .is_match(description)
                {
                    return Err(format!(
                        "{schema_name}.{field} needs an integer ID rule in xtask/schema-names.toml"
                    ));
                }
                Ok("i64".to_owned())
            }
            Some("number") => Ok("RentmanAmount".to_owned()),
            Some("boolean") => Ok("bool".to_owned()),
            Some("object") => Ok("CustomFields".to_owned()),
            Some("string") => {
                if optional_string(schema, "format") == Some("date-time")
                    || self
                        .config
                        .date_time_fields
                        .iter()
                        .any(|configured| configured == field)
                {
                    return Ok("DateTime<FixedOffset>".to_owned());
                }
                match optional_string(schema, "format") {
                    Some("date") => return Ok("NaiveDate".to_owned()),
                    Some("email") => return Ok("EmailAddress".to_owned()),
                    _ => {}
                }
                if is_reference_field(field, schema, &self.config.reference_fields) {
                    return Ok(format!(
                        "Id<{}>",
                        self.link_target(schema_name, field, schema)?
                    ));
                }
                if self
                    .config
                    .url_fields
                    .iter()
                    .any(|configured| configured == field)
                    || field.ends_with("_url")
                {
                    return Ok("Url".to_owned());
                }
                Ok("String".to_owned())
            }
            _ => Err(format!(
                "unsupported property type for {schema_name}.{field}: {schema}"
            )),
        }
    }

    /// Rust type of a component field without the `Option` for optional fields.
    pub fn base_type(&self, schema: &str, field: &str) -> &str {
        &self.field(schema, field).base
    }

    /// OpenAPI `type` of a component field.
    pub fn field_kind(&self, schema: &str, field: &str) -> Option<&str> {
        self.field(schema, field).kind.as_deref()
    }

    /// Rust type of a component field given the fields that count as required.
    pub fn field_type(&self, schema: &str, field: &str, required: &BTreeSet<String>) -> String {
        let field_type = self.field(schema, field);
        if field_type.nullable || !required.contains(field) {
            format!("Option<{}>", field_type.base)
        } else {
            field_type.base.clone()
        }
    }

    fn field(&self, schema: &str, field: &str) -> &FieldType {
        &self.fields[&(schema.to_owned(), field.to_owned())]
    }

    pub fn effective_required(&self, schema_name: &str, schema: &Value) -> BTreeSet<String> {
        if self.is_resource_schema(schema_name) {
            let fields = properties(schema);
            return self
                .config
                .response_core_fields
                .iter()
                .filter(|field| fields.contains_key(*field))
                .cloned()
                .collect();
        }
        required(schema)
    }
}

fn is_reference_field(field: &str, schema: &Value, configured: &[String]) -> bool {
    schema
        .get("example")
        .and_then(Value::as_str)
        .is_some_and(|example| example.starts_with('/'))
        || configured.iter().any(|configured| configured == field)
}

/// The documented 200 response of an operation, with its item schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    NoContent,
    Collection(String),
    Item(String),
}

impl Response {
    pub fn schema(&self) -> Option<&str> {
        match self {
            Self::NoContent => None,
            Self::Collection(schema) | Self::Item(schema) => Some(schema),
        }
    }
}

pub fn response_shape(operation: &Value) -> Result<Response, String> {
    let operation_id = optional_string(operation, "operationId").unwrap_or("unknown");
    let schema = operation
        .get("responses")
        .and_then(|v| v.get("200"))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.get("application/json"))
        .and_then(|v| v.get("schema"))
        .ok_or_else(|| format!("operation {operation_id} has no 200 JSON schema"))?;
    if schema.get("type").and_then(Value::as_str) == Some("null") {
        return Ok(Response::NoContent);
    }
    let all_of = schema
        .get("allOf")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("operation {operation_id} has no data schema"))?;
    let data = all_of
        .iter()
        .find_map(|item| item.get("properties").and_then(|p| p.get("data")))
        .ok_or_else(|| format!("operation {operation_id} has no data schema"))?;
    if data.get("type").and_then(Value::as_str) == Some("array") {
        let reference = data
            .get("items")
            .and_then(|item| item.get("$ref"))
            .ok_or_else(|| {
                format!("operation {operation_id} has an array without an item schema")
            })?;
        return Ok(Response::Collection(ref_name(reference)?));
    }
    if let Some(reference) = data.get("$ref") {
        return Ok(Response::Item(ref_name(reference)?));
    }
    Err(format!(
        "operation {operation_id} has unsupported response schema"
    ))
}

fn ref_name(reference: &Value) -> Result<String, String> {
    reference
        .as_str()
        .and_then(|reference| reference.rsplit('/').next())
        .map(str::to_owned)
        .ok_or_else(|| "invalid OpenAPI schema reference".to_owned())
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub method: String,
    pub path: String,
    pub operation_id: String,
    pub tag: String,
    pub endpoint_name: String,
    pub parameters: Vec<Parameter>,
    pub request: Option<String>,
    pub response: Response,
    pub errors: Vec<(u16, String)>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub value: String,
}

pub fn collect_operations(context: &Context<'_>) -> Result<Vec<Operation>, String> {
    let mut operations = Vec::new();
    let mut seen = BTreeSet::new();
    for (path, methods) in context.paths {
        for (method, operation) in object(methods)? {
            let Some(operation_id) = optional_string(operation, "operationId") else {
                continue;
            };
            let response = response_shape(operation)?;
            let request = operation
                .get("requestBody")
                .and_then(|v| v.get("content"))
                .and_then(|v| v.get("application/json"))
                .and_then(|v| v.get("schema"))
                .and_then(|v| v.get("$ref"))
                .map(ref_name)
                .transpose()?;
            let described = format!("operation {operation_id}");
            for schema in response.schema().into_iter().chain(request.as_deref()) {
                context.referenced_type(schema, &described)?;
            }
            let mut endpoint_name = format!("{}Endpoint", rust_ident(operation_id));
            if !seen.insert(endpoint_name.clone()) {
                endpoint_name = format!("{}{}Endpoint", rust_ident(method), rust_ident(path));
                let base = endpoint_name.clone();
                let mut suffix = 2;
                while !seen.insert(endpoint_name.clone()) {
                    endpoint_name = format!("{base}{suffix}");
                    suffix += 1;
                }
            }
            let mut parameters = Vec::new();
            if let Some(params) = operation.get("parameters").and_then(Value::as_array) {
                for parameter in params {
                    let location = match optional_string(parameter, "in") {
                        Some("path") => "Path",
                        Some("query") => "Query",
                        other => {
                            return Err(format!(
                                "operation {operation_id} has unsupported parameter location {other:?}"
                            ));
                        }
                    };
                    let schema = parameter.get("schema").unwrap_or(&Value::Null);
                    let (kind, nullable) = openapi_type(schema)?;
                    if nullable {
                        return Err(format!(
                            "nullable endpoint parameters are unsupported: {schema}"
                        ));
                    }
                    let value = match kind {
                        Some("integer") => "Integer",
                        Some("number") => "Number",
                        Some("boolean") => "Boolean",
                        Some("string") => "String",
                        _ => {
                            return Err(format!("unsupported endpoint parameter schema: {schema}"));
                        }
                    };
                    parameters.push(Parameter {
                        name: string(parameter, "name")?.to_owned(),
                        location: location.to_owned(),
                        required: parameter
                            .get("required")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                        value: value.to_owned(),
                    });
                }
            }
            let tag = operation
                .get("tags")
                .and_then(Value::as_array)
                .and_then(|tags| tags.first())
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();
            let mut errors = Vec::new();
            // `response_shape` found `responses.200`, so `responses` is an object.
            for (status, response) in operation["responses"].as_object().into_iter().flatten() {
                if status == "200" {
                    continue;
                }
                let status = status
                    .parse::<u16>()
                    .map_err(|_| format!("invalid response status {status:?}"))?;
                errors.push((
                    status,
                    optional_string(response, "description")
                        .unwrap_or("")
                        .to_owned(),
                ));
            }
            errors.sort_by_key(|error| error.0);
            operations.push(Operation {
                method: method.to_ascii_uppercase(),
                path: path.clone(),
                operation_id: operation_id.to_owned(),
                tag,
                endpoint_name,
                parameters,
                request,
                response,
                errors,
            });
        }
    }
    operations.sort_by(|left, right| {
        (&left.path, &left.method, &left.operation_id).cmp(&(
            &right.path,
            &right.method,
            &right.operation_id,
        ))
    });
    Ok(operations)
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::{config_with, spec};
    use serde_json::json;

    fn context_error(spec: &Value) -> String {
        Context::with_config(spec, crate::test_support::config()).unwrap_err()
    }

    fn context_error_with(spec: &Value, edit: impl FnOnce(&mut GeneratorConfig)) -> String {
        Context::with_config(spec, config_with(edit)).unwrap_err()
    }

    fn operations_error(spec: &Value) -> String {
        let context = Context::with_config(spec, crate::test_support::config()).unwrap();
        collect_operations(&context).unwrap_err()
    }

    fn field(spec: &mut Value, schema: &str, field: &str, value: Value) {
        spec["components"]["schemas"][schema]["properties"][field] = value;
    }

    fn override_(schema: &str, field: &str, rust_type: &str) -> FieldOverride {
        FieldOverride {
            schema: schema.to_owned(),
            field: field.to_owned(),
            rust_type: rust_type.to_owned(),
        }
    }

    #[test]
    fn embedded_config_parses_and_invalid_files_are_named() {
        assert_eq!(config().revision_field, "updateHash");
        let schemas = include_str!("../schema-names.toml");
        let responses = include_str!("../response-overrides.toml");
        let fields = include_str!("../field-rules.toml");
        assert!(parse_config(schemas, responses, fields).is_ok());
        for (arguments, file) in [
            (["x", responses, fields], "schema-names.toml"),
            ([schemas, "x", fields], "response-overrides.toml"),
            ([schemas, responses, "x"], "field-rules.toml"),
        ] {
            let error = parse_config(arguments[0], arguments[1], arguments[2]).unwrap_err();
            assert!(
                error.starts_with(&format!("invalid xtask/{file}")),
                "{error}"
            );
        }
    }

    #[test]
    fn json_helpers_report_the_expected_shape() {
        assert_eq!(object(&json!(1)), Err("expected a JSON object".to_owned()));
        assert_eq!(
            string(&json!({"name": 1}), "name"),
            Err("expected string field name".to_owned())
        );
        assert_eq!(string(&json!({"name": "x"}), "name"), Ok("x"));
        assert!(properties(&json!({})).is_empty());
        assert_eq!(
            schema_names(&json!({"components": {}})),
            Err("OpenAPI document has no components.schemas".to_owned())
        );
        assert_eq!(
            schema_names(&json!({"components": {"schemas": 1}})),
            Err("expected a JSON object".to_owned())
        );
    }

    #[test]
    fn names_follow_rust_conventions() {
        assert_eq!(rust_type_name("WidgetResponse"), Ok("Widget".to_owned()));
        assert_eq!(
            rust_type_name("WidgetRequest"),
            Ok("WidgetPayload".to_owned())
        );
        assert_eq!(rust_type_name("FactuurResponse"), Ok("Invoice".to_owned()));
        assert_eq!(
            rust_type_name("Widget"),
            Err("schema Widget is neither a Response nor a Request".to_owned())
        );
        for (value, ident) in [
            ("in use", "InUse"),
            ("", "Value"),
            ("--", "Value"),
            ("1st", "Value1st"),
        ] {
            assert_eq!(rust_ident(value), ident, "{value:?}");
        }
        for (value, field) in [
            ("TaskStatus", "task_status"),
            ("crewID", "crew_id"),
            ("type", "type_"),
            ("--", "value"),
            ("1st", "field_1st"),
            ("a-b", "a_b"),
        ] {
            assert_eq!(rust_field(value), field, "{value:?}");
        }
        assert_eq!(rust_doc(Some("  two\n words "), "fallback"), "two words");
        assert_eq!(rust_doc(Some("   "), "fall \"back\""), "fall \\\"back\\\"");
        assert_eq!(rust_doc(None, "fallback"), "fallback");
    }

    #[test]
    fn openapi_types_allow_one_nullable_kind() {
        assert_eq!(
            openapi_type(&json!({"type": "string"})),
            Ok((Some("string"), false))
        );
        assert_eq!(
            openapi_type(&json!({"type": ["null", "integer"]})),
            Ok((Some("integer"), true))
        );
        assert_eq!(openapi_type(&json!({})), Ok((None, false)));
        assert_eq!(
            openapi_type(&json!({"type": ["string", "integer"]})),
            Err("unsupported union type: [String(\"string\"), String(\"integer\")]".to_owned())
        );
    }

    #[test]
    fn enum_variants_are_unique_identifiers() {
        let mut used = BTreeSet::from(["Unknown".to_owned()]);
        assert_eq!(enum_variant_name(&json!("unknown"), &mut used), "Unknown2");
        assert_eq!(enum_variant_name(&json!("unknown"), &mut used), "Unknown3");
        assert_eq!(enum_variant_name(&json!("self"), &mut used), "SelfValue");
        assert_eq!(enum_variant_name(&json!(2), &mut used), "Value2");
    }

    #[test]
    fn config_must_match_the_document() {
        let spec = spec();
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .resource_names
                    .insert("Ghost".to_owned(), "Spirit".to_owned());
            }),
            "schema-names.toml maps unknown resource schema GhostResponse"
        );
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .schema_type_overrides
                    .insert("GhostResponse".to_owned(), "Ghost".to_owned());
            }),
            "schema-names.toml overrides unknown schema GhostResponse"
        );
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .response_type_overrides
                    .push(override_("GhostResponse", "id", "i64"));
            }),
            "generator config references unknown schema GhostResponse"
        );
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .integer_id_fields
                    .push(override_("PartResponse", "ghost", "i64"));
            }),
            "generator config references unknown field PartResponse.ghost"
        );
        let nullable = |schema: &str, field: &str| SchemaField {
            schema: schema.to_owned(),
            field: field.to_owned(),
        };
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .nullable_response_fields
                    .push(nullable("GhostResponse", "id"));
            }),
            "generator config references unknown schema GhostResponse"
        );
        assert_eq!(
            context_error_with(&spec, |config| {
                config
                    .nullable_response_fields
                    .push(nullable("PartResponse", "ghost"));
            }),
            "generator config references unknown nullable field PartResponse.ghost"
        );
        assert_eq!(
            context_error_with(&spec, |config| config
                .uncountable_types
                .push("Ghost".to_owned())),
            "field-rules.toml marks unknown resource Ghost uncountable"
        );
    }

    #[test]
    fn context_rejects_malformed_documents() {
        assert_eq!(
            context_error(&json!({"paths": 1})),
            "expected a JSON object"
        );
        assert_eq!(
            context_error(&json!({"paths": {}})),
            "OpenAPI document has no components.schemas"
        );

        let mut spec = spec();
        spec["components"]["schemas"]["PartResponse"]["properties"] = json!([]);
        assert_eq!(
            context_error(&spec),
            "schema PartResponse has non-object properties"
        );

        let mut spec = crate::test_support::spec();
        spec["components"]["schemas"]["Part"] = json!({});
        assert_eq!(
            context_error(&spec),
            "schema Part is neither a Response nor a Request"
        );

        let mut spec = crate::test_support::spec();
        spec["paths"]["/parts/{id}"]["get"]["responses"] = json!({});
        assert_eq!(
            context_error(&spec),
            "operation getPartItem has no 200 JSON schema"
        );

        let mut spec = crate::test_support::spec();
        spec["paths"]["/parts/{id}"]["get"]["responses"]["200"]["content"]["application/json"]["schema"]
            ["allOf"][0]["properties"]["data"]["$ref"] =
            json!("#/components/schemas/GhostResponse");
        assert_eq!(
            context_error(&spec),
            "/parts/{id} references undefined schema GhostResponse"
        );

        let mut spec = crate::test_support::spec();
        spec["paths"]["/parts/{id}"]["get"]["responses"]["200"]["content"]["application/json"]["schema"]
            ["allOf"][0]["properties"]["data"]["$ref"] = json!("#/components/schemas/PartRequest");
        assert_eq!(
            context_error(&spec),
            "/parts/{id} returns PartRequest, not a Response schema"
        );

        let mut spec = crate::test_support::spec();
        spec["paths"]["/spares/{id}"] = spec["paths"]["/parts/{id}"].clone();
        assert!(context_error(&spec).starts_with("a type serves several collections: "));
    }

    #[test]
    fn context_skips_paths_that_define_no_item_read() {
        let mut spec = spec();
        spec["paths"]["/drafts/{id}"] =
            json!({"delete": spec["paths"]["/widgets/{id}"]["delete"].clone()});
        spec["paths"]["/lists/{id}"] = json!({"get": spec["paths"]["/parts"]["get"].clone()});
        let context = Context::with_config(&spec, crate::test_support::config()).unwrap();
        assert_eq!(
            context.resources,
            BTreeMap::from([
                ("parts".to_owned(), "Part".to_owned()),
                ("widgets".to_owned(), "Widget".to_owned()),
            ])
        );
        assert_eq!(context.resource_schema("Part").0, "PartResponse");
        assert_eq!(
            context.resource_for_collection("ghosts", "Widget.ghost"),
            Err("Widget.ghost links to undocumented collection /ghosts".to_owned())
        );
    }

    #[test]
    fn field_types_follow_the_document_and_config() {
        let mut spec = spec();
        field(
            &mut spec,
            "WidgetResponse",
            "order",
            json!({"type": "string"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "quantity",
            json!({"type": "integer"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "factor",
            json!({"type": "string"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "website",
            json!({"type": "string"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "modified",
            json!({"type": "string"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "owner",
            json!({"type": "string", "example": "/parts/2"}),
        );
        field(
            &mut spec,
            "WidgetResponse",
            "maybe",
            json!({"type": "string"}),
        );
        let config = config_with(|config| {
            config
                .response_string_integer_fields
                .push("order".to_owned());
            config
                .integer_id_fields
                .push(override_("WidgetResponse", "quantity", "ItemId"));
            config.response_type_overrides.push(override_(
                "WidgetResponse",
                "factor",
                "RentmanAmount",
            ));
            config.url_fields.push("website".to_owned());
            config.date_time_fields.push("modified".to_owned());
            config.reference_fields.push("owner".to_owned());
            config.nullable_response_fields.push(SchemaField {
                schema: "WidgetResponse".to_owned(),
                field: "maybe".to_owned(),
            });
        });
        let context = Context::with_config(&spec, config).unwrap();
        let required = BTreeSet::from(["id".to_owned(), "maybe".to_owned()]);
        for (name, expected) in [
            ("id", "Id<Widget>"),
            ("name", "Option<String>"),
            ("created", "Option<DateTime<FixedOffset>>"),
            ("modified", "Option<DateTime<FixedOffset>>"),
            ("part", "Option<Id<Part>>"),
            ("owner", "Option<Id<Part>>"),
            ("status", "Option<WidgetStatus>"),
            ("price", "Option<RentmanAmount>"),
            ("active", "Option<bool>"),
            ("day", "Option<NaiveDate>"),
            ("email", "Option<EmailAddress>"),
            ("homepage_url", "Option<Url>"),
            ("website", "Option<Url>"),
            ("count", "Option<i64>"),
            ("custom", "Option<CustomFields>"),
            ("order", "Option<i64>"),
            ("quantity", "Option<ItemId>"),
            ("factor", "Option<RentmanAmount>"),
            ("maybe", "Option<String>"),
        ] {
            assert_eq!(
                context.field_type("WidgetResponse", name, &required),
                expected,
                "{name}"
            );
        }
        assert_eq!(context.field_kind("WidgetResponse", "part"), Some("string"));
        assert_eq!(context.base_type("WidgetResponse", "part"), "Id<Part>");
        assert_eq!(
            context.effective_required(
                "WidgetResponse",
                &spec["components"]["schemas"]["WidgetResponse"]
            ),
            BTreeSet::from(["id".to_owned(), "name".to_owned()])
        );
        assert!(context.is_resource_schema("PartResponse"));
        assert!(!context.is_resource_schema("PartRequest"));
        assert_eq!(
            context.referenced_type("PartRequest", "x"),
            Ok("PartPayload")
        );
    }

    #[test]
    fn field_types_reject_what_the_generator_cannot_map() {
        let cases = [
            (
                json!({"type": ["string", "integer"]}),
                "unsupported union type: [String(\"string\"), String(\"integer\")]",
            ),
            (
                json!({"type": "integer", "description": "ID of the part"}),
                "WidgetResponse.extra needs an integer ID rule in xtask/schema-names.toml",
            ),
            (
                json!({"type": "string", "example": "/parts/x"}),
                "link field WidgetResponse.extra has invalid example: \"/parts/x\"",
            ),
            (
                json!({"type": "string", "example": "/ghosts/1"}),
                "WidgetResponse.extra links to undocumented collection /ghosts",
            ),
            (
                json!({"type": "array"}),
                "unsupported property type for WidgetResponse.extra: {\"type\":\"array\"}",
            ),
        ];
        for (schema, expected) in cases {
            let mut spec = spec();
            field(&mut spec, "WidgetResponse", "extra", schema);
            assert_eq!(context_error(&spec), expected);
        }

        let mut spec = spec();
        field(
            &mut spec,
            "WidgetResponse",
            "part_id",
            json!({"type": "integer"}),
        );
        assert_eq!(
            context_error(&spec),
            "WidgetResponse.part_id needs an integer ID rule in xtask/schema-names.toml"
        );

        let mut spec = crate::test_support::spec();
        field(&mut spec, "WidgetRequest", "id", json!({"type": "integer"}));
        assert_eq!(
            context_error(&spec),
            "WidgetRequest.id belongs to no documented collection"
        );

        let mut spec = crate::test_support::spec();
        field(
            &mut spec,
            "WidgetResponse",
            "owner",
            json!({"type": "string"}),
        );
        let error = Context::with_config(
            &spec,
            config_with(|config| config.reference_fields.push("owner".to_owned())),
        )
        .unwrap_err();
        assert_eq!(
            error,
            "link field WidgetResponse.owner has no /{collection}/{id} example"
        );

        let mut spec = crate::test_support::spec();
        field(
            &mut spec,
            "WidgetResponse",
            "order",
            json!({"type": ["string", "integer"]}),
        );
        let error = Context::with_config(
            &spec,
            config_with(|config| {
                config
                    .response_string_integer_fields
                    .push("order".to_owned())
            }),
        )
        .unwrap_err();
        assert!(error.starts_with("unsupported union type"), "{error}");
    }

    #[test]
    fn response_shapes_name_their_item_schema() {
        let shape = |schema: Value| {
            response_shape(
                &json!({"operationId": "op", "responses": {"200": {"content": {"application/json": {"schema": schema}}}}}),
            )
        };
        assert_eq!(shape(json!({"type": "null"})), Ok(Response::NoContent));
        assert_eq!(
            shape(json!({"type": "object"})),
            Err("operation op has no data schema".to_owned())
        );
        assert_eq!(
            shape(json!({"allOf": [{}]})),
            Err("operation op has no data schema".to_owned())
        );
        assert_eq!(
            shape(json!({"allOf": [{"properties": {"data": {"type": "array"}}}]})),
            Err("operation op has an array without an item schema".to_owned())
        );
        assert_eq!(
            shape(
                json!({"allOf": [{"properties": {"data": {"type": "array", "items": {"$ref": 1}}}}]})
            ),
            Err("invalid OpenAPI schema reference".to_owned())
        );
        assert_eq!(
            shape(json!({"allOf": [{"properties": {"data": {"$ref": 1}}}]})),
            Err("invalid OpenAPI schema reference".to_owned())
        );
        assert_eq!(
            shape(json!({"allOf": [{"properties": {"data": {"type": "object"}}}]})),
            Err("operation op has unsupported response schema".to_owned())
        );
        assert_eq!(
            response_shape(&json!({})),
            Err("operation unknown has no 200 JSON schema".to_owned())
        );
        assert_eq!(Response::Collection("A".to_owned()).schema(), Some("A"));
        assert_eq!(Response::NoContent.schema(), None);
    }

    #[test]
    fn operations_carry_parameters_errors_and_unique_names() {
        let mut spec = spec();
        spec["paths"]["/b"] = json!({"get": {"operationId": "GetB", "responses": spec["paths"]["/parts"]["get"]["responses"].clone()}});
        spec["paths"]["/b/"] = json!({"get": {"operationId": "getPartCollection", "responses": spec["paths"]["/parts"]["get"]["responses"].clone()}});
        spec["paths"]["/c"] = json!({"get": {"operationId": "getPartCollection", "responses": spec["paths"]["/parts"]["get"]["responses"].clone()}});
        let context = Context::with_config(&spec, crate::test_support::config()).unwrap();
        let operations = collect_operations(&context).unwrap();
        let names = operations
            .iter()
            .map(|operation| (operation.path.as_str(), operation.endpoint_name.as_str()))
            .filter(|(path, _)| ["/b", "/b/", "/c", "/parts"].contains(path))
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                ("/b", "GetBEndpoint"),
                ("/b/", "GetBEndpoint2"),
                ("/c", "GetCEndpoint"),
                ("/parts", "GetPartCollectionEndpoint"),
            ]
        );
        let widgets = operations
            .iter()
            .find(|operation| operation.operation_id == "getWidgetCollection")
            .unwrap();
        assert_eq!(widgets.tag, "Widgets");
        assert_eq!(
            widgets.errors,
            [(400, "Bad request".to_owned()), (404, String::new())]
        );
        let parameters = widgets
            .parameters
            .iter()
            .map(|parameter| {
                (
                    parameter.name.as_str(),
                    parameter.location.as_str(),
                    parameter.required,
                    parameter.value.as_str(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parameters,
            [
                ("limit", "Query", false, "Integer"),
                ("ratio", "Query", false, "Number"),
                ("archived", "Query", false, "Boolean"),
                ("sort", "Query", false, "String"),
            ]
        );
        let item = operations
            .iter()
            .find(|operation| operation.operation_id == "getWidgetItem")
            .unwrap();
        assert_eq!(
            (
                item.parameters[0].location.as_str(),
                item.parameters[0].required
            ),
            ("Path", true)
        );
    }

    #[test]
    fn operations_reject_unsupported_definitions() {
        let parameter = |value: Value| {
            let mut spec = spec();
            spec["paths"]["/parts"]["get"]["parameters"] = json!([value]);
            operations_error(&spec)
        };
        assert_eq!(
            parameter(json!({"name": "x", "in": "header", "schema": {"type": "string"}})),
            "operation getPartCollection has unsupported parameter location Some(\"header\")"
        );
        assert!(
            parameter(
                json!({"name": "x", "in": "query", "schema": {"type": ["string", "integer"]}})
            )
            .starts_with("unsupported union type")
        );
        assert_eq!(
            parameter(json!({"name": "x", "in": "query", "schema": {"type": ["string", "null"]}})),
            "nullable endpoint parameters are unsupported: {\"type\":[\"string\",\"null\"]}"
        );
        assert_eq!(
            parameter(json!({"name": "x", "in": "query"})),
            "unsupported endpoint parameter schema: null"
        );
        assert_eq!(
            parameter(json!({"in": "query", "schema": {"type": "string"}})),
            "expected string field name"
        );

        let mut spec = spec();
        spec["paths"]["/numbers"] = json!(1);
        assert_eq!(operations_error(&spec), "expected a JSON object");

        let mut spec = crate::test_support::spec();
        spec["paths"]["/parts"]["get"]["responses"] = json!({});
        assert_eq!(
            operations_error(&spec),
            "operation getPartCollection has no 200 JSON schema"
        );

        let mut spec = crate::test_support::spec();
        spec["paths"]["/widgets"]["post"]["requestBody"]["content"]["application/json"]["schema"]
            ["$ref"] = json!(1);
        assert_eq!(operations_error(&spec), "invalid OpenAPI schema reference");

        let mut spec = crate::test_support::spec();
        spec["paths"]["/widgets"]["post"]["requestBody"]["content"]["application/json"]["schema"]
            ["$ref"] = json!("#/components/schemas/GhostRequest");
        assert_eq!(
            operations_error(&spec),
            "operation createWidget references undefined schema GhostRequest"
        );
    }
}
