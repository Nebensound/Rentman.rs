use crate::generator::{
    contract::{sample_value, sample_wire},
    live_sweep::accessor_name,
};
use crate::openapi::{Context, Operation, properties, rust_doc, rust_field};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub fn generate(
    context: &Context<'_>,
    operations: &[Operation],
) -> Result<(String, String), String> {
    let top_collection = Regex::new(r"^/([a-z_]+)$").expect("static regex");
    let item_path = Regex::new(r"^/([a-z_]+)/\{id\}$").expect("static regex");
    let nested_path = Regex::new(r"^/([a-z_]+)/\{id\}/([a-z_]+)$").expect("static regex");
    let action_path = Regex::new(r"^/([a-z_]+)(/\{id\})?/:([A-Za-z]+)$").expect("static regex");
    let mut impls = Vec::new();
    let mut markers = Vec::new();
    let mut collection_methods: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut reachable = Vec::new();
    let mut tests = Vec::new();
    let mut sealed = BTreeSet::new();
    let samples = response_samples(context, operations)?;

    for operation in operations {
        let path = operation.path.as_str();
        let method = operation.method.as_str();
        let item = operation
            .response
            .schema()
            .map(|schema| context.type_name(schema));
        let payload = operation
            .request
            .as_deref()
            .map(|schema| context.type_name(schema));
        let endpoint_spec = format!(
            "Operation(<endpoint::{} as Endpoint>::SPEC)",
            operation.endpoint_name
        );
        let test_name = rust_field(&operation.endpoint_name)
            .trim_end_matches("_endpoint")
            .to_owned();

        if let Some(captures) = top_collection.captures(path) {
            let collection = &captures[1];
            let owner = context.resource_for_collection(collection, path)?;
            seal(&mut impls, &mut sealed, owner);
            let accessor = accessor_name(owner);
            let base = format!("client.{accessor}()");
            match method {
                "GET" => {
                    let item = item.ok_or("collection GET has no response item")?;
                    impls.push(format!("impl List for {owner} {{ type Item = {item}; const LIST: Operation = {endpoint_spec}; }}"));
                    reachable.push(format!("<{owner} as List>::LIST.0.operation_id"));
                    tests.push(test_source(
                        &test_name,
                        "json_response(r#\"{\"data\":[],\"itemCount\":0,\"limit\":300}\"#)",
                        &format!("{base}.list().all()"),
                        "GET",
                        path,
                        None,
                        "assert!(result.unwrap().is_empty());",
                    ));
                    add_paged_collection_test(
                        &mut tests,
                        &test_name,
                        path,
                        &format!("{base}.list()"),
                        &samples,
                        operation
                            .response
                            .schema()
                            .expect("collection has item schema"),
                    );
                    add_collection_errors(
                        &mut tests,
                        &test_name,
                        &base,
                        path,
                        &operation.operation_id,
                        owner,
                    );
                }
                "POST" => {
                    let (payload, item) = (
                        payload.ok_or("POST has no request schema")?,
                        item.ok_or("POST has no response schema")?,
                    );
                    impls.push(format!("impl Create for {owner} {{ type Payload = {payload}; type Item = {item}; const CREATE: Operation = {endpoint_spec}; }}"));
                    reachable.push(format!("<{owner} as Create>::CREATE.0.operation_id"));
                    let expression = payload_expression(
                        context,
                        operation.request.as_deref().unwrap(),
                        "crate::",
                    )?;
                    let response =
                        item_response_expression(&samples, operation.response.schema().unwrap(), 7);
                    tests.push(test_source(
                        &test_name,
                        &response,
                        &format!("{base}.create_as::<{owner}IdProjection>(&{expression})"),
                        "POST",
                        path,
                        Some(&expression),
                        SENT_ONCE,
                    ));
                }
                _ => return Err(format!("unsupported collection operation {method} {path}")),
            }
            continue;
        }

        if let Some(captures) = item_path.captures(path) {
            let collection = &captures[1];
            let owner = context.resource_for_collection(collection, path)?;
            seal(&mut impls, &mut sealed, owner);
            let base = format!("client.{}()", accessor_name(owner));
            let concrete = path.replace("{id}", "7");
            let (trait_name, const_name) = match method {
                "GET" => ("Get", "GET"),
                "PUT" => ("Update", "UPDATE"),
                "DELETE" => ("Delete", "DELETE"),
                _ => return Err(format!("unsupported item operation {method} {path}")),
            };
            let assoc = if method == "PUT" {
                format!(
                    "type Payload = {}; type Item = {owner};",
                    payload.ok_or("PUT has no payload schema")?
                )
            } else {
                format!("type Item = {owner};")
            };
            impls.push(format!("impl {trait_name} for {owner} {{ {assoc} const {const_name}: Operation = {endpoint_spec}; }}"));
            if method == "GET" {
                let response_schema = operation
                    .response
                    .schema()
                    .expect("item GETs of documented collections return an item");
                let schema = &context.schemas[response_schema];
                let mut fields = properties(schema)
                    .keys()
                    .filter(|field| field.as_str() != "custom")
                    .map(|field| format!("\"{field}\""))
                    .collect::<Vec<_>>();
                fields.push(format!("\"{}\"", context.config.revision_field));
                impls.push(format!("impl GetItem for {owner} {{ const GET_ITEM: Operation = {endpoint_spec}; const ITEM_FIELDS: &'static [&'static str] = &[{}]; }}", fields.join(", ")));
            }
            reachable.push(format!(
                "<{owner} as {trait_name}>::{const_name}.0.operation_id"
            ));
            // `trait_name` admitted only GET, PUT and DELETE above.
            let (response, call, body, check) = match trait_name {
                "Get" => (
                    item_response_expression(
                        &samples,
                        operation.response.schema().expect("item GET has schema"),
                        7,
                    ),
                    format!("{base}.get(crate::Id::new(7))"),
                    None,
                    SENT_ONCE.to_owned(),
                ),
                "Update" => {
                    let expression = payload_expression(
                        context,
                        operation.request.as_deref().unwrap(),
                        "crate::",
                    )?;
                    (
                        item_response_expression(
                            &samples,
                            operation.response.schema().expect("item update has schema"),
                            7,
                        ),
                        format!(
                            "{base}.update_as::<{owner}IdProjection>(crate::Id::new(7), &{expression})"
                        ),
                        Some(expression),
                        SENT_ONCE.to_owned(),
                    )
                }
                _ => (
                    "empty_response(\"HTTP/1.1 204 No Content\")".to_owned(),
                    format!("{base}.delete(crate::Id::new(7))"),
                    None,
                    "result.unwrap();".to_owned(),
                ),
            };
            tests.push(test_source(
                &test_name,
                &response,
                &call,
                method,
                &concrete,
                body.as_deref(),
                &check,
            ));
            continue;
        }

        if let Some(captures) = nested_path.captures(path) {
            let parent_collection = &captures[1];
            let child_collection = &captures[2];
            let parent = context.resource_for_collection(parent_collection, path)?;
            let child = context.resource_for_collection(child_collection, path)?;
            let child_accessor = accessor_name(child);
            let marker = format!("{parent}{}", pascal(&child_accessor));
            if !sealed.contains(&marker) {
                markers.extend([
                    format!("/// The {} of one [`{parent}`]: `/{parent_collection}/{{id}}/{child_collection}`.", child_accessor.replace('_', " ")),
                    "#[derive(Debug, Clone, Copy, PartialEq, Eq)]".to_owned(), format!("pub struct {marker};"), String::new(),
                ]);
                let parent_accessor = accessor_name(parent);
                // A collection without GET is shown with `create`; an invalid
                // payload is reported by the POST arm below.
                let create_only = context.spec["paths"][path].get("get").is_none();
                let create_payload = operation
                    .request
                    .as_deref()
                    .filter(|_| create_only)
                    .and_then(|request| {
                        payload_expression(context, request, "rentman_client::").ok()
                    });
                let call = match create_payload {
                    Some(payload) => format!(
                        "let payload = {payload};\nlet created = client.{parent_accessor}().{child_accessor}(rentman_client::Id::new(12)).create(&payload).await?;"
                    ),
                    None => format!(
                        "let items = client.{parent_accessor}().{child_accessor}(rentman_client::Id::new(12)).list().all().await?;"
                    ),
                };
                let methods = collection_methods.entry(parent.to_owned()).or_default();
                methods.extend([
                    format!("    /// The {} of one item: `/{parent_collection}/{{id}}/{child_collection}`.", child_accessor.replace('_', " ")),
                    doc_example(&call),
                    format!("    pub fn {child_accessor}(&self, {}: Id<{parent}>) -> Collection<{marker}> {{", rust_field(parent)),
                    format!("        Collection::new(&self.client, Some({}.get()))", rust_field(parent)), "    }".to_owned(), String::new(),
                ]);
            }
            seal(&mut impls, &mut sealed, &marker);
            seal(&mut impls, &mut sealed, parent);
            let base = format!(
                "client.{}().{}(crate::Id::new(7))",
                accessor_name(parent),
                child_accessor
            );
            let concrete = path.replace("{id}", "7");
            match method {
                "GET" => {
                    let item = item.ok_or("nested GET has no response item")?;
                    impls.push(format!("impl List for {marker} {{ type Item = {item}; const LIST: Operation = {endpoint_spec}; }}"));
                    reachable.push(format!("<{marker} as List>::LIST.0.operation_id"));
                    tests.push(test_source(
                        &test_name,
                        EMPTY_PAGE,
                        &format!("{base}.list().all()"),
                        method,
                        &concrete,
                        None,
                        "assert!(result.unwrap().is_empty());",
                    ));
                    add_paged_collection_test(
                        &mut tests,
                        &test_name,
                        &concrete,
                        &format!("{base}.list()"),
                        &samples,
                        operation
                            .response
                            .schema()
                            .expect("nested GET has item schema"),
                    );
                    add_collection_errors(
                        &mut tests,
                        &test_name,
                        &base,
                        &concrete,
                        &operation.operation_id,
                        child,
                    );
                }
                "POST" => {
                    let (payload, item) = (
                        payload.ok_or("nested POST has no payload")?,
                        item.ok_or("nested POST has no item")?,
                    );
                    impls.push(format!("impl Create for {marker} {{ type Payload = {payload}; type Item = {item}; const CREATE: Operation = {endpoint_spec}; }}"));
                    reachable.push(format!("<{marker} as Create>::CREATE.0.operation_id"));
                    let expression = payload_expression(
                        context,
                        operation.request.as_deref().unwrap(),
                        "crate::",
                    )?;
                    let response =
                        item_response_expression(&samples, operation.response.schema().unwrap(), 7);
                    tests.push(test_source(
                        &test_name,
                        &response,
                        &format!("{base}.create_as::<{child}IdProjection>(&{expression})"),
                        method,
                        &concrete,
                        Some(&expression),
                        SENT_ONCE,
                    ));
                }
                _ => return Err(format!("unsupported nested operation {method} {path}")),
            }
            continue;
        }

        if let Some(captures) = action_path.captures(path) {
            let collection = &captures[1];
            let owner = context.resource_for_collection(collection, path)?;
            let action = rust_field(&captures[3]);
            let const_name = format!(
                "{}_{}",
                rust_field(owner).to_ascii_uppercase(),
                action.to_ascii_uppercase()
            );
            impls.push(format!("const {const_name}: Operation = {endpoint_spec};"));
            reachable.push(format!("{const_name}.0.operation_id"));
            seal(&mut impls, &mut sealed, owner);
            let summary = context.spec["paths"][path]["post"]
                .get("summary")
                .and_then(Value::as_str);
            let doc = rust_doc(summary, &format!("Runs `{method} {path}`."));
            let payload_type = payload.ok_or("action has no request schema")?;
            let item_type = item.ok_or("action has no response schema")?;
            let response = item_response_expression(
                &samples,
                operation.response.schema().expect("action has item schema"),
                7,
            );
            let payload =
                payload_expression(context, operation.request.as_deref().unwrap(), "crate::")?;
            let example_payload = payload.replace("crate::", "rentman_client::");
            let owner_accessor = accessor_name(owner);
            let methods = collection_methods.entry(owner.to_owned()).or_default();
            if captures.get(2).is_some() {
                let id_param = rust_field(owner);
                methods.extend([
                    format!("    /// {doc}"), "    ///".to_owned(), "    /// Sent exactly once, like [`Collection::create`].".to_owned(),
                    doc_example(&format!("let payload = {example_payload};\nlet result = client.{owner_accessor}().{action}(rentman_client::Id::new(12), &payload).await?;")),
                    format!("    pub async fn {action}(&self, {id_param}: Id<{owner}>, payload: &{payload_type}) -> Result<{item_type}> {{"),
                    format!("        self.item_action({const_name}, {id_param}, payload).await"), "    }".to_owned(), String::new(),
                ]);
                tests.push(test_source(
                    &test_name,
                    &response,
                    &format!(
                        "client.{}().{action}(crate::Id::new(7), &{payload})",
                        accessor_name(owner)
                    ),
                    method,
                    &path.replace("{id}", "7"),
                    Some(&payload),
                    SENT_ONCE,
                ));
            } else {
                methods.extend([
                    format!("    /// {doc}"), "    ///".to_owned(), "    /// Sent exactly once, like [`Collection::create`].".to_owned(),
                    doc_example(&format!("let payload = {example_payload};\nlet result = client.{owner_accessor}().{action}(&payload).await?;")),
                    format!("    pub async fn {action}(&self, payload: &{payload_type}) -> Result<{item_type}> {{"),
                    format!("        self.collection_action({const_name}, payload).await"), "    }".to_owned(), String::new(),
                ]);
                tests.push(test_source(
                    &test_name,
                    &response,
                    &format!("client.{}().{action}(&{payload})", accessor_name(owner)),
                    method,
                    path,
                    Some(&payload),
                    SENT_ONCE,
                ));
            }
            continue;
        }
        return Err(format!("no resource shape for {method} {path}"));
    }

    let mut resources = context.resources.iter().collect::<Vec<_>>();
    resources.sort_by_key(|(_, resource)| accessor_name(resource));
    let accessors = resources.into_iter().map(|(collection, owner)| {
        let accessor = accessor_name(owner);
        let (schema, _) = context.resource_schema(owner);
        let paths = &context.spec["paths"];
        let example = if paths[format!("/{collection}")].get("get").is_some() {
            doc_example(&format!("let first: Vec<rentman_client::model::{owner}> = client.{accessor}().list().first(10).await?;"))
        } else {
            // Every resource is defined by its item GET, so `get` always exists.
            doc_example(&format!("let item: rentman_client::model::{owner} = client.{accessor}().get(rentman_client::Id::new(12)).await?;"))
        };
        format!("    /// All [`{owner}`] items: `/{collection}`.\n    ///\n    /// OpenAPI response schema: `{schema}`; Rust resource: [`{owner}`].\n    /// See the [resource name mapping](crate::model#resource-names-and-api-collections).\n{example}\n    pub fn {accessor}(&self) -> Collection<{owner}> {{\n        Collection::new(self, None)\n    }}\n\n")
    }).collect::<Vec<_>>().join("");

    let mut output = String::from(
        "//! Generated collections of the resource layer.\n//!\n//! This file is generated by `cargo xtask update-openapi`.\n\nuse super::{Collection, Create, Delete, Get, GetItem, List, Operation, Update, sealed::Sealed};\nuse crate::{\n    RentmanClient,\n    endpoint::{self, Endpoint},\n    error::Result,\n    id::Id,\n    model::*,\n};\n\n",
    );
    for line in markers {
        writeln!(output, "{line}").unwrap();
    }
    for line in impls {
        writeln!(output, "{line}").unwrap();
    }
    output.push_str("\nimpl RentmanClient {\n");
    output.push_str(&accessors);
    output.push_str("}\n\n");
    for (owner, methods) in collection_methods {
        writeln!(output, "impl Collection<{owner}> {{").unwrap();
        for method in methods {
            writeln!(output, "{method}").unwrap();
        }
        output.push_str("}\n\n");
    }
    output.push_str("/// Operation IDs reachable through the resource layer.\n#[doc(hidden)]\npub const REACHABLE_OPERATIONS: &[&str] = &[\n");
    for operation in reachable {
        writeln!(output, "    {operation},").unwrap();
    }
    output.push_str("] ;\n".replace(" ", "").as_str());

    let mut test_output = String::from(
        "//! Generated local-server behavior tests for resource operations.\n//!\n//! This file is generated by `cargo xtask generate-openapi --only tests`.\n\nuse crate::test_support::*;\nuse crate::{RentmanApiToken, RentmanClient, Url};\n\nfn client(server: &TestServer) -> RentmanClient {\n    RentmanClient::with_base_url(Url::parse(server.base_url()).unwrap(), RentmanApiToken::new(\"token\")).unwrap()\n}\n\n",
    );
    let mut resources = context.resources.values().cloned().collect::<Vec<_>>();
    resources.sort();
    for resource in resources {
        writeln!(test_output, "    crate::projection! {{ #[allow(dead_code)] struct {resource}IdProjection from {resource} {{ id }} }}").unwrap();
    }
    test_output.push('\n');
    for test in tests {
        writeln!(test_output, "{test}").unwrap();
    }
    Ok((output, test_output))
}

