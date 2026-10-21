use std::env;
use std::io;
use std::path::Path;
use std::process::{self, Command};

const USAGE: &str = "cargo xtask <command>\n\nCommands:\n  update-openapi  Refresh the checked-in Rentman API contract";

fn main() {
    let result = dispatch(env::args().skip(1).collect(), update_openapi);
    if let Err(error) = result {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn dispatch(args: Vec<String>, update: impl FnOnce() -> Result<(), String>) -> Result<(), String> {
    match args.as_slice() {
        [command] if command == "--help" || command == "-h" => {
            println!("{USAGE}");
            Ok(())
        }
        [command] if command == "update-openapi" => update(),
        _ => Err(format!("Usage: {USAGE}")),
    }
}

fn update_openapi() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    run_update_openapi(&workspace_root, |command| {
        command.status().map(|status| status.success())
    })
}

fn run_update_openapi(
    workspace_root: &Path,
    execute: impl FnOnce(&mut Command) -> io::Result<bool>,
) -> Result<(), String> {
    let mut command = Command::new("python3");
    command
        .args(["scripts/update_openapi.py"])
        .current_dir(workspace_root);

    match execute(&mut command) {
        Ok(true) => Ok(()),
        Ok(false) => Err("OpenAPI update script failed".to_owned()),
        Err(error) => Err(format!("Could not run OpenAPI update script: {error}")),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::{dispatch, run_update_openapi};
    use std::io;
    use std::path::Path;

    #[test]
    fn dispatches_help() {
        assert!(dispatch(vec!["--help".to_owned()], || Ok(())).is_ok());
    }

    #[test]
    fn rejects_unknown_commands_and_extra_arguments() {
        assert!(dispatch(vec!["unknown".to_owned()], || Ok(())).is_err());
        assert!(
            dispatch(
                vec!["update-openapi".to_owned(), "extra".to_owned()],
                || Ok(())
            )
            .is_err()
        );
    }

    #[test]
    fn dispatches_update_openapi() {
        let mut called = false;
        let result = dispatch(vec!["update-openapi".to_owned()], || {
            called = true;
            Ok(())
        });

        assert!(result.is_ok());
        assert!(called);
    }

    #[test]
    fn reports_update_failures() {
        let root = Path::new("workspace");
        let failed_status = run_update_openapi(root, |_| Ok(false));
        assert_eq!(
            failed_status,
            Err("OpenAPI update script failed".to_owned())
        );

        let failed_to_start = run_update_openapi(root, |_| Err(io::Error::other("missing python")));
        assert_eq!(
            failed_to_start,
            Err("Could not run OpenAPI update script: missing python".to_owned())
        );
    }

    #[test]
    fn runs_the_refresh_script_from_the_workspace_root() {
        let root = Path::new("workspace");
        let result = run_update_openapi(root, |command| {
            assert_eq!(command.get_program(), "python3");
            assert_eq!(
                command.get_args().collect::<Vec<_>>(),
                ["scripts/update_openapi.py"]
            );
            assert_eq!(command.get_current_dir(), Some(root));
            Ok(true)
        });

        assert_eq!(result, Ok(()));
    }
}
