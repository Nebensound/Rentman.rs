#!/usr/bin/env bash
# Builds the crate package and checks it from outside the source tree: a
# throwaway binary crate depends on the unpacked package contents only, is
# built in the release profile, and exercises the public API. This catches
# files missing from the package and dependencies on the source checkout.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cargo package --locked

version="$(cargo pkgid | sed 's/.*[#@]//')"
package_dir="$repo_root/target/package/rentman-client-$version"
if [ ! -d "$package_dir" ]; then
  echo "Packaged sources not found at $package_dir" >&2
  exit 1
fi

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT
cp -R "$package_dir" "$work_dir/rentman-client"
mkdir -p "$work_dir/smoke/src"

cat > "$work_dir/smoke/Cargo.toml" <<TOML
[package]
name = "rentman-client-smoke"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
rentman-client = { path = "../rentman-client" }

[workspace]
TOML

cat > "$work_dir/smoke/src/main.rs" <<'RUST'
use rentman_client::{RentmanApiToken, RentmanClient, endpoint, model};

fn main() {
    let token = RentmanApiToken::new("smoke-secret");
    assert!(
        !format!("{token:?}").contains("smoke-secret"),
        "the API token must be redacted in debug output"
    );

    // Building typed requests needs no network access.
    let client = RentmanClient::new(token);
    let _request = client
        .endpoint::<endpoint::GetAccessoryItemEndpoint>()
        .id(model::AccessoryId::new(1));

    println!("rentman-client package smoke test passed");
}
RUST

# Build with the versions that `cargo package --locked` verified: start from
# the repository lockfile, let cargo add only the smoke crate itself, and
# refuse to continue if any dependency version changed.
cp "$repo_root/Cargo.lock" "$work_dir/smoke/Cargo.lock"
cd "$work_dir/smoke"
cargo update --workspace --quiet

locked_versions() {
  awk '/^name = /{name=$3} /^version = /{print name, $3}' "$1" | sort -u
}
changed="$(comm -23 <(locked_versions Cargo.lock | grep -v '^"rentman-client-smoke" ') \
  <(locked_versions "$repo_root/Cargo.lock"))"
if [ -n "$changed" ]; then
  echo "The smoke build would use dependency versions that Cargo.lock does not pin:" >&2
  echo "$changed" >&2
  exit 1
fi

cargo run --release --locked --quiet
