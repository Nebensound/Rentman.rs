//! Shared helpers for the generator's unit tests.

use crate::openapi::{Context, GeneratorConfig, Operation, collect_operations};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A temporary directory that is removed when dropped.
pub struct ScratchDir(PathBuf);

impl ScratchDir {
    pub fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "rentman-xtask-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Serves one canned response per request path on a local port and returns
/// the base URL. `responses` receives that URL so bodies can link back to it.
pub fn serve(responses: impl FnOnce(&str) -> Vec<(&'static str, u16, String)>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let routes = responses(&base)
        .into_iter()
        .map(|(path, status, body)| (path.to_owned(), (status, body)))
        .collect::<BTreeMap<_, _>>();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request_line = String::new();
            BufReader::new(&stream)
                .read_line(&mut request_line)
                .unwrap();
            let path = request_line.split_whitespace().nth(1).unwrap_or("/");
            let (status, body) = routes
                .get(path)
                .cloned()
                .unwrap_or((404, "not found".to_owned()));
            write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
    });
    base
}

/// An empty generator config; `edit` adds the rules a test needs.
pub fn config_with(edit: impl FnOnce(&mut GeneratorConfig)) -> &'static GeneratorConfig {
    let mut config = GeneratorConfig {
        revision_field: "updateHash".to_owned(),
        response_core_fields: vec!["id".to_owned(), "name".to_owned()],
        response_string_integer_fields: Vec::new(),
        date_time_fields: Vec::new(),
        url_fields: Vec::new(),
        reference_fields: Vec::new(),
        filterable_types: [
            "String",
            "i64",
            "bool",
            "Decimal",
            "DateTime<FixedOffset>",
            "NaiveDate",
            "EmailAddress",
            "Url",
        ]
        .map(str::to_owned)
        .to_vec(),
        uncountable_types: Vec::new(),
        resource_names: BTreeMap::new(),
        schema_type_overrides: BTreeMap::new(),
        enum_variant_names: BTreeMap::new(),
        response_type_overrides: Vec::new(),
        nullable_response_fields: Vec::new(),
        integer_id_fields: Vec::new(),
    };
    edit(&mut config);
    Box::leak(Box::new(config))
}

pub fn config() -> &'static GeneratorConfig {
    config_with(|_| {})
}

fn collection(schema: &str) -> Value {
    json!({"content": {"application/json": {"schema": {"allOf": [
        {"type": "object"},
        {"properties": {"data": {"type": "array", "items": {"$ref": format!("#/components/schemas/{schema}")}}}}
    ]}}}})
}

fn item(schema: &str) -> Value {
    json!({"content": {"application/json": {"schema": {"allOf": [
        {"properties": {"data": {"$ref": format!("#/components/schemas/{schema}")}}}
    ]}}}})
}

fn body(schema: &str) -> Value {
    json!({"content": {"application/json": {"schema": {"$ref": format!("#/components/schemas/{schema}")}}}})
}

fn id_parameter() -> Value {
    json!({"name": "id", "in": "path", "required": true, "schema": {"type": "integer"}})
}