const EMPTY_PAGE: &str = "json_response(r#\"{\"data\":[],\"itemCount\":0,\"limit\":300}\"#)";
const SENT_ONCE: &str = "assert!(result.is_ok(), \"sample response decodes\");";

/// One sample wire item per response schema, built before any test so a schema
/// the generator cannot sample fails once instead of at every operation.
fn response_samples<'a>(
    context: &Context<'_>,
    operations: &'a [Operation],
) -> Result<BTreeMap<&'a str, Value>, String> {
    operations
        .iter()
        .filter_map(|operation| operation.response.schema())
        .map(|schema_name| response_item_wire(context, schema_name).map(|item| (schema_name, item)))
        .collect()
}

fn sample_with_id(samples: &BTreeMap<&str, Value>, schema_name: &str, id: u64) -> Value {
    let mut item = samples[schema_name].clone();
    item["id"] = json!(id);
    item
}

fn item_response_expression(samples: &BTreeMap<&str, Value>, schema_name: &str, id: u64) -> String {
    let data = sample_with_id(samples, schema_name, id);
    json_response_expression(&json!({"data": data}))
}

fn response_item_wire(context: &Context<'_>, schema_name: &str) -> Result<Value, String> {
    // Operations only reference schemas the context resolved.
    let schema = &context.schemas[schema_name];
    let required = context.effective_required(schema_name, schema);
    let mut item = Map::new();
    for (field, field_schema) in properties(schema) {
        let value = if let Some(example) = field_schema.get("example") {
            example.clone()
        } else if field == "custom" {
            json!({})
        } else if field == "id" {
            json!(0)
        } else {
            let rust_type = context.field_type(schema_name, field, &required);
            let rust_type = rust_type
                .strip_prefix("Option<")
                .and_then(|inner| inner.strip_suffix('>'))
                .unwrap_or(&rust_type);
            match rust_type {
                "String" => json!("string"),
                "DateTime<FixedOffset>" => json!("2019-08-24T14:15:22Z"),
                "i64" => json!(0),
                _ => sample_wire(context, rust_type, schema_name, field, field_schema)?,
            }
        };
        item.insert(field.clone(), value);
    }
    item.insert("id".to_owned(), json!(0));
    item.insert("updateHash".to_owned(), json!("test"));
    Ok(Value::Object(item))
}

