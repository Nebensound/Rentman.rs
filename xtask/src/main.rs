#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod generator;
mod openapi;

use std::env;
use std::path::Path;
use std::process;

const USAGE: &str = "cargo xtask <command>\n\nCommands:\n  fetch-openapi                 Download the current Rentman OpenAPI spec\n  generate-openapi [--only X]   Generate all outputs or one group\n  update-openapi                Fetch the spec and generate everything\n\nGroups: models, endpoints, resources, fields, projections, tests";

enum Command {
    Help,
    Fetch,
    Generate(Option<generator::Group>),
    Update,
}

fn main() {
    let root = workspace_root();
    let settings = generator::Settings {
        root: &root,
        docs_url: generator::DOCS_URL,
        format_program: "cargo",
        format_args: &["fmt", "--all"],
        config: openapi::config(),
    };
    if let Err(error) = dispatch(env::args().skip(1).collect(), &settings) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn dispatch(args: Vec<String>, settings: &generator::Settings<'_>) -> Result<(), String> {
    let command = match args.as_slice() {
        [command] if command == "--help" || command == "-h" => Command::Help,
        [command] if command == "fetch-openapi" => Command::Fetch,
        [command] if command == "generate-openapi" => Command::Generate(None),
        [command, only, group] if command == "generate-openapi" && only == "--only" => {
            Command::Generate(Some(generator::Group::parse(group)?))
        }
        [command] if command == "update-openapi" => Command::Update,
        _ => return Err(format!("Usage: {USAGE}")),
    };
    match command {
        Command::Help => {
            println!("{USAGE}");
            Ok(())
        }
        Command::Fetch => generator::fetch(settings),
        Command::Generate(group) => generator::generate(settings, group),
        Command::Update => generator::update(settings),
    }
}

fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod test_support;

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::{dispatch, generator};
    use crate::test_support::{ScratchDir, config, serve, spec};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    fn settings<'a>(root: &'a std::path::Path, docs_url: &'a str) -> generator::Settings<'a> {
        generator::Settings {
            root,
            docs_url,
            format_program: "true",
            format_args: &[],
            config: config(),
        }
    }

    fn spec_server() -> String {
        let document = spec().to_string();
        serve(move |base| {
            vec![
                (
                    "/",
                    200,
                    format!("<script>Redoc.init('{base}/spec.json')</script>"),
                ),
                ("/spec.json", 200, document),
            ]
        })
    }

    #[test]
    fn dispatches_help_without_touching_the_workspace() {
        let root = ScratchDir::new();
        let settings = settings(root.path(), "http://127.0.0.1:9/");
        assert_eq!(dispatch(args(&["--help"]), &settings), Ok(()));
        assert_eq!(dispatch(args(&["-h"]), &settings), Ok(()));
        assert_eq!(std::fs::read_dir(root.path()).unwrap().count(), 0);
    }

    #[test]
    fn rejects_unknown_commands_and_extra_arguments() {
        let root = ScratchDir::new();
        let settings = settings(root.path(), "http://127.0.0.1:9/");
        for invalid in [
            &["unknown"][..],
            &["update-openapi", "extra"],
            &["generate-openapi", "--only"],
            &["generate-openapi", "--all", "models"],
        ] {
            let error = dispatch(args(invalid), &settings).unwrap_err();
            assert!(
                error.starts_with("Usage: cargo xtask"),
                "{invalid:?}: {error}"
            );
        }
        assert_eq!(
            dispatch(args(&["generate-openapi", "--only", "unknown"]), &settings),
            Err("unknown generator group \"unknown\"; choose models, endpoints, resources, fields, projections, or tests".to_owned())
        );
    }

    #[test]
    fn fetches_the_spec_named_by_the_documentation_page() {
        let root = ScratchDir::new();
        let docs = spec_server();
        dispatch(args(&["fetch-openapi"]), &settings(root.path(), &docs)).unwrap();
        let saved = std::fs::read_to_string(root.path().join("openapi/rentman-oas.json")).unwrap();
        assert_eq!(saved, spec().to_string());
        assert!(!root.path().join("src").exists());
    }

    #[test]
    fn generates_all_groups_or_one_from_the_saved_spec() {
        let root = ScratchDir::new();
        std::fs::create_dir_all(root.path().join("openapi")).unwrap();
        std::fs::write(
            root.path().join("openapi/rentman-oas.json"),
            spec().to_string(),
        )
        .unwrap();
        let settings = settings(root.path(), "http://127.0.0.1:9/");

        dispatch(args(&["generate-openapi", "--only", "fields"]), &settings).unwrap();
        assert!(root.path().join("src/fields.rs").exists());
        assert!(!root.path().join("src/model/generated.rs").exists());

        dispatch(args(&["generate-openapi"]), &settings).unwrap();
        let models = std::fs::read_to_string(root.path().join("src/model/generated.rs")).unwrap();
        assert!(models.contains("pub struct Widget {"));
        assert!(
            root.path()
                .join("tests/contract/generated_revisions.rs")
                .exists()
        );
    }

    #[test]
    fn updates_spec_and_every_output_in_one_run() {
        let root = ScratchDir::new();
        let docs = spec_server();
        dispatch(args(&["update-openapi"]), &settings(root.path(), &docs)).unwrap();
        assert!(root.path().join("openapi/rentman-oas.json").exists());
        let resources =
            std::fs::read_to_string(root.path().join("src/resource/generated.rs")).unwrap();
        assert!(resources.contains("pub fn widgets(&self) -> Collection<Widget>"));
    }
}
