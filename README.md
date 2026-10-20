# Rentman.rs

Typed Rust client for calling the Rentman API.

## Independence and API access

This is an independent, unofficial project. It is not affiliated with,
developed by, sponsored by, or endorsed by Rentman. Its only integration
with Rentman is through Rentman's public API.

Calling the Rentman API with this client requires your own Rentman API key
(API token). Rentman makes API keys available to customers with a paid
subscription to its Software as a Service (SaaS). This project does not
provide API keys or include a Rentman subscription.

See [Rentman's API access instructions](https://support.rentman.io/hc/en-us/articles/360013767839-The-Rentman-API)
for how to generate your token in your Rentman account.

The hand-written client code favors small domain types, explicit API
boundaries, redacted secrets, retry handling, and rate limiting.

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

Every documented endpoint is executable through the typed endpoint layer.

User-facing crate documentation lives in `crates-io.md`: it is published as
the crates.io README and included as the crate-level rustdoc, so its Rust
example is compile-checked by `cargo test --doc`. This README is the
contributor documentation and stays out of the published package.

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

Where live API payloads contradict the documented schema, the generator applies
deliberate, documented overrides for response models (see the `RESPONSE_*`
tables in `scripts/generate_endpoint_manifest.py`) — for example fields that
are documented as strings but returned as integers, documented-required fields
that collection responses omit, and documented non-nullable fields that return
null. Request models always keep the documented types. The live API tests
guard these overrides against future drift.

## Checks

Tests:

```bash
cargo test --workspace --locked
```

Rustdoc including documentation examples:

```bash
cargo test --workspace --locked --doc
env RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --locked --no-deps
```

Formatting and static checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-targets -- -D warnings
```

Minimum supported Rust version (MSRV): the `rust-version` in `Cargo.toml`
(currently 1.88). CI builds and tests with exactly that toolchain:

```bash
cargo +1.88 test --workspace --locked
```

Package build and smoke test: `cargo package` builds the crate from its
packaged contents, then a throwaway binary outside the source tree depends on
the unpacked package and runs in the release profile. It needs a clean working
tree:

```bash
scripts/package_smoke_test.sh
```

Live API tests call the real Rentman API and are ignored by default. They read
`RENTMAN_API_TOKEN` from the environment or, locally, from a `.env` file in the
crate root. Generate the token in Rentman under Configuration → Integrations;
`.env` is git-ignored and must never be committed:

```bash
cp .env.example .env   # then fill in RENTMAN_API_TOKEN
cargo test --test live_api -- --ignored
```

A token already set in the environment takes precedence over `.env`, so
`RENTMAN_API_TOKEN=... cargo test --test live_api -- --ignored` keeps working.

Live tests only read, with one opt-in exception: when
`RENTMAN_LIVE_WRITE_EQUIPMENT_ID` names a dedicated, non-archived test
equipment item, a test writes its current price back unchanged through
`update_equipment_prices()` and through a typed `send_once()` request, and
checks that no price field changed. Without the variable nothing is written.
Payments are not covered, because a created payment cannot be undone without
side effects; that needs a dedicated Rentman test account. In CI the
`live-api` workflow reads the ID from the optional `RENTMAN_LIVE_WRITE_EQUIPMENT_ID`
repository variable (a variable, not a secret, because the ID is not
confidential).

Every documented `GET` collection endpoint without path parameters has its own
generated test in `tests/live/generated_sweep.rs`: each calls the endpoint
with `limit=1` and fails individually when the live payload no longer matches
the typed models — this catches drift between Rentman's documentation and the
real API per endpoint. All tests share one client so the built-in rate limiter
applies across the parallel test threads. The tests are regenerated by
`scripts/generate_endpoint_manifest.py` together with the contract files. CI
runs the live tests in the non-blocking `live-api` workflow (manual dispatch
plus a weekly schedule) using the `RENTMAN_API_TOKEN` repository secret.

Coverage gate:

```bash
cargo +nightly llvm-cov --workspace --locked \
  --fail-under-regions 100 \
  --fail-under-lines 100 \
  --fail-under-functions 100
```

Fuzzing of amount parsing (nightly, [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)):

```bash
cargo install cargo-fuzz --version 0.13.2 --locked
mkdir -p fuzz/corpus/amount
cargo +nightly fuzz run amount fuzz/corpus/amount fuzz/seeds/amount -- \
  -max_total_time=60 -dict=fuzz/amount.dict
```

The `amount` target in `fuzz/` feeds arbitrary input to `RentmanAmount`'s
deserialization. It checks that parsing never panics, that every JSON number
with an exponent up to ±1000 gives the same value as an independent
arbitrary-precision reference (`bigdecimal`), and that every accepted amount
survives a JSON round trip with value and scale. Pull requests with changes beyond documentation and crate-version-only edits
run it for 60 seconds in the `fuzz` CI job; the non-blocking `Fuzz` workflow runs it
for 30 minutes every week and on manual dispatch. CI adds
`--target x86_64-unknown-linux-gnu`, because `cargo fuzz` otherwise builds for
the triple its own binary was built for, and a musl build has no address
sanitizer. A crashing input is
uploaded as a workflow artifact and reproduced with
`cargo +nightly fuzz run amount <file>`; every finding becomes a regression
test in the library. `fuzz/` is a separate crate with its own lockfile, kept
in line with the library's `Cargo.lock`, and is not part of the published
package.

## Versioning

The crate version in `Cargo.toml` is a placeholder. Every CI run starts with a
`version` job that derives the semantic version from git history using
[PaulHatch/semantic-version](https://github.com/PaulHatch/semantic-version):
`v*` tags mark releases, and commit messages containing `(MAJOR)` or `(MINOR)`
bump the respective component. The crate stays on `0.x` for now: breaking
changes are marked `(MINOR)`, because a `0.x` minor bump is already breaking
under Cargo's semver rules, and `(MAJOR)` is not used. All later jobs (tests,
coverage, release) depend on this job and rewrite the `Cargo.toml` version
before building, so published artifacts always carry the derived version.

On pull request branches the `version` job also commits the derived version to
`Cargo.toml` on the branch itself when it changed, and dispatches a follow-up
CI run so the new head commit gets its required status checks.

A manually raised version wins: when a commit sets a higher version in
`Cargo.toml` than the derived one, that version is kept, is not overwritten,
and the release publishes it; later versions then derive from its tag.

Outside pull requests the version job cannot push a fix, so it fails when the
`Cargo.toml` version does not match the effective version; the mismatch is
then resolved on a pull request branch.

Every merge into `main` triggers the `release` job after tests, coverage, the
MSRV build, and the package smoke test pass, and the short fuzz run either
passes or is explicitly skipped for documentation/version-only changes. It
verifies the package again from the merged commit, publishes it to crates.io without `--allow-dirty` (using the
`CARGO_REGISTRY_TOKEN` repository secret), and creates the matching `v*` git
tag and GitHub release, which in turn seeds the next version calculation.

## Coding Guidelines

The README is the canonical technical documentation for humans and AI. The
entire crate uses English: comments, documentation, logs, user-facing errors,
tests, scripts, generated helper text, and Rust identifiers.

Types are as narrow as reasonably possible. OpenAPI is the minimum contract, not
the upper bound of strict typing:

- API fields with a finite set of values are modeled as enums.
- Timestamps are `DateTime<FixedOffset>`, not strings.
- Money-like values use exact decimal types, never `f32` or `f64`.
  `RentmanAmount` reads and writes JSON numbers from their exact text. An
  amount with more precision than `Decimal` holds (more than 28 decimal places
  or a mantissa beyond 96 bits) is rounded half away from zero to the nearest
  value that fits, with a `tracing` warning naming the original and the
  rounded value, so one such value cannot make a whole listing unreadable. An
  integer part that does not fit is rejected.
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

Complete collections are read through one shared paging core (`src/paging.rs`);
no other code pages by itself. It follows Rentman's `next_page_url` unchanged
until it is `null` (the cursor carries `fields`, filters, and the page size),
and requests the next offset only when an older response has a full page
without `next_page_url`. A `next_page_url` that repeats an earlier page or
leaves the configured API origin (it would receive the API token) fails with
an `UnexpectedResponse` error. Page sizes are a validated `PageSize` (1 to
1500, default 300).

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

All hand-written production code requires 100% coverage, not only new or
changed lines. Regions, lines, and functions are measured. Tests assert behavior, not the coverage counter. Test modules do not
count toward production coverage and use
`#[cfg_attr(coverage_nightly, coverage(off))]`. Exclusions are allowed only for
structurally untestable code and must be justified directly at the affected
item.

### Tests

Tests cover the happy path, error paths, boundary values, and every enum variant
and branch of hand-written code. Coverage exclusions are allowed only where the
instrumentation cannot measure the code structurally; "async IO", "timers", or
"hard to test" are not valid reasons. Retries, backoff, rate limiting, and
transport failures are tested against the local test server. Parsers and
conversions such as exact amount parsing additionally get property-based tests.
Amount parsing is also checked differentially: for generated number texts, an
independent arbitrary-precision computation (`bigdecimal`) decides the expected
value and whether it is exact or rounded, and the parser must agree.
Unit tests live next to the module they test; integration and contract tests
live in `tests/`. Error behavior is asserted on typed error variants, not on
message substrings.

Live tests are `#[ignore]`d and never required for the default checks. Live
tests that write are opt-in through a dedicated environment variable, skip
with a clear message when it is unset, and are either a verifiable no-op or run
against a dedicated test account. They never create data in a production
account.

### Errors and configuration

The library returns its own typed, matchable error type and does not expose
`anyhow` in public signatures. Errors carry structured context such as
operation, resource ID, and HTTP status. For writes, the error states whether
the request was certainly not applied or whether its outcome is unknown.

Client configuration is typed and validated when the client is built. A present
but invalid value fails with an error naming the setting; it never falls back to
a default silently. Reading environment variables is an explicit opt-in.
Production code does not use `.unwrap()`; `expect` is reserved for invariants
that hold by construction.

### Writes and retries

Only idempotent requests (`GET`, `PUT`, `DELETE`) are retried automatically.
Requests that change data do not follow redirects: a redirect answer to a
write is reported as an error whose outcome is unknown, because the first
server may already have applied it.
Requests that create data (`POST`) are sent exactly once, and their
documentation names the read method to reconcile with after an error. Every
high-level write documents its retry behavior, and a test proves how many
requests reach the server on a transient failure.

### Toolchain and CI

The crate declares its MSRV (`rust-version`) and CI builds and tests with
exactly that toolchain; raising it is a minor version bump. `unsafe` code is
forbidden by a crate-level lint. Every cargo command in CI that resolves
dependencies (build, test, clippy, doc, coverage, package) uses `--locked`;
`cargo fmt` does not resolve dependencies and takes no `--locked`.

Required pull request checks are formatting, clippy with `-D warnings`, tests
and documentation tests, `cargo doc` with warnings as errors, the coverage gate,
the MSRV build, a package build with a smoke test outside the source tree, and
the 60-second fuzz run.

The branch rules require the status contexts `test` and `coverage`. The `test`
job is an aggregate gate over `version`, `checks` (formatting, tests, rustdoc,
and clippy), `coverage`, `msrv`, `package`, `changes`, and `fuzz`. It always
runs and requires explicit success from all unconditional jobs. The `changes`
job skips fuzzing only when all changes are documentation (`.md`, `.rst`, `.txt`,
or license files outside `fuzz/`) or the crate's own version in `Cargo.toml`,
`Cargo.lock`, and `fuzz/Cargo.lock`. Files under `fuzz/` always require fuzzing,
regardless of extension, except for that lockfile's crate-version-only changes. Rust code, dependencies, build settings, CI, and other file
changes require fuzzing. Version-job dispatches compare the entire branch
against the default branch; manual default-branch runs always fuzz.
The gate accepts a skipped fuzz job only after a successful, explicit
`fuzz_required=false` decision. Failure or cancellation always blocks merging.
The scheduled live API and 30-minute fuzz workflows
remain separate, non-blocking checks. Releases also wait for this gate.

Every check command in this README matches what CI runs. Third-party actions
are pinned to a commit SHA or an immutable version tag, jobs get explicit
minimal `permissions`, and the release publishes exactly the verified package.

### Deviations

A deliberate deviation from these rules is documented here with its rationale,
risk, and a linked follow-up issue. Open deviations are tracked in
[#14](https://github.com/Nebensound/Rentman.rs/issues/14).

- `RentmanAmount` has no currency. Rentman's API sends amounts as bare numbers
  and the currency is an account setting, so the type is a wire-boundary type,
  not a money type for domain logic. Consumers convert it into their own
  currency-carrying type. Risk: mixing amounts of different currencies or
  meanings (net/gross) if it is used beyond the boundary. Decided in
  [#23](https://github.com/Nebensound/Rentman.rs/issues/23); `RentmanAmount`'s
  documentation states the same.
