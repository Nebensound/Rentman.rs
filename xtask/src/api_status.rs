use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const REGISTRY: &str = "https://crates.io/api/v1/crates/rentman-client";

pub fn run(root: &Path, available: bool) -> Result<(), String> {
    let comparison = if available {
        compare_latest(root, REGISTRY, "tar")
    } else {
        Err("The current OpenAPI refresh was unavailable".to_owned())
    };
    report(comparison)
}

fn report(comparison: Result<(String, bool), String>) -> Result<(), String> {
    let success = comparison.is_ok();
    let checked_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the check runs after the Unix epoch")
        .as_secs();
    println!("{:#}", badge(comparison, checked_at));
    if success {
        Ok(())
    } else {
        Err("Release contract status is unavailable".to_owned())
    }
}

fn badge(comparison: Result<(String, bool), String>, checked_at: u64) -> Value {
    let mut badge = json!({
        "schemaVersion": 1,
        "label": "released crate / OpenAPI",
        "message": "check unavailable",
        "color": "lightgrey",
        "checked_at_unix": checked_at,
    });
    match comparison {
        Ok((version, matches)) => {
            badge["message"] = json!(format!(
                "{version}: {}",
                if matches { "matches" } else { "differs" }
            ));
            badge["color"] = json!(if matches { "brightgreen" } else { "orange" });
            badge["release_version"] = json!(version);
        }
        Err(error) => eprintln!("Release contract check failed: {error}"),
    }
    badge
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    reqwest::blocking::Client::new()
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "Rentman.rs API contract status (github.com/Nebensound/Rentman.rs)",
        )
        .timeout(Duration::from_secs(60))
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .and_then(reqwest::blocking::Response::bytes)
        .map(|bytes| bytes.to_vec())
        .map_err(|error| format!("could not download {url}: {error}"))
}

fn parse_json(bytes: &[u8]) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("invalid JSON: {error}"))
}

