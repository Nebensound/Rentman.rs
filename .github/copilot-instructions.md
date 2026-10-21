# Copilot Instructions - Rentman.rs

The README is the canonical technical documentation for humans and AI. Do not
let rules in this file drift from the README.

This repository is a typed Rust client for the Rentman API. Keep the public API
small and explicit. The repository is self-contained: every rule a change is
reviewed against is written down here and in the README, not in other
repositories.

## Working Rules

- The entire crate uses English: hand-written comments, documentation, logs,
  user-facing errors, tests, scripts, generated helper text, and Rust
  identifiers.
- Before non-trivial architecture changes, present a short concept and task list.
- When conventions change, update the README in the same change.

## Coding Style

- Use Rust 2024 and keep `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
  and `cargo test` clean.
- `missing_docs` is denied. Public items need useful documentation.
- Keep the library fully async for Rentman API I/O. Do not add blocking HTTP
  clients, blocking sleeps, or thread-based production request execution.
- Prefer small domain types over raw primitives at API boundaries. Examples:
  `RentmanApiToken`, `Id<R>` (e.g. `InvoiceId = Id<Invoice>`), `InvoiceNumber`,
  and `RentmanAmount`. IDs and links to the same resource share one `Id<R>`
  type; never add untyped ID or link types.
- Do not leak secrets. Token wrappers must keep `Debug` redacted.
- Use exact decimal values for payment amounts. Do not use `f32` or `f64` for
  money-like values.
- Treat the OpenAPI document as the minimum contract, not as the maximum
  strictness. If a field has clear semantics, model that semantics even when
  the OpenAPI type is only `string` or `integer`.
- Keep generated or documentation-derived data behind a hand-written facade.
  The public user-facing modules are `model` and `endpoint`; do not expose
  `openapi` as part of the ergonomic main public API.
- Add abstractions only when they remove real duplication or make API boundaries
  safer.

## Rentman API Boundary

- Follow the checked-in OpenAPI document in `openapi/rentman-oas.json`.
- Every documented OpenAPI line must be represented deliberately: schemas,
  fields, enums, path/query parameters, request bodies, success responses,
  pagination metadata, and every documented error status. If a response code is
  documented, the library must know it through `EndpointErrorSpec`.
- Refresh the OpenAPI fixture with `scripts/update_openapi.py`; do not edit
  `src/endpoint/generated.rs` or `src/model/generated.rs` by hand.
- The generated OpenAPI contract files are reviewable contract data. If Rentman
  changes documented endpoints, fields, schemas, enum values, request bodies,
  response types, or error codes, regenerate them and review the diff.
- Every endpoint must have a meaningful Rust request type and response type.
  Use `NoRequest` or `NoContent` only when the OpenAPI operation actually has
  no JSON request or response body.
- Every documented endpoint must be executable through `RentmanClient` with a
  typed request model and typed response model. The generator must keep the
  compile-time executability assertion in sync with every endpoint type.
- Every documented path/query parameter must be represented through
  `EndpointParameterSpec`. When a parameter has clear semantics, expose a typed
  builder helper such as `id(Id<R>)` instead of making users pass raw
  strings.
- Response enums are open (`Unknown(String)` keeps an undocumented value as
  sent); request enums stay closed. Every documented value must still be a
  named variant.
- Avoid public `serde_json::Value` as an escape hatch. For intentionally open
  OpenAPI shapes, create an explicit domain type such as `CustomFields`.
- Page collections only through the shared paging core in `src/paging.rs`. It
  follows Rentman's `next_page_url` unchanged until it is `null`, keeps the
  offset fallback only for older responses, and rejects a repeated or
  foreign-origin `next_page_url`. Page sizes are a validated `PageSize`.
- Filters and sorting are typed through the generated `fields` constants;
  never add raw query strings to the public API. Booleans are sent as `1`/`0`.
- Keep authentication, retry handling, and rate limiting inside the client.
- Translate wire quirks at the boundary. For example, Rentman's invoice links
  such as `/invoices/14` become `InvoiceId` through the generated link
  adapters, not through hand-written parsing.

## Tests And Coverage

- Unit tests for client methods should prove the outgoing HTTP request is
  correct: method, path, query parameters, authorization header, and JSON body.
- Contract tests should catch drift between the hand-written facade and the
  documented Rentman API schemas it depends on.
- Contract tests must fail when any documented endpoint, schema, request body,
  response type, enum variant, or error status is missing from the generated
  Rust contract.
- All hand-written production code requires 100% coverage, not only new or
  changed lines. Coverage is measured for regions, lines, and functions with
  `cargo-llvm-cov` on nightly.
- The local/CI coverage gate is:
  `cargo +nightly llvm-cov --workspace --locked --fail-under-regions 100 --fail-under-lines 100 --fail-under-functions 100`.
- Tests must assert behavior, not just execute code for the coverage counter.
  Tests without concrete result assertions are review failures.
- Test modules are excluded from production coverage with
  `#[cfg_attr(coverage_nightly, coverage(off))]`.
