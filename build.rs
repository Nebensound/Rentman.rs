//! Enables live API integration tests when a local Rentman token is available.

#[path = "build/live_test_selection.rs"]
mod live_test_selection;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(live_api_tests_enabled)");
    println!("cargo:rerun-if-env-changed=RENTMAN_API_TOKEN");
    println!("cargo:rerun-if-changed=.env");

    print!(
        "{}",
        live_test_selection::live_api_test_cfg(
            std::env::var_os("RENTMAN_API_TOKEN"),
            std::path::Path::new(".env"),
        )
    );
}
