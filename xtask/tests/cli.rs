use std::process::Command;

#[test]
#[cfg_attr(coverage_nightly, coverage(off))]
fn cli_displays_help_and_rejects_unknown_commands() {
    let help = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("--help")
        .output()
        .expect("xtask help command should start");
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("update-openapi"));

    let unknown = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("unknown")
        .output()
        .expect("xtask unknown-command check should start");
    assert!(!unknown.status.success());
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("Usage:"));
}
