# Contributing

Thanks for helping improve `rentman-client`.

## Before you start

- For bugs and feature requests, open an issue with the matching form first.
- Security problems are reported privately, see [SECURITY.md](SECURITY.md).
- Larger architecture changes start with a short concept and task list in the
  issue.

## Conventions

The rules every change is reviewed against are written down in this
repository, in the README section "Coding Guidelines" and in
[`.github/copilot-instructions.md`](.github/copilot-instructions.md). The most
important ones:

- 100% region, line, and function coverage for hand-written code; coverage
  exclusions only for code the instrumentation cannot measure.
- Tests assert concrete behavior and typed error variants, never just run code.
- Standard tests never need real Rentman credentials.
- Do not edit the generated OpenAPI files by hand; regenerate them (see the
  README section "Update the API contract").
- Everything in the crate is written in English.

## Local checks

Run the same checks as CI before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo test --workspace --locked
env RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --locked --no-deps
cargo +nightly llvm-cov --workspace --locked --fail-under-regions 100 --fail-under-lines 100 --fail-under-functions 100
```

CI also builds and tests with the minimum supported Rust version from
`rust-version` in `Cargo.toml`, and smoke tests the packaged crate outside the
source tree (needs a clean working tree):

```bash
cargo +1.88 test --workspace --locked
scripts/package_smoke_test.sh
```

Live API tests are optional and need a token; see the README section "Checks".

## Pull requests and versions

- Keep pull requests small and focused; describe problem, solution, and test
  evidence (the pull request template asks for them).
- The version is derived from commit messages: include `(MINOR)` for new
  features and `(MAJOR)` for breaking changes in the pull request title, which
  becomes the squash commit message. Everything else is a patch release.
- Every merge into `main` publishes a release to crates.io.