fn compare_latest(root: &Path, registry: &str, tar: &str) -> Result<(String, bool), String> {
    let current = fs::read(root.join("openapi/rentman-oas.json"))
        .map_err(|error| format!("could not read the current OpenAPI contract: {error}"))?;
    let current = parse_json(&current)?;
    let metadata = parse_json(&download(registry)?)?;
    let version = metadata["crate"]["max_stable_version"]
        .as_str()
        .filter(|version| {
            regex::Regex::new(r"^\d+\.\d+\.\d+$")
                .expect("static version regex")
                .is_match(version)
        })
        .ok_or_else(|| "No stable published crate version is available".to_owned())?;
    let archive = download(&format!("{registry}/{version}/download"))?;
    let directory = root.join("target/api-status");
    fs::create_dir_all(&directory)
        .map_err(|error| format!("could not create the archive directory: {error}"))?;
    let path = directory.join("package.crate");
    fs::write(&path, archive)
        .map_err(|error| format!("could not save the released package: {error}"))?;
    let extracted = Command::new(tar)
        .arg("-xOf")
        .arg(path)
        .arg(format!("rentman-client-{version}/openapi/rentman-oas.json"))
        .output()
        .map_err(|error| format!("could not run tar: {error}"))?;
    if !extracted.status.success() {
        return Err("could not extract the released OpenAPI contract".to_owned());
    }
    let released = parse_json(&extracted.stdout)?;
    Ok((version.to_owned(), current == released))
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::{ScratchDir, serve, serve_bytes, spec};

    fn workspace() -> ScratchDir {
        let root = ScratchDir::new();
        fs::create_dir(root.path().join("openapi")).unwrap();
        fs::write(
            root.path().join("openapi/rentman-oas.json"),
            spec().to_string(),
        )
        .unwrap();
        root
    }

    fn archive(document: &str) -> Vec<u8> {
        let root = ScratchDir::new();
        let directory = root.path().join("rentman-client-1.2.3/openapi");
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("rentman-oas.json"), document).unwrap();
        let output = Command::new("tar")
            .args(["-czf", "-", "-C"])
            .arg(root.path())
            .arg("rentman-client-1.2.3")
            .output()
            .unwrap();
        assert!(output.status.success());
        output.stdout
    }

    fn registry(metadata: &str, archive: Vec<u8>) -> String {
        serve_bytes(|_| {
            vec![
                ("/", 200, metadata.as_bytes().to_vec()),
                ("/1.2.3/download", 200, archive),
            ]
        })
    }

    const METADATA: &str = r#"{"crate":{"max_stable_version":"1.2.3"}}"#;

    #[test]
    fn compares_the_packaged_contract_and_reports_drift() {
        let root = workspace();
        let server = registry(METADATA, archive(&spec().to_string()));
        assert_eq!(
            compare_latest(root.path(), &server, "tar"),
            Ok(("1.2.3".to_owned(), true))
        );
        let server = registry(METADATA, archive(r#"{"paths":{}}"#));
        assert_eq!(
            compare_latest(root.path(), &server, "tar"),
            Ok(("1.2.3".to_owned(), false))
        );
        let server = registry(METADATA, archive("not JSON"));
        assert!(
            compare_latest(root.path(), &server, "tar")
                .unwrap_err()
                .starts_with("invalid JSON:")
        );
    }

    #[test]
    fn compares_object_content_not_key_order_or_formatting() {
        let first = parse_json(br#"{"openapi":"3.0.0", "paths":{}}"#).unwrap();
        let second = parse_json(br#"{"paths":{},"openapi":"3.0.0"}"#).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn reports_download_metadata_and_archive_errors() {
        let root = workspace();
        let server = serve(|_| vec![("/", 500, "unavailable".to_owned())]);
        assert!(
            compare_latest(root.path(), &server, "tar")
                .unwrap_err()
                .starts_with("could not download")
        );
        for metadata in [
            "not JSON",
            "{}",
            r#"{"crate":{"max_stable_version":"1.2.3-beta"}}"#,
        ] {
            let server = registry(metadata, Vec::new());
            assert!(compare_latest(root.path(), &server, "tar").is_err());
        }
        let server = registry(METADATA, b"invalid archive".to_vec());
        assert_eq!(
            compare_latest(root.path(), &server, "tar"),
            Err("could not extract the released OpenAPI contract".to_owned())
        );
        let server = registry(METADATA, Vec::new());
        assert!(
            compare_latest(root.path(), &server, "missing-tar-program")
                .unwrap_err()
                .starts_with("could not run tar:")
        );
        let server = serve(|_| vec![("/", 200, METADATA.to_owned())]);
        assert!(
            compare_latest(root.path(), &server, "tar")
                .unwrap_err()
                .contains("/1.2.3/download")
        );
    }

    #[test]
    fn reports_local_filesystem_errors() {
        let root = ScratchDir::new();
        assert!(
            compare_latest(root.path(), "unused", "tar")
                .unwrap_err()
                .starts_with("could not read")
        );
        let root = ScratchDir::new();
        fs::create_dir(root.path().join("openapi")).unwrap();
        fs::write(root.path().join("openapi/rentman-oas.json"), "not JSON").unwrap();
        assert!(
            compare_latest(root.path(), "unused", "tar")
                .unwrap_err()
                .starts_with("invalid JSON:")
        );
        let root = workspace();
        fs::write(root.path().join("target"), "not a directory").unwrap();
        let server = registry(METADATA, Vec::new());
        assert!(
            compare_latest(root.path(), &server, "tar")
                .unwrap_err()
                .starts_with("could not create")
        );
        fs::remove_file(root.path().join("target")).unwrap();
        fs::create_dir_all(root.path().join("target/api-status/package.crate")).unwrap();
        let server = registry(METADATA, Vec::new());
        assert!(
            compare_latest(root.path(), &server, "tar")
                .unwrap_err()
                .starts_with("could not save")
        );
    }

    #[test]
    fn renders_matching_different_and_unknown_badges() {
        let matching = badge(Ok(("1.2.3".to_owned(), true)), 123);
        assert_eq!(matching["message"], "1.2.3: matches");
        assert_eq!(matching["color"], "brightgreen");
        assert_eq!(matching["release_version"], "1.2.3");
        assert_eq!(matching["schemaVersion"], 1);
        assert_eq!(matching["checked_at_unix"], 123);
        let different = badge(Ok(("1.2.3".to_owned(), false)), 456);
        assert_eq!(different["message"], "1.2.3: differs");
        assert_eq!(different["color"], "orange");
        let unknown = badge(Err("download failed".to_owned()), 789);
        assert_eq!(unknown["message"], "check unavailable");
        assert_eq!(unknown["color"], "lightgrey");
        assert!(unknown.get("release_version").is_none());
        assert_eq!(report(Ok(("1.2.3".to_owned(), true))), Ok(()));
        assert_eq!(report(Ok(("1.2.3".to_owned(), false))), Ok(()));
        assert_eq!(
            report(Err("unavailable".to_owned())),
            Err("Release contract status is unavailable".to_owned())
        );
    }
}
