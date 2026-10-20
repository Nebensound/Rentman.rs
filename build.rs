//! Enables live API integration tests when a local Rentman token is available.

use std::{env, path::Path};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(live_api_tests_enabled)");
    println!("cargo:rerun-if-env-changed=RENTMAN_API_TOKEN");
    println!("cargo:rerun-if-changed=.env");

    let token_in_environment =
        env::var("RENTMAN_API_TOKEN").is_ok_and(|token| !token.trim().is_empty());
    let token_in_dotenv = if Path::new(".env").exists() {
        dotenvy::from_path_iter(".env")
            .unwrap_or_else(|error| panic!("could not read .env: {error}"))
            .any(|entry| {
                let (key, value) =
                    entry.unwrap_or_else(|error| panic!("could not parse .env: {error}"));
                key == "RENTMAN_API_TOKEN" && !value.trim().is_empty()
            })
    } else {
        false
    };

    if token_in_environment || token_in_dotenv {
        println!("cargo:rustc-cfg=live_api_tests_enabled");
    }
}