- Coverage exclusions are only allowed for code the instrumentation cannot
  measure structurally. "Async IO", "timers", or "hard to test" are not valid
  reasons: retries, backoff, rate limiting, and transport failures are tested
  against the local test server, with controllable timing where needed. Every
  exclusion must sit directly on the affected item and name the concrete
  reason.
- Do not add broad file-level exclusions for production code.
- Cover the happy path, error paths, boundary values, and every enum variant and
  branch of hand-written code.
- Parsers and conversions, for example exact amount parsing, additionally get
  property-based tests (`proptest`) for round trips, equivalent notations, and
  "never panics" on arbitrary input.
- Unit tests live in a `#[cfg(test)] mod tests` next to the module they test.
  Integration and contract tests live in `tests/`. Do not grow one test module
  for the whole crate.
- Assert error behavior on typed error variants, not on error message
  substrings.
- Do not require real Rentman credentials for default tests.
- When adding a high-level client method, add both:
  - a behavior test using the local test server, and
  - an OpenAPI contract assertion for the documented operation and schema fields.

## Live API Tests

- Live tests call the real Rentman API, are `#[ignore]`d, and run only in the
  separate, non-blocking live workflow or locally on demand.
- Read-only live tests may run against any account. Live tests that write must
  be opt-in through a dedicated environment variable, must skip with a clear
  message when it is unset, and must either be a verifiable no-op (write back
  the value just read) or run against a dedicated test account.
- Never create data in a production account from a live test.
- Document every live-test variable in `.env.example` and the README.

## Errors And Configuration

- The library returns its own typed, matchable error type. Public signatures do
  not expose `anyhow`. Callers must never need to compare error strings.
- Errors carry structured context (operation, resource ID, HTTP status) as
  fields, not only as text.
- For writes, the error states whether the request was certainly not applied or
  whether the outcome is unknown and the caller has to reconcile.
- Client configuration is typed and validated when the client is built. A
  present but invalid value fails with an error naming the setting and value;
  it never falls back to a default silently. Reading environment variables is
  an explicit opt-in, not hidden in a constructor.
- No `.unwrap()` in production code. `expect("reason")` only for an invariant
  that holds by construction, with the reason in the message.

## Writes And Retries

- Only idempotent requests (`GET`, `PUT`, `DELETE`) are retried automatically.
- Requests that create data (`POST`) are sent exactly once. Their docs state
  that the caller has to reconcile after an error, and which read method to use
  for it.
- Every high-level write method documents its retry behavior, and a test proves
  how many requests reach the server on a transient failure.

## Toolchain And CI

- The crate declares its MSRV in `Cargo.toml` (`rust-version`); CI builds and
  tests with exactly that toolchain. Raising the MSRV is a minor version bump.
- `unsafe` code is forbidden by a crate-level lint.
- Every cargo command in CI that resolves dependencies (build, test, clippy,
  doc, coverage, package) uses `--locked`. `cargo fmt` takes no `--locked`.
- Required pull request checks: formatting, clippy with `-D warnings`, tests and
  documentation tests, `cargo doc` with warnings as errors, the coverage gate,
  the MSRV build, and a package build with a smoke test outside the source tree.
- Every documented local check command matches what CI runs.
- Third-party actions are pinned to a commit SHA or an immutable version tag,
  never to a moving branch. Jobs get explicit, minimal `permissions`.
- The release publishes exactly the package that was verified, never with
  `--allow-dirty`.
- The daily OpenAPI update workflow uses `GITHUB_TOKEN` and the reserved
  `automation/update-openapi` branch to create or update one PR. It explicitly
  dispatches CI after PR changes; dispatched CI on a same-repository branch
  with an open PR to the default branch may commit the derived version just
  like a normal PR run. Unchanged generated files leave an open update PR alone.

## Deviations

- A deliberate deviation from these rules is documented in the README with its
  rationale, risk, and a linked follow-up issue.
- `RentmanAmount` is a wire-boundary type without a currency, because the
  Rentman API sends bare numbers. It must not be used as a general money type in
  domain logic; consumers convert it into their own currency-carrying type.