fn json_response_expression(value: &Value) -> String {
    let body = serde_json::to_string(value).expect("JSON values serialize");
    format!("json_response({body:?})")
}

fn add_paged_collection_test(
    tests: &mut Vec<String>,
    name: &str,
    path: &str,
    list_call: &str,
    samples: &BTreeMap<&str, Value>,
    schema_name: &str,
) {
    let first_item = sample_with_id(samples, schema_name, 1);
    let second_item = sample_with_id(samples, schema_name, 2);
    let first_page = json!({
        "data": [first_item],
        "itemCount": 2,
        "limit": 2,
        "next_page_url": format!("{{base}}{path}?cursor=next"),
    });
    let second_page = json!({
        "data": [second_item],
        "itemCount": 1,
        "limit": 2,
        "next_page_url": null,
    });
    let first_path = path.to_owned();
    let test_name = format!("{name}_follows_pages");
    let first_response = json_response_expression(&first_page);
    let second_response = json_response_expression(&second_page);
    let mut test = String::new();
    writeln!(
        test,
        "    #[tokio::test]\n    async fn {test_name}() {{\n        let server = spawn_server(vec![{first_response}, {second_response}]);\n        let client = client(&server);\n        let items = {list_call}.page_size(crate::PageSize::new(2).unwrap()).all().await.unwrap();\n        assert_eq!(items.len(), 2);\n        let first = server.next_request();\n        assert_request(&first, \"GET\", {first_path:?});\n        assert_bearer_token(&first);\n        assert_eq!(first.query_value(\"limit\").as_deref(), Some(\"2\"));\n        assert_eq!(first.query_value(\"offset\").as_deref(), Some(\"0\"));\n        let second = server.next_request();\n        assert_request(&second, \"GET\", {first_path:?});\n        assert_bearer_token(&second);\n        assert_eq!(second.query_value(\"cursor\").as_deref(), Some(\"next\"));\n        assert_eq!(second.query_value(\"limit\"), None);\n        assert_eq!(second.query_value(\"offset\"), None);\n        server.assert_no_more_requests();\n    }}\n"
    )
    .unwrap();
    tests.push(test);
}

