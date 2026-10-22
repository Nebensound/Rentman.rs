use std::{ffi::OsString, path::Path};

pub(crate) fn live_api_test_cfg(
    token: Option<OsString>,
    full_collections: Option<OsString>,
    dotenv_path: &Path,
) -> String {
    let mut cfg = String::new();
    if live_api_tests_enabled(token, dotenv_path) {
        cfg.push_str("cargo:rustc-cfg=live_api_tests_enabled\n");
    }
    if live_full_collection_tests_enabled(full_collections, dotenv_path) {
        cfg.push_str("cargo:rustc-cfg=live_api_full_collection_tests_enabled\n");
    }
    cfg
}

fn live_full_collection_tests_enabled(value: Option<OsString>, dotenv_path: &Path) -> bool {
    if let Some(value) = value {
        return value.to_str().is_some_and(|value| value.trim() == "1");
    }
    let Ok(entries) = dotenvy::from_path_iter(dotenv_path) else {
        return false;
    };
    for entry in entries {
        let Ok((key, value)) = entry else {
            return false;
        };
        if key == "RENTMAN_LIVE_FULL_COLLECTIONS" {
            return value.trim() == "1";
        }
    }
    false
}

fn live_api_tests_enabled(token: Option<OsString>, dotenv_path: &Path) -> bool {
    if let Some(token) = token {
        return token.to_str().is_some_and(|token| !token.trim().is_empty());
    }
    let entries = match dotenvy::from_path_iter(dotenv_path) {
        Ok(entries) => entries,
        Err(error) if error.not_found() => return false,
        Err(_) => panic!("could not read crate-root .env"),
    };
    for entry in entries {
        let (key, value) = entry.unwrap_or_else(|_| panic!("could not parse crate-root .env"));
        if key == "RENTMAN_API_TOKEN" {
            return !value.trim().is_empty();
        }
    }
    false
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// build-script behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fixture(std::path::PathBuf);

    impl Fixture {
        fn new(contents: &str) -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "rentman-build-test-{}-{}.env",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::write(&path, contents).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_file(&self.0).unwrap();
        }
    }

    #[test]
    fn emits_only_a_boolean_cfg_directive_without_the_token() {
        let dotenv = Fixture::new("RENTMAN_API_TOKEN=dotenv-token\n");
        assert_eq!(
            live_api_test_cfg(Some("dummy-secret".into()), None, &dotenv.0),
            "cargo:rustc-cfg=live_api_tests_enabled\n"
        );
        assert_eq!(live_api_test_cfg(Some("".into()), None, &dotenv.0), "");
    }

    #[test]
    fn enables_full_collection_tests_only_for_the_opt_in_value() {
        let dotenv = Fixture::new("RENTMAN_LIVE_FULL_COLLECTIONS=1\n");
        assert_eq!(
            live_api_test_cfg(None, None, &dotenv.0),
            "cargo:rustc-cfg=live_api_full_collection_tests_enabled\n"
        );
        for value in ["", "0", "true", "  "] {
            assert_eq!(live_api_test_cfg(None, Some(value.into()), &dotenv.0), "");
        }
        assert_eq!(
            live_api_test_cfg(None, Some("1".into()), &dotenv.0),
            "cargo:rustc-cfg=live_api_full_collection_tests_enabled\n"
        );
    }

    #[test]
    fn full_collection_tests_stay_off_without_a_readable_dotenv() {
        let missing = Fixture::new("");
        assert!(!live_full_collection_tests_enabled(
            None,
            &missing.0.with_extension("missing")
        ));
        for contents in [
            "OTHER_SETTING=value\n",
            "invalid dotenv\nRENTMAN_LIVE_FULL_COLLECTIONS=1\n",
        ] {
            let dotenv = Fixture::new(contents);
            assert!(!live_full_collection_tests_enabled(None, &dotenv.0));
        }
    }

    #[test]
    fn environment_takes_precedence_even_when_empty() {
        let dotenv = Fixture::new("RENTMAN_API_TOKEN=dotenv-token\n");
        for (token, enabled) in [("environment-token", true), ("", false), ("  ", false)] {
            assert_eq!(
                live_api_tests_enabled(Some(token.into()), &dotenv.0),
                enabled
            );
        }
        let malformed = Fixture::new("invalid dotenv\n");
        assert!(live_api_tests_enabled(Some("token".into()), &malformed.0));
        assert!(!live_api_tests_enabled(Some("".into()), &malformed.0));
    }

    #[test]
    fn reads_first_dotenv_token_like_the_runtime_loader() {
        for (contents, enabled) in [
            ("OTHER_SETTING=value\nRENTMAN_API_TOKEN=token\n", true),
            ("RENTMAN_API_TOKEN='  '\n", false),
            ("RENTMAN_API_TOKEN=\nRENTMAN_API_TOKEN=second\n", false),
            ("OTHER_SETTING=value\n", false),
        ] {
            let dotenv = Fixture::new(contents);
            assert_eq!(live_api_tests_enabled(None, &dotenv.0), enabled);
        }
    }

    #[test]
    fn missing_dotenv_disables_live_tests() {
        let dotenv = Fixture::new("");
        assert!(!live_api_tests_enabled(
            None,
            &dotenv.0.with_extension("missing")
        ));
    }

    #[test]
    #[should_panic(expected = "could not read crate-root .env")]
    fn unreadable_dotenv_is_reported() {
        live_api_tests_enabled(None, Path::new("\0"));
    }

    #[test]
    fn malformed_dotenv_errors_do_not_expose_token_contents() {
        let dotenv = Fixture::new("RENTMAN_API_TOKEN=\"dummy-secret-without-closing-quote\n");
        let panic = std::panic::catch_unwind(|| live_api_tests_enabled(None, &dotenv.0))
            .expect_err("a malformed token assignment must fail");
        let message = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .expect("panic contains a string message");
        assert_eq!(message, "could not parse crate-root .env");
        assert!(!message.contains("dummy-secret"));
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_environment_does_not_fall_back_to_dotenv() {
        use std::os::unix::ffi::OsStringExt;
        let dotenv = Fixture::new("RENTMAN_API_TOKEN=dotenv-token\n");
        assert!(!live_api_tests_enabled(
            Some(OsString::from_vec(vec![0xff])),
            &dotenv.0
        ));
    }
}
