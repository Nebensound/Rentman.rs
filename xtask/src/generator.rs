pub mod contract;
pub mod endpoints;
pub mod fields;
pub mod live_sweep;
pub mod models;
pub mod resources;

use crate::openapi::{Context, GeneratorConfig, collect_operations};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const OPENAPI_PATH: &str = "openapi/rentman-oas.json";

/// Rentman's API documentation page, which names the current OpenAPI document.
pub const DOCS_URL: &str = "https://api.rentman.net/";

/// Where the generator reads and writes, and how it formats what it wrote.
pub struct Settings<'a> {
    pub root: &'a Path,
    pub docs_url: &'a str,
    pub format_program: &'a str,
    pub format_args: &'a [&'a str],
    pub config: &'static GeneratorConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Group {
    Models,
    Endpoints,
    Resources,
    Fields,
    Projections,
    Tests,
}

impl Group {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "models" => Ok(Self::Models),
            "endpoints" => Ok(Self::Endpoints),
            "resources" => Ok(Self::Resources),
            "fields" => Ok(Self::Fields),
            "projections" => Ok(Self::Projections),
            "tests" => Ok(Self::Tests),
            _ => Err(format!(
                "unknown generator group {value:?}; choose models, endpoints, resources, fields, projections, or tests"
            )),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Models => "models",
            Self::Endpoints => "endpoints",
            Self::Resources => "resources",
            Self::Fields => "fields",
            Self::Projections => "projections",
            Self::Tests => "tests",
        }
    }
}

pub fn render(
    spec: &Value,
    config: &'static GeneratorConfig,
) -> Result<Vec<(PathBuf, String)>, String> {
    render_selected(spec, None, config)
}

pub fn render_selected(
    spec: &Value,
    selected: Option<Group>,
    config: &'static GeneratorConfig,
) -> Result<Vec<(PathBuf, String)>, String> {
    let context = Context::with_config(spec, config)?;
    let operations = collect_operations(&context)?;
    let mut outputs = Vec::new();
    let includes = |group| selected.is_none() || selected == Some(group);
    if includes(Group::Models) {
        outputs.push((
            PathBuf::from("src/model/generated.rs"),
            models::generate(&context)?,
        ));
    }
    if includes(Group::Endpoints) {
        outputs.push((
            PathBuf::from("src/endpoint/generated.rs"),
            endpoints::generate(&context, &operations),
        ));
    }
    if includes(Group::Resources) {
        let (code, _) = resources::generate(&context, &operations)?;
        outputs.push((PathBuf::from("src/resource/generated.rs"), code));
    }
    if includes(Group::Fields) {
        outputs.push((
            PathBuf::from("src/fields.rs"),
            fields::generate_fields(&context)?,
        ));
    }
    if includes(Group::Projections) {
        outputs.push((
            PathBuf::from("src/projection_fields.rs"),
            fields::generate_projection_fields(&context),
        ));
    }
    if includes(Group::Tests) {
        let live_sweep = live_sweep::generate(&context, &operations)?;
        let (_, resource_tests) = resources::generate(&context, &operations)?;
        outputs.extend([
            (
                PathBuf::from("src/resource/generated_tests.rs"),
                resource_tests,
            ),
            (PathBuf::from("tests/live/generated_sweep.rs"), live_sweep),
            (
                PathBuf::from("tests/contract/generated_response_enums.rs"),
                contract::response_enum_tests(&context),
            ),
            (
                PathBuf::from("tests/contract/generated_revisions.rs"),
                contract::revision_tests(&context)?,
            ),
            (
                PathBuf::from("tests/contract/generated_payload_constructors.rs"),
                contract::payload_constructor_tests(&context)?,
            ),
        ]);
    }
    Ok(outputs)
}

pub fn fetch(settings: &Settings<'_>) -> Result<(), String> {
    eprintln!("Fetching Rentman API documentation and OpenAPI spec...");
    let document = download_spec(settings.docs_url)?;
    validate_spec(&document)?;
    write_file(settings.root, Path::new(OPENAPI_PATH), &document)?;
    eprintln!("Saved {OPENAPI_PATH}.");
    Ok(())
}