fn seal(impls: &mut Vec<String>, sealed: &mut BTreeSet<String>, marker: &str) {
    if sealed.insert(marker.to_owned()) {
        impls.push(format!("impl Sealed for {marker} {{}}"));
    }
}

fn test_source(
    name: &str,
    response: &str,
    call: &str,
    method: &str,
    path: &str,
    expected_body: Option<&str>,
    check: &str,
) -> String {
    let body_check = expected_body.map_or_else(String::new, |body| {
        format!(
            "        let expected_body = serde_json::to_value({body}).unwrap();\n        assert_eq!(request.json_body(), expected_body);\n"
        )
    });
    format!(
        "    #[tokio::test]\n    async fn {name}() {{\n        let server = spawn_server(vec![{response}]);\n        let client = client(&server);\n        let result = {call}.await;\n        {check}\n        let request = server.next_request();\n        assert_request(&request, \"{method}\", \"{path}\");\n        assert_bearer_token(&request);\n{body_check}        server.assert_no_more_requests();\n    }}\n"
    )
}

fn add_collection_errors(
    tests: &mut Vec<String>,
    name: &str,
    call_base: &str,
    path: &str,
    operation_id: &str,
    item_type: &str,
) {
    let name = format!("{name}_collection_errors");
    let mut test = format!(
        "    #[tokio::test]\n    async fn {name}() {{\n        let server = spawn_server(vec![text_response(\"HTTP/1.1 400 Bad Request\", \"rejected\"), text_response(\"HTTP/1.1 400 Bad Request\", \"rejected\"), text_response(\"HTTP/1.1 400 Bad Request\", \"rejected\")]);\n        let client = client(&server);\n"
    );
    let calls = [
        (
            format!("{call_base}.list().all_as::<{item_type}IdProjection>()"),
            Some("id"),
        ),
        (format!("{call_base}.list().first(1)"), None),
        (
            format!("{call_base}.list().first_as::<{item_type}IdProjection>(1)"),
            Some("id"),
        ),
    ];
    for (call, fields) in calls {
        writeln!(test, "        let error = {call}.await.unwrap_err();\n        assert_eq!(error.status().map(|status| status.as_u16()), Some(400));\n        assert_eq!(error.operation(), Some(\"{operation_id}\"));\n        let request = server.next_request();\n        assert_request(&request, \"GET\", \"{path}\");").unwrap();
        if let Some(fields) = fields {
            writeln!(test, "        assert_eq!(request.query_value(\"fields\").as_deref(), Some(\"{fields}\"));").unwrap();
        }
    }
    test.push_str("        server.assert_no_more_requests();\n    }\n");
    tests.push(test);
}

