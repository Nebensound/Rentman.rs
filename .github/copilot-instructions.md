# Copilot Instructions - Rentman.rs

The README is the canonical technical documentation for humans and AI. Do not
let rules in this file drift from the README.

This repository is a typed Rust client for the Rentman API. Keep the public API
small, explicit, and close to the style used in the Nebensound workers.

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
  `RentmanApiToken`, `InvoiceId`, `PaymentId`, `InvoiceNumber`, and
  `RentmanAmount`.
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
  builder helper such as `id(model::RentmanId)` instead of making users pass raw
  strings.
- Avoid public `serde_json::Value` as an escape hatch. For intentionally open
  OpenAPI shapes, create an explicit domain type such as `CustomFields`.
- For collection pagination, follow Rentman's `next_page_url` when present.
  Keep the offset fallback only for compatibility with older responses.
- Keep authentication, retry handling, and rate limiting inside the client.
- Translate wire quirks at the boundary. For example, Rentman's invoice links
  such as `/invoices/14` should become `InvoiceId`.

## Tests And Coverage

- Unit tests for client methods should prove the outgoing HTTP request is
  correct: method, path, query parameters, authorization header, and JSON body.
- Contract tests should catch drift between the hand-written facade and the
  documented Rentman API schemas it depends on.
- Contract tests must fail when any documented endpoint, schema, request body,
  response type, enum variant, or error status is missing from the generated
  Rust contract.
- New production code requires 100% coverage, matching the Nebensound workers.
  Coverage is measured for regions, lines, and functions with `cargo-llvm-cov`
  on nightly.
- The local/CI coverage gate is:
  `cargo +nightly llvm-cov --workspace --locked --fail-under-regions 100 --fail-under-lines 100 --fail-under-functions 100`.
- Tests must assert behavior, not just execute code for the coverage counter.
  Tests without concrete result assertions are review failures.
- Test modules are excluded from production coverage with
  `#[cfg_attr(coverage_nightly, coverage(off))]`.
- Coverage exclusions are only allowed for structurally untestable production
  paths, for example process entrypoints, signal waiting, or nondeterministic
  external IO failures. Every exclusion must sit directly on the affected item
  and include a short reason.
- Do not add broad file-level exclusions for production code.
- Do not require real Rentman credentials for default tests.
- When adding a high-level client method, add both:
  - a behavior test using the local test server, and
  - an OpenAPI contract assertion for the documented operation and schema fields.
