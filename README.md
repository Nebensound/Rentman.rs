# Rentman.rs

Typed Rust client for calling the Rentman API.

The hand-written client code follows the Nebensound worker style: small domain
types, explicit API boundaries, redacted secrets, retry handling, and rate
limiting.

The public crate surface is Rentman-focused:

- `RentmanClient` owns authentication, retry handling, rate limiting, and async
  HTTP transport.
- `model` contains typed Rentman API request and response models.
- `endpoint` contains typed Rentman API endpoint definitions and endpoint
  metadata.
- High-level domain methods such as `all_invoices()` keep common workflows
  smaller and stricter than raw API calls.

The checked-in Rentman OpenAPI definition lives in `openapi/rentman-oas.json`.
It is an internal contract source, not the public abstraction. The generated
files `src/model/generated.rs` and `src/endpoint/generated.rs` are exposed as
Rentman API models and endpoints through `model` and `endpoint`.

Every documented endpoint is executable through the typed endpoint layer:

```rust
use rentman_client::{RentmanApiToken, RentmanClient, Url, endpoint, model};

# async fn example() -> anyhow::Result<()> {
let client = RentmanClient::with_base_url(
    Url::parse("https://api.rentman.net")?,
    RentmanApiToken::new("token"),
);

let response: model::CollectionResponse<model::FactuurResponse> = client
    .endpoint::<endpoint::GetFactuurCollectionEndpoint>()
    .query(&[("limit", "1500")])?
    .send()
    .await?;

let response: model::ItemResponse<model::AccessoryResponse> = client
    .endpoint::<endpoint::GetAccessoryItemEndpoint>()
    .id(model::RentmanId(42))
    .send()
    .await?;
# Ok(())
# }
```

The hand-written domain facade translates API formats into narrower domain
types such as
`RentmanApiToken`, `InvoiceId`, `PaymentId`, `InvoiceNumber`, and
`RentmanAmount`.

## Update the API contract

```bash
scripts/update_openapi.py
cargo test
```

When Rentman adds, removes, or renames endpoints, fields, schemas, enum values,
request bodies, response types, or error status codes, the contract tests fail
until the generated contract is updated and reviewed.

## Checks

Tests:

```bash
cargo test --workspace
```

Rustdoc including documentation examples:

```bash
cargo test --workspace --doc
env RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps
```

Formatting and static checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Coverage gate matching the Nebensound workers:

```bash
cargo +nightly llvm-cov --workspace --locked \
  --fail-under-regions 100 \
  --fail-under-lines 100 \
  --fail-under-functions 100
```

## Versioning

The crate version in `Cargo.toml` is a placeholder. Every CI run starts with a
`version` job that derives the semantic version from git history using
[PaulHatch/semantic-version](https://github.com/PaulHatch/semantic-version):
`v*` tags mark releases, and commit messages containing `(MAJOR)` or `(MINOR)`
bump the respective component. All later jobs (tests, coverage, release) depend
on this job and rewrite the `Cargo.toml` version before building, so published
artifacts always carry the derived version.

On pull request branches the `version` job also commits the derived version to
`Cargo.toml` on the branch itself when it changed, and dispatches a follow-up
CI run so the new head commit gets its required status checks.

A manually raised version wins: when a commit sets a higher version in
`Cargo.toml` than the derived one, that version is kept, is not overwritten,
and the release publishes it; later versions then derive from its tag.

Outside pull requests the version job cannot push a fix, so it fails when the
`Cargo.toml` version does not match the effective version; the mismatch is
then resolved on a pull request branch.

Every merge into `main` triggers the `release` job after tests and coverage
pass: it publishes the crate to crates.io (using the `CARGO_REGISTRY_TOKEN`
repository secret) and creates the matching `v*` git tag and GitHub release,
which in turn seeds the next version calculation.

## Coding Guidelines

The README is the canonical technical documentation for humans and AI. The
entire crate uses English: comments, documentation, logs, user-facing errors,
tests, scripts, generated helper text, and Rust identifiers.

Types are as narrow as reasonably possible. OpenAPI is the minimum contract, not
the upper bound of strict typing:

- API fields with a finite set of values are modeled as enums.
- Timestamps are `DateTime<FixedOffset>`, not strings.
- Money-like values use exact decimal types, never `f32` or `f64`.
- Identifiers, domain keys, URLs, and API tokens are dedicated types.
- `bool` is only acceptable for real on/off switches; domain states are modeled
  as enums.
- Conversion to and from the API representation happens at the boundary.
- New stringly typed code is a review failure.

Every line of the OpenAPI definition must be handled deliberately. Schemas,
fields, enum values, path/query parameters, request bodies, success responses,
pagination metadata, and every documented error status code must be represented
in the generated contract or in an explicit hand-written boundary type. The
public API must expose this as Rentman API models/endpoints, not as an
`openapi` user-facing module.

Every generated endpoint must be executable. The build checks that each endpoint
has a serializable request type and a deserializable response type, and unit
tests verify the shared execution path for path parameters, query parameters,
typed JSON bodies, typed JSON responses, and no-content responses.
Documented path/query parameters are represented as `EndpointParameterSpec`
metadata; clear parameter semantics also get typed builder helpers such as
`id(model::RentmanId)`.

All Rentman API I/O is async. Public API operations return futures and are used
with `.await`; production code must not add blocking HTTP clients, blocking
sleep calls, or thread-based request execution.

New production code requires 100% coverage. Regions, lines, and functions are
measured. Tests assert behavior, not the coverage counter. Test modules do not
count toward production coverage and use
`#[cfg_attr(coverage_nightly, coverage(off))]`. Exclusions are allowed only for
structurally untestable code and must be justified directly at the affected
item.