pub(crate) fn payload_expression(
    context: &Context<'_>,
    schema_name: &str,
    prefix: &str,
) -> Result<String, String> {
    let schema = &context.schemas[schema_name];
    let required = context.effective_required(schema_name, schema);
    let rust_name = context.type_name(schema_name);
    if required.is_empty() {
        return Ok(format!("{prefix}model::{rust_name}::default()"));
    }
    let mut args = Vec::new();
    for (field, field_schema) in properties(schema) {
        if required.contains(field) {
            let ty = context.field_type(schema_name, field, &required);
            args.push(sample_value(
                context,
                &ty,
                schema_name,
                field,
                field_schema,
                prefix,
            )?);
        }
    }
    Ok(format!(
        "{prefix}model::{rust_name}::new({})",
        args.join(", ")
    ))
}

/// A compile-checked rustdoc example of `body`, indented as a method doc.
fn doc_example(body: &str) -> String {
    let mut lines = vec![
        "    ///".to_owned(),
        "    /// ```".to_owned(),
        "    /// # async fn example(client: rentman_client::RentmanClient) -> rentman_client::Result<()> {".to_owned(),
    ];
    lines.extend(body.lines().map(|line| format!("    /// {line}")));
    lines.extend([
        "    /// # Ok(())".to_owned(),
        "    /// # }".to_owned(),
        "    /// ```".to_owned(),
    ]);
    lines.join("\n")
}