/// A small document that exercises every resource shape and field type the
/// generator supports and renders without errors under [`config`].
pub fn spec() -> Value {
    json!({
        "openapi": "3.0.0",
        "paths": {
            "/parts": {
                "get": {"operationId": "getPartCollection", "responses": {"200": collection("PartResponse")}}
            },
            "/parts/{id}": {
                "get": {"operationId": "getPartItem", "parameters": [id_parameter()], "responses": {"200": item("PartResponse")}}
            },
            "/widgets": {
                "get": {
                    "operationId": "getWidgetCollection",
                    "tags": ["Widgets"],
                    "parameters": [
                        {"name": "limit", "in": "query", "schema": {"type": "integer"}},
                        {"name": "ratio", "in": "query", "schema": {"type": "number"}},
                        {"name": "archived", "in": "query", "schema": {"type": "boolean"}},
                        {"name": "sort", "in": "query", "schema": {"type": "string"}}
                    ],
                    "responses": {"200": collection("WidgetResponse"), "400": {"description": "Bad request"}, "404": {}}
                },
                "post": {"operationId": "createWidget", "requestBody": body("WidgetRequest"), "responses": {"200": item("WidgetResponse")}}
            },
            "/widgets/{id}": {
                "get": {"operationId": "getWidgetItem", "parameters": [id_parameter()], "responses": {"200": item("WidgetResponse")}},
                "put": {"operationId": "updateWidget", "parameters": [id_parameter()], "requestBody": body("WidgetUpdateRequest"), "responses": {"200": item("WidgetResponse")}},
                "delete": {"operationId": "deleteWidget", "parameters": [id_parameter()], "responses": {"200": {"content": {"application/json": {"schema": {"type": "null"}}}}}}
            },
            "/widgets/{id}/parts": {
                "get": {"operationId": "getWidgetPartCollection", "parameters": [id_parameter()], "responses": {"200": collection("PartResponse")}},
                "post": {"operationId": "createWidgetPart", "parameters": [id_parameter()], "requestBody": body("PartRequest"), "responses": {"200": item("PartResponse")}}
            },
            "/widgets/{id}/:archive": {
                "post": {"operationId": "archiveWidget", "summary": "Archive a widget.", "parameters": [id_parameter()], "requestBody": body("WidgetArchiveRequest"), "responses": {"200": item("WidgetResponse")}}
            },
            "/widgets/:bulk": {
                "post": {"operationId": "bulkWidget", "requestBody": body("WidgetBulkRequest"), "responses": {"200": item("WidgetResponse")}}
            },
            "/documentation": {"description": "not an operation"}
        },
        "components": {"schemas": {
            "PartResponse": {"properties": {
                "id": {"type": "integer"},
                "name": {"type": "string"}
            }},
            "PartRequest": {"required": ["name"], "properties": {"name": {"type": "string"}}},
            "WidgetResponse": {"description": "A widget.", "properties": {
                "id": {"type": "integer"},
                "name": {"type": ["string", "null"], "description": "Display   name."},
                "created": {"type": "string", "format": "date-time"},
                "part": {"type": "string", "example": "/parts/1"},
                "status": {"type": "string", "enum": ["active", "in use", 1]},
                "price": {"type": "number"},
                "active": {"type": "boolean"},
                "day": {"type": "string", "format": "date"},
                "email": {"type": "string", "format": "email"},
                "homepage_url": {"type": "string"},
                "count": {"type": "integer"},
                "type": {"type": "string"},
                "custom": {"type": "object"},
                "note": {"type": "string", "description": "GENERATED FIELD"}
            }},
            "WidgetRequest": {
                "required": ["name", "status", "price", "active", "day", "created", "part", "count"],
                "properties": {
                    "name": {"type": "string"},
                    "status": {"type": "string", "enum": ["active", "retired"]},
                    "price": {"type": "number"},
                    "active": {"type": "boolean"},
                    "day": {"type": "string", "format": "date"},
                    "created": {"type": "string", "format": "date-time"},
                    "part": {"type": "string", "example": "/parts/1"},
                    "count": {"type": "integer"},
                    "label": {"type": "string"}
                }
            },
            "WidgetUpdateRequest": {"properties": {"name": {"type": "string"}}},
            "WidgetArchiveRequest": {"required": ["reason"], "properties": {"reason": {"type": "string"}}},
            "WidgetBulkRequest": {"properties": {"names": {"type": "string"}}}
        }}
    })
}

/// The context and operations for `spec` under [`config`].
pub fn context(spec: &Value) -> Result<(Context<'_>, Vec<Operation>), String> {
    let context = Context::with_config(spec, config())?;
    let operations = collect_operations(&context)?;
    Ok((context, operations))
}
