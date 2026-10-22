//! Enables live API integration tests when a local Rentman token is available.

#[path = "build/live_test_selection.rs"]
mod live_test_selection;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(live_api_tests_enabled)");
    println!("cargo:rustc-check-cfg=cfg(live_api_full_collection_tests_enabled)");
    println!("cargo:rerun-if-env-changed=RENTMAN_API_TOKEN");
    println!("cargo:rerun-if-env-changed=RENTMAN_LIVE_FULL_COLLECTIONS");
    println!("cargo:rerun-if-changed=.env");

    print!(
        "{}",
        live_test_selection::live_api_test_cfg(
            std::env::var_os("RENTMAN_API_TOKEN"),
            std::env::var_os("RENTMAN_LIVE_FULL_COLLECTIONS"),
            std::path::Path::new(".env"),
        )
    );
}