pub fn generate(settings: &Settings<'_>, selected: Option<Group>) -> Result<(), String> {
    let path = settings.root.join(OPENAPI_PATH);
    let document = fs::read_to_string(&path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let spec = validate_spec(&document)?;
    eprintln!(
        "Generating OpenAPI contract ({})...",
        selected.map_or("all groups", Group::name)
    );
    let outputs = render_selected(&spec, selected, settings.config)?;
    let file_count = outputs.len();
    write_outputs(settings.root, outputs)?;
    format_generated(settings)?;
    eprintln!("Generated {file_count} files from {OPENAPI_PATH}.");
    Ok(())
}

pub fn update(settings: &Settings<'_>) -> Result<(), String> {
    eprintln!("Fetching Rentman API documentation and OpenAPI spec...");
    let document = download_spec(settings.docs_url)?;
    let spec = validate_spec(&document)?;
    eprintln!("Generating the full contract...");
    let outputs = render(&spec, settings.config)?;
    write_file(settings.root, Path::new(OPENAPI_PATH), &document)?;
    write_outputs(settings.root, outputs)?;
    format_generated(settings)?;
    eprintln!("Updated {OPENAPI_PATH} and generated the full contract.");
    Ok(())
}

fn download_spec(docs_url: &str) -> Result<String, String> {
    let documentation = reqwest::blocking::get(docs_url)
        .and_then(reqwest::blocking::Response::error_for_status)
        .and_then(reqwest::blocking::Response::text)
        .map_err(|error| format!("could not download Rentman API documentation: {error}"))?;
    let matcher = regex::Regex::new(r"Redoc\.init\('([^']+)'").expect("static regex");
    let url = matcher
        .captures(&documentation)
        .and_then(|captures| captures.get(1))
        .map(|matched| matched.as_str())
        .ok_or_else(|| "Could not find Redoc OpenAPI URL in Rentman docs".to_owned())?;
    reqwest::blocking::get(url)
        .and_then(reqwest::blocking::Response::error_for_status)
        .and_then(reqwest::blocking::Response::text)
        .map_err(|error| format!("could not download Rentman OpenAPI document: {error}"))
}

fn validate_spec(document: &str) -> Result<Value, String> {
    let spec: Value = serde_json::from_str(document)
        .map_err(|error| format!("OpenAPI document is not valid JSON: {error}"))?;
    if !spec
        .get("openapi")
        .and_then(Value::as_str)
        .is_some_and(|version| version.starts_with("3."))
    {
        return Err("OpenAPI document has no supported OpenAPI 3 version".to_owned());
    }
    Ok(spec)
}

fn write_outputs(root: &Path, outputs: Vec<(PathBuf, String)>) -> Result<(), String> {
    for (relative, content) in outputs {
        write_file(root, &relative, &content)?;
    }
    Ok(())
}

fn write_file(root: &Path, relative: &Path, content: &str) -> Result<(), String> {
    let path = root.join(relative);
    let parent = path.parent().unwrap_or(root);
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::write(&path, content)
        .map_err(|error| format!("could not write {}: {error}", path.display()))
}

fn format_generated(settings: &Settings<'_>) -> Result<(), String> {
    eprintln!("Formatting generated Rust files...");
    let program = settings.format_program;
    let status = Command::new(program)
        .args(settings.format_args)
        .current_dir(settings.root)
        .status()
        .map_err(|error| format!("could not run {program}: {error}"))?;
    if !status.success() {
        return Err(format!("{program} failed after OpenAPI generation"));
    }
    Ok(())
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::{ScratchDir, config, serve, spec};
    use serde_json::json;

    const GROUPS: [Group; 6] = [
        Group::Models,
        Group::Endpoints,
        Group::Resources,
        Group::Fields,
        Group::Projections,
        Group::Tests,
    ];

    fn settings<'a>(root: &'a Path, docs_url: &'a str) -> Settings<'a> {
        Settings {
            root,
            docs_url,
            format_program: "true",
            format_args: &[],
            config: config(),
        }
    }

    fn paths(outputs: &[(PathBuf, String)]) -> Vec<String> {
        outputs
            .iter()
            .map(|(path, _)| path.to_string_lossy().into_owned())
            .collect()
    }

    fn render_error(spec: &Value, group: Group) -> String {
        render_selected(spec, Some(group), config()).unwrap_err()
    }

    fn checked_in_spec() -> Value {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        serde_json::from_str(&fs::read_to_string(root.join(OPENAPI_PATH)).unwrap()).unwrap()
    }

    #[test]
    fn groups_parse_from_their_names() {
        for group in GROUPS {
            assert_eq!(Group::parse(group.name()), Ok(group));
        }
        assert!(Group::parse("Models").is_err());
    }

    #[test]
    fn renders_every_contract_artifact_from_the_checked_in_spec() {
        let outputs = render(&checked_in_spec(), crate::openapi::config()).unwrap();
        let paths = paths(&outputs);
        assert_eq!(paths.len(), 10);
        assert!(paths.contains(&"src/model/generated.rs".to_owned()));
        assert!(paths.contains(&"src/resource/generated_tests.rs".to_owned()));
        assert!(paths.contains(&"tests/contract/generated_revisions.rs".to_owned()));
    }

    #[test]
    fn each_group_renders_only_its_own_outputs() {
        let spec = spec();
        let expected: [&[&str]; 6] = [
            &["src/model/generated.rs"],
            &["src/endpoint/generated.rs"],
            &["src/resource/generated.rs"],
            &["src/fields.rs"],
            &["src/projection_fields.rs"],
            &[
                "src/resource/generated_tests.rs",
                "tests/live/generated_sweep.rs",
                "tests/contract/generated_response_enums.rs",
                "tests/contract/generated_revisions.rs",
                "tests/contract/generated_payload_constructors.rs",
            ],
        ];
        for (group, expected) in GROUPS.into_iter().zip(expected) {
            let outputs = render_selected(&spec, Some(group), config()).unwrap();
            assert_eq!(paths(&outputs), expected, "{group:?}");
        }
    }

    #[test]
    fn render_reports_the_first_failing_stage() {
        let mut no_paths = spec();
        no_paths.as_object_mut().unwrap().remove("paths");
        assert_eq!(
            render_error(&no_paths, Group::Models),
            "OpenAPI document has no paths"
        );

        let mut bad_status = spec();
        bad_status["paths"]["/parts"]["get"]["responses"]["oops"] = json!({});
        assert_eq!(
            render_error(&bad_status, Group::Models),
            "invalid response status \"oops\""
        );

        let mut collision = spec();
        collision["components"]["schemas"]["NoRequestResponse"] = json!({});
        assert_eq!(
            render_error(&collision, Group::Models),
            "generated type names collide: NoRequest"
        );

        let mut patch = spec();
        patch["paths"]["/widgets/{id}"]["patch"] = patch["paths"]["/widgets/{id}"]["put"].clone();
        patch["paths"]["/widgets/{id}"]["patch"]["operationId"] = json!("patchWidget");
        let expected = "unsupported item operation PATCH /widgets/{id}";
        assert_eq!(render_error(&patch, Group::Resources), expected);
        assert_eq!(render_error(&patch, Group::Tests), expected);

        let mut unfilterable = spec();
        unfilterable["components"]["schemas"]["PartResponse"]["properties"]["tags"] =
            json!({"type": "object"});
        unfilterable["components"]["schemas"]["PartResponse"]["properties"]["size"] =
            json!({"type": "string", "format": "date"});
        let config = crate::test_support::config_with(|config| {
            config.filterable_types.retain(|ty| ty != "NaiveDate");
        });
        assert_eq!(
            render_selected(&unfilterable, Some(Group::Fields), config).unwrap_err(),
            "no filter value type for PartResponse.size: NaiveDate"
        );

        let mut sweep = spec();
        sweep["paths"]["/ghosts"] = sweep["paths"]["/parts"].clone();
        assert_eq!(
            render_error(&sweep, Group::Tests),
            "/ghosts links to undocumented collection /ghosts"
        );

        let mut revision = spec();
        revision["components"]["schemas"]["PartResponse"]["properties"]["updateHash"] =
            json!({"type": "string"});
        assert_eq!(
            render_error(&revision, Group::Tests),
            "PartResponse now documents updateHash; review revision support"
        );

        let mut sample = spec();
        sample["components"]["schemas"]["UnusedRequest"] = json!({"required": ["mail"], "properties": {"mail": {"type": "string", "format": "email"}}});
        assert_eq!(
            render_error(&sample, Group::Tests),
            "no sample value for UnusedRequest.mail: EmailAddress"
        );
    }

    #[test]
    fn validates_downloaded_documents() {
        assert_eq!(
            validate_spec("{").unwrap_err().split(':').next(),
            Some("OpenAPI document is not valid JSON")
        );
        for document in [r#"{}"#, r#"{"openapi": 3}"#, r#"{"openapi": "2.0"}"#] {
            assert_eq!(
                validate_spec(document),
                Err("OpenAPI document has no supported OpenAPI 3 version".to_owned())
            );
        }
        assert_eq!(
            validate_spec(r#"{"openapi": "3.1.0"}"#),
            Ok(json!({"openapi": "3.1.0"}))
        );
    }

    #[test]
    fn download_reports_each_failing_request() {
        let failing_docs = serve(|_| vec![("/", 500, "down".to_owned())]);
        let error = download_spec(&failing_docs).unwrap_err();
        assert!(
            error.starts_with("could not download Rentman API documentation"),
            "{error}"
        );

        let no_link = serve(|_| vec![("/", 200, "<html></html>".to_owned())]);
        assert_eq!(
            download_spec(&no_link),
            Err("Could not find Redoc OpenAPI URL in Rentman docs".to_owned())
        );

        let missing_spec =
            serve(|base| vec![("/", 200, format!("Redoc.init('{base}/missing.json'"))]);
        let error = download_spec(&missing_spec).unwrap_err();
        assert!(
            error.starts_with("could not download Rentman OpenAPI document"),
            "{error}"
        );
    }

    #[test]
    fn fetch_and_update_stop_at_download_validation_and_write_errors() {
        let root = ScratchDir::new();
        let down = serve(|_| vec![("/", 500, String::new())]);
        let invalid = serve(|base| {
            vec![
                ("/", 200, format!("Redoc.init('{base}/spec.json'")),
                ("/spec.json", 200, r#"{"openapi": "2.0"}"#.to_owned()),
            ]
        });
        let not_renderable = serve(|base| {
            vec![
                ("/", 200, format!("Redoc.init('{base}/spec.json'")),
                ("/spec.json", 200, r#"{"openapi": "3.0.0"}"#.to_owned()),
            ]
        });
        let document = spec().to_string();
        let valid = serve(move |base| {
            vec![
                ("/", 200, format!("Redoc.init('{base}/spec.json'")),
                ("/spec.json", 200, document),
            ]
        });
        let unsupported = "OpenAPI document has no supported OpenAPI 3 version";
        for run in [fetch, update] {
            assert!(
                run(&settings(root.path(), &down))
                    .unwrap_err()
                    .starts_with("could not download")
            );
            assert_eq!(
                run(&settings(root.path(), &invalid)),
                Err(unsupported.to_owned())
            );
        }
        assert_eq!(
            update(&settings(root.path(), &not_renderable)),
            Err("OpenAPI document has no paths".to_owned())
        );

        fs::write(root.path().join("openapi"), "a file, not a directory").unwrap();
        for run in [fetch, update] {
            let error = run(&settings(root.path(), &valid)).unwrap_err();
            assert!(error.starts_with("could not create"), "{error}");
        }
        fs::remove_file(root.path().join("openapi")).unwrap();

        fs::write(root.path().join("src"), "a file, not a directory").unwrap();
        let error = update(&settings(root.path(), &valid)).unwrap_err();
        assert!(error.starts_with("could not create"), "{error}");
        fs::remove_file(root.path().join("src")).unwrap();

        let mut failing_format = settings(root.path(), &valid);
        failing_format.format_program = "false";
        assert_eq!(
            update(&failing_format),
            Err("false failed after OpenAPI generation".to_owned())
        );
        assert!(root.path().join("src/fields.rs").exists());
    }

    #[test]
    fn generate_stops_at_read_validation_render_write_and_format_errors() {
        let root = ScratchDir::new();
        let settings_for = |program| Settings {
            format_program: program,
            ..settings(root.path(), "http://127.0.0.1:9/")
        };
        let error = generate(&settings_for("true"), None).unwrap_err();
        assert!(error.starts_with("could not read"), "{error}");

        fs::create_dir_all(root.path().join("openapi")).unwrap();
        let spec_path = root.path().join(OPENAPI_PATH);
        fs::write(&spec_path, "{}").unwrap();
        assert_eq!(
            generate(&settings_for("true"), None),
            Err("OpenAPI document has no supported OpenAPI 3 version".to_owned())
        );
        fs::write(&spec_path, r#"{"openapi": "3.0.0"}"#).unwrap();
        assert_eq!(
            generate(&settings_for("true"), Some(Group::Models)),
            Err("OpenAPI document has no paths".to_owned())
        );

        fs::write(&spec_path, spec().to_string()).unwrap();
        fs::create_dir_all(root.path().join("src/fields.rs")).unwrap();
        let error = generate(&settings_for("true"), Some(Group::Fields)).unwrap_err();
        assert!(error.starts_with("could not write"), "{error}");
        fs::remove_dir(root.path().join("src/fields.rs")).unwrap();

        assert_eq!(
            generate(&settings_for("false"), Some(Group::Fields)),
            Err("false failed after OpenAPI generation".to_owned())
        );
        let error =
            generate(&settings_for("./no-such-formatter"), Some(Group::Fields)).unwrap_err();
        assert!(
            error.starts_with("could not run ./no-such-formatter"),
            "{error}"
        );
        assert_eq!(generate(&settings_for("true"), Some(Group::Fields)), Ok(()));
    }
}