fn pascal(snake: &str) -> String {
    snake
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::{context, spec};
    use serde_json::json;

    fn generated() -> (String, String) {
        let spec = spec();
        let (context, operations) = context(&spec).unwrap();
        generate(&context, &operations).unwrap()
    }

    fn error(edit: impl FnOnce(&mut Value)) -> String {
        let mut spec = spec();
        edit(&mut spec);
        let (context, operations) = context(&spec).unwrap();
        generate(&context, &operations).unwrap_err()
    }

    fn no_content() -> Value {
        json!({"200": {"content": {"application/json": {"schema": {"type": "null"}}}}})
    }

    fn require_email(spec: &mut Value, schema: &str) {
        let schema = &mut spec["components"]["schemas"][schema];
        schema["properties"]["mail"] = json!({"type": "string", "format": "email"});
        schema["required"] = json!(["mail"]);
    }

    #[test]
    fn generates_every_resource_shape() {
        let (code, tests) = generated();
        for expected in [
            "impl List for Widget { type Item = Widget;",
            "impl Create for Widget { type Payload = WidgetPayload; type Item = Widget;",
            "impl Get for Widget { type Item = Widget;",
            "impl GetItem for Widget {",
            "\"id\", \"name\", \"created\", \"part\", \"status\"",
            "impl Update for Widget { type Payload = WidgetUpdatePayload; type Item = Widget;",
            "impl Delete for Widget { type Item = Widget;",
            "pub struct WidgetParts;",
            "impl List for WidgetParts { type Item = Part;",
            "impl Create for WidgetParts { type Payload = PartPayload; type Item = Part;",
            "pub fn parts(&self, widget: Id<Widget>) -> Collection<WidgetParts>",
            "const WIDGET_ARCHIVE: Operation",
            "/// Archive a widget.",
            "pub async fn archive(&self, widget: Id<Widget>, payload: &WidgetArchivePayload) -> Result<Widget>",
            "const WIDGET_BULK: Operation",
            "/// Runs `POST /widgets/:bulk`.",
            "pub async fn bulk(&self, payload: &WidgetBulkPayload) -> Result<Widget>",
            "pub fn widgets(&self) -> Collection<Widget>",
            "REACHABLE_OPERATIONS",
        ] {
            assert!(code.contains(expected), "missing {expected:?}");
        }
        assert_eq!(code.matches("pub struct WidgetParts;").count(), 1);
        for expected in [
            "async fn get_widget_collection()",
            "async fn get_widget_collection_follows_pages()",
            "async fn get_widget_collection_collection_errors()",
            "assert!(result.is_ok(), \"sample response decodes\");",
            r#"\"updateHash\":\"test\""#,
            "client.widgets().create_as::<WidgetIdProjection>(&crate::model::WidgetPayload::new(String::from(\"x\"), crate::model::WidgetPayloadStatus::Active",
            "client.widgets().update_as::<WidgetIdProjection>(crate::Id::new(7), &crate::model::WidgetUpdatePayload::default())",
            "client.widgets().delete(crate::Id::new(7))",
            "client.widgets().parts(crate::Id::new(7)).create_as::<PartIdProjection>",
            "client.widgets().archive(crate::Id::new(7), &crate::model::WidgetArchivePayload::new(String::from(\"x\")))",
            "assert_request(&request, \"POST\", \"/widgets/7/:archive\")",
            "struct PartIdProjection from Part { id }",
        ] {
            assert!(tests.contains(expected), "missing {expected:?}");
        }
    }

    #[test]
    fn response_fixtures_use_documented_examples_and_fill_nullable_values() {
        let mut spec = spec();
        spec["components"]["schemas"]["WidgetResponse"]["properties"]["name"]["example"] =
            json!("Widget sample");
        spec["components"]["schemas"]["WidgetResponse"]["properties"]["created"]["type"] =
            json!(["string", "null"]);
        let (context, _) = context(&spec).unwrap();

        let samples = BTreeMap::from([(
            "WidgetResponse",
            response_item_wire(&context, "WidgetResponse").unwrap(),
        )]);
        let item = sample_with_id(&samples, "WidgetResponse", 7);

        assert_eq!(item["id"], 7);
        assert_eq!(item["name"], "Widget sample");
        assert_eq!(item["part"], "/parts/1");
        assert_eq!(item["created"], "2019-08-24T14:15:22Z");
        assert_eq!(item["updateHash"], "test");
    }

    #[test]
    fn response_fixtures_need_a_wire_sample_for_every_field() {
        let spec = spec();
        let config = crate::test_support::config_with(|config| {
            config
                .response_type_overrides
                .push(crate::openapi::FieldOverride {
                    schema: "PartResponse".to_owned(),
                    field: "name".to_owned(),
                    rust_type: "Vec<String>".to_owned(),
                });
        });
        let context = Context::with_config(&spec, config).unwrap();
        let operations = crate::openapi::collect_operations(&context).unwrap();
        assert_eq!(
            generate(&context, &operations).unwrap_err(),
            "no sample wire value for PartResponse.name: Vec<String>"
        );
    }

    #[test]
    fn documents_each_collection_with_an_operation_it_supports() {
        let (code, _) = generated();
        for expected in [
            "    /// let first: Vec<rentman_client::model::Widget> = client.widgets().list().first(10).await?;",
            "    /// let items = client.widgets().parts(rentman_client::Id::new(12)).list().all().await?;",
            "    /// let result = client.widgets().archive(rentman_client::Id::new(12), &payload).await?;",
            "    /// let result = client.widgets().bulk(&payload).await?;",
        ] {
            assert!(code.contains(expected), "missing {expected:?}");
        }

        let mut spec = spec();
        let paths = spec["paths"].as_object_mut().unwrap();
        for path in ["/widgets/{id}/parts", "/parts"] {
            paths[path].as_object_mut().unwrap().remove("get");
        }
        let (context, operations) = context(&spec).unwrap();
        let (code, _) = generate(&context, &operations).unwrap();
        for expected in [
            "    /// let payload = rentman_client::model::PartPayload::new(String::from(\"x\"));\n    /// let created = client.widgets().parts(rentman_client::Id::new(12)).create(&payload).await?;",
            "    /// let item: rentman_client::model::Part = client.parts().get(rentman_client::Id::new(12)).await?;",
        ] {
            assert!(code.contains(expected), "missing {expected:?}");
        }
    }

    #[test]
    fn top_level_collections_need_a_known_owner_and_supported_operations() {
        assert_eq!(
            error(|spec| spec["paths"]["/ghosts"] = spec["paths"]["/parts"].clone()),
            "/ghosts links to undocumented collection /ghosts"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets"]["get"]["responses"] = no_content()),
            "collection GET has no response item"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets"]["post"]
                .as_object_mut()
                .unwrap()
                .remove("requestBody")
                .map(drop)
                .unwrap()),
            "POST has no request schema"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets"]["post"]["responses"] = no_content()),
            "POST has no response schema"
        );
        assert_eq!(
            error(|spec| require_email(spec, "WidgetRequest")),
            "no sample value for WidgetRequest.mail: EmailAddress"
        );
        assert_eq!(
            error(|spec| {
                spec["paths"]["/widgets"]["put"] = spec["paths"]["/widgets/{id}"]["put"].clone();
                spec["paths"]["/widgets"]["put"]["operationId"] = json!("replaceWidgets");
            }),
            "unsupported collection operation PUT /widgets"
        );
    }

    #[test]
    fn items_need_a_known_owner_and_a_payload_to_update() {
        assert_eq!(
            error(|spec| spec["paths"]["/ghosts/{id}"] =
                json!({"delete": spec["paths"]["/widgets/{id}"]["delete"].clone()})),
            "/ghosts/{id} links to undocumented collection /ghosts"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/{id}"]["put"]
                .as_object_mut()
                .unwrap()
                .remove("requestBody")
                .map(drop)
                .unwrap()),
            "PUT has no payload schema"
        );
        assert_eq!(
            error(|spec| require_email(spec, "WidgetUpdateRequest")),
            "no sample value for WidgetUpdateRequest.mail: EmailAddress"
        );
    }

    #[test]
    fn nested_collections_need_known_collections_and_complete_operations() {
        assert_eq!(
            error(|spec| spec["paths"]["/ghosts/{id}/parts"] =
                spec["paths"]["/widgets/{id}/parts"].clone()),
            "/ghosts/{id}/parts links to undocumented collection /ghosts"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/{id}/ghosts"] =
                spec["paths"]["/widgets/{id}/parts"].clone()),
            "/widgets/{id}/ghosts links to undocumented collection /ghosts"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/{id}/parts"]["get"]["responses"] = no_content()),
            "nested GET has no response item"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/{id}/parts"]["post"]
                .as_object_mut()
                .unwrap()
                .remove("requestBody")
                .map(drop)
                .unwrap()),
            "nested POST has no payload"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/{id}/parts"]["post"]["responses"] = no_content()),
            "nested POST has no item"
        );
        assert_eq!(
            error(|spec| require_email(spec, "PartRequest")),
            "no sample value for PartRequest.mail: EmailAddress"
        );
        assert_eq!(
            error(|spec| {
                spec["paths"]["/widgets/{id}/parts"]["put"] =
                    spec["paths"]["/widgets/{id}/parts"]["post"].clone();
                spec["paths"]["/widgets/{id}/parts"]["put"]["operationId"] =
                    json!("replaceWidgetParts");
            }),
            "unsupported nested operation PUT /widgets/{id}/parts"
        );
    }

    #[test]
    fn actions_need_a_known_owner_a_payload_and_an_item() {
        assert_eq!(
            error(|spec| spec["paths"]["/ghosts/:bulk"] = spec["paths"]["/widgets/:bulk"].clone()),
            "/ghosts/:bulk links to undocumented collection /ghosts"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/:bulk"]["post"]
                .as_object_mut()
                .unwrap()
                .remove("requestBody")
                .map(drop)
                .unwrap()),
            "action has no request schema"
        );
        assert_eq!(
            error(|spec| spec["paths"]["/widgets/:bulk"]["post"]["responses"] = no_content()),
            "action has no response schema"
        );
        assert_eq!(
            error(|spec| require_email(spec, "WidgetArchiveRequest")),
            "no sample value for WidgetArchiveRequest.mail: EmailAddress"
        );
    }

    #[test]
    fn rejects_paths_without_a_resource_shape() {
        assert_eq!(
            error(|spec| spec["paths"]["/Widgets"] = spec["paths"]["/parts"].clone()),
            "no resource shape for GET /Widgets"
        );
    }

    #[test]
    fn pascal_case_joins_snake_case_words() {
        assert_eq!(pascal("stock_movements"), "StockMovements");
        assert_eq!(pascal("a__b"), "AB");
    }
}
