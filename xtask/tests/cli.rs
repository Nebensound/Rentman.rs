#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::process::Command;

#[test]
// Coverage is disabled for test code so the production 100% gate measures only
// generator behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
fn cli_displays_help_and_rejects_unknown_commands() {
    let help = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("--help")
        .output()
        .expect("xtask help command should start");
    assert!(help.status.success());
    let help_text = String::from_utf8_lossy(&help.stdout);
    assert!(help_text.contains("fetch-openapi"));
    assert!(help_text.contains("generate-openapi [--only X]"));
    assert!(help_text.contains("update-openapi"));
    assert!(help_text.contains("api-status [--unavailable]"));

    let unknown = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("unknown")
        .output()
        .expect("xtask unknown-command check should start");
    assert!(!unknown.status.success());
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("Usage:"));
}
