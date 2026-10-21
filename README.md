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
  HTTP transport, and is the entry point to every resource (`client.crew()`).
- `resource` contains the collections and their operations (`List`, `Get`,
  `Create`, `Update`, `Delete`, `ListRequest`).
- `model` contains the item types (`Crew`), write payloads (`CrewPayload`),
  enums, and ID aliases (`CrewId`).
- `fields` contains the filter and sort constants, `projection!` and
  `projection_fields` the field selection.
- `Id<R>`, `Decimal`, `PageSize`, and the typed `Error`.

The resource layer is the only public API for Rentman data. The endpoint layer
(`src/endpoint.rs` and `src/endpoint/generated.rs`: one typed endpoint per
documented operation with its parameters, body, response, and error statuses)
and the response envelopes (`CollectionResponse`, `ItemResponse`) are the
crate-internal foundation the resources are built on; compile-fail doctests in
`src/resource.rs` keep them private. The checked-in Rentman OpenAPI definition
lives in `openapi/rentman-oas.json`; it is an internal contract source, not
the public abstraction.

Every documented operation is reachable through the resource layer (`src/resource.rs`): the generator groups
the documented paths into collections (`client.crew()`), nested collections
(`client.crew().appointments(id)`), and named actions
(`client.projects().set_client(id, &payload)`). Accessors are the snake_case
plural of the item type (`project_equipment_groups`, `time_registrations`;
`xtask/field-rules.toml` keeps `crew` and `equipment` singular). A
collection gets only the operation traits (`List`, `Get`, `Create`, `Update`,
`Delete`) that Rentman documents for it, so an undocumented operation does not
compile. All request logic is hand-written in `src/resource.rs`; the generated
`src/resource/generated.rs` contains only trait impls, accessors, and one
local-server test per operation that checks method and path.
`resource::REACHABLE_OPERATIONS` is compared with the OpenAPI document by a
contract test. `create` and actions are sent exactly once; `get`, `update`,
`delete` and every list page are retried.

Lists take typed filters and sorting built from the generated field
constants in `fields` (`src/fields.rs`, one module per resource):
`.filter(contact::COUNTRY.eq(ContactCountry::Gb))`,
`.filter(contact::DISTANCE.lte(Decimal::from(300)))`,
`.filter(contact::FOLDER.is_null(false))`, `.sort(contact::NAME.asc())`. A
`Field<R, V>` only accepts values of type `V` and only filters lists of `R`
(compile-fail doctests in `src/filter.rs`). Values are rendered as Rentman
expects them (checked live on 2026-10-21): booleans as `1`/`0` (Rentman
silently ignores `true`/`false`), IDs and links as the numeric ID, timestamps
as RFC 3339, enums as their documented text. Fields marked `GENERATED FIELD`
and the `custom` object get no constant; `fields::QUERYABLE_FIELDS` is
compared with the OpenAPI document by a contract test. Rentman carries filters
through `next_page_url`; with several sort fields only the first applies
across pages.

Projections select exactly the fields a struct declares:
`projection! { pub struct CrewName from Crew { id, firstname, email } }`
declares the struct by field names only; the field types come from the
generated model through one marker per field in `projection_fields`
(`projection_fields::Crew::firstname`, `r#type` for `type`), and
`all_as::<CrewName>()`, `first_as` and `get_as` send exactly those names as
`fields=`. An unknown field does not compile (compile-fail doctest in
`src/projection.rs`). It is a `macro_rules!` macro: the expansion decodes field
by field through the hand-written `ProjectedItem::take`, so consumers need no
serde dependency and link fields are read through the same adapters as the
models. Rentman always adds `id`, `created`, `modified`, and `updateHash` to a
selection; `custom` appears when the account defines custom fields. Projections
ignore fields they do not declare. When the client has a custom-field
registry, the resource layer requests `custom` and validates registered keys.

User-facing crate documentation lives in `crates-io.md`: it is published as
the crates.io README and included as the crate-level rustdoc, so its Rust
example is compile-checked by `cargo test --doc`. This README is the
contributor documentation and stays out of the published package.

Hand-written boundary types translate API formats into narrower types such as
`RentmanApiToken` and `Id<R>`. Every OpenAPI number in generated models,
payloads, and projections uses `Decimal`; its arbitrary-precision JSON Serde
integration reads and writes the exact number text without an `f64` conversion.
Values in the open `custom` fields object also use `Decimal`. A number with
more than 28 decimal places is rounded to 28 without a warning; a number whose
digits do not fit `Decimal`'s 96-bit mantissa, or an exponent beyond its scale,
is rejected. For response models, only the core fields `id`, `updateHash`, `created`, `modified`, and `creator` are
required. The first two are non-null; the last three may be null but their
keys must be present. Other declared response fields are optional because
Rentman omits them from field selections and some responses. Unknown response
fields are ignored and are not re-emitted when a model is serialized. Request
payloads still follow the OpenAPI request schema's required fields.

Custom-field definitions are account-specific. The `ExtraInputField` resource
exposes each field's ID, `type`, and `itemtype`, so callers can interpret
`custom_<id>` values at runtime. Enum definitions do not include their allowed
values; `CustomFields` therefore keeps values as open JSON-compatible data.
Register the fields used by the application when building the client; only
those keys are retained and validated, and each must be present in returned
`custom` data:

```rust
use rentman_client::{CustomFieldDefinition, CustomFieldRegistry, RentmanApiToken, RentmanClient, model};

let custom_fields = CustomFieldRegistry::new([
  CustomFieldDefinition::new::<model::Equipment>(8, model::ExtraInputFieldType::Text),
])?;
let client = RentmanClient::new(RentmanApiToken::new("token"))
  .with_custom_fields(custom_fields);
```

Every Resource model carries `revision: Revision<R>` (`updateHash` on the
wire). `client.equipment().list().revisions().await?` reads a paginated
ID-to-revision snapshot without loading full item bodies; compare snapshots
with `changes_since`.

`create` and `update` return a full resource. If a write response omits fields,
the client follows it with the resource's GET endpoint. If that GET fails after
a successful write, the error outcome is `Applied`. `create_as` and `update_as`
return only the requested projection and fetch only when selected fields are
missing from the write response.

IDs are typed by their resource: `Id<R>` (`src/id.rs`) is used for an item's
own `id` and for every link to it, with a generated alias per resource
(`CrewId = Id<Crew>`, `InvoiceId = Id<Invoice>`, ...). An `id` field is a
number on the wire and a link field a path such as `/crew/12`; the generator
types each link field from its documented example and attaches a serde
adapter, so reading a link to the wrong collection fails. Integer ID fields
without such an example (for example `template` or the IDs of the
`:setClient` actions) are typed through `xtask/schema-names.toml`,
and the generator refuses an `ID of ...` field that is missing there.
Polymorphic `item` fields, whose collection is named by `itemtype`, are an
`ItemId`. Errors name the affected item as a `ResourceRef` (`/payments/7`).
Write payloads with required fields get a generated `new` taking exactly those
fields; payloads without required fields implement `Default`.

## Update the API contract

```bash
cargo xtask update-openapi
cargo test --workspace --locked
```

`update-openapi` is the normal path: it downloads the current document and
regenerates every group. `fetch-openapi` and `generate-openapi` are optional
commands for separating those steps; `generate-openapi` uses the checked-in
document without network access, and `--only` accepts `models`, `endpoints`,
`resources`, `fields`, `projections`, or `tests`. The `tests` group includes the
resource HTTP tests, live sweep, and generated contract tests. Each command
prints a short status for its current phase.

The `Update OpenAPI` workflow runs daily at 05:37 UTC and can also be started
manually in GitHub Actions. It runs `cargo xtask update-openapi` on the default
branch and, when the contract changes, creates or updates a pull request from
the fixed `automation/update-openapi` branch. An open update PR is reused;
unchanged generated files do not cause another commit or CI run. Keep this
branch reserved for automation.

The workflow uses the built-in `GITHUB_TOKEN`; no additional secret is needed.
Under **Settings > Actions > General > Workflow permissions**, enable
**Allow GitHub Actions to create and approve pull requests** (organization
policy must also allow it). The workflow only creates PRs; it does not approve
or merge them. Since token-created PRs and pushes do not trigger PR workflows,
the updater explicitly dispatches `ci.yml` after creating or updating a PR.
The normal CI checks must pass before merging; a failed download or generation
fails the update workflow without publishing a partial update.

When Rentman adds, removes, or renames endpoints, fields, schemas, enum values,
request bodies, response types, or error status codes, the contract tests fail
until the generated contract is updated and reviewed.

Generated models carry resource names, not schema names: a response schema
drops its suffix (`CrewResponse` → `Crew`) and a request schema becomes a
write payload (`CrewRequest` → `CrewPayload`); generated enums follow the new
type name (`CrewCountry`). Schemas whose name does not match their collection
are mapped in `xtask/schema-names.toml` (`FactuurResponse` →
`Invoice`, `CrewRateResponse` → `Rate` for `/rates`, `CrewRatesResponse` →
`CrewRate` for `/crewrates`, `QuotationResponse` → `Quote`, `LedgerResponse`
→ `LedgerCode`, ...). `model::SCHEMA_TYPES` lists every schema with its Rust
type; the contract tests require exactly one type per schema. Endpoint
metadata (`EndpointRequestSpec`, `EndpointResponseSpec`) and the override
tables keep the OpenAPI schema names.

Where live API payloads contradict the documented schema, the generator applies
deliberate, documented overrides for response models (see
`xtask/response-overrides.toml`) — for example fields that
are documented as strings but returned as integers, documented-required fields
that collection responses omit, and documented non-nullable fields that return
null. Request models always keep the documented types. The live API tests
guard these overrides against future drift.

`xtask/schema-names.toml`, `xtask/response-overrides.toml`, and
`xtask/field-rules.toml` contain the generator's explicit schema mappings,
observed response deviations, and field classification rules. The generator
validates schema- and field-specific entries against the OpenAPI fixture and
fails on stale or misspelled entries.

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

Generated files must match the checked-in OpenAPI document. CI regenerates them
without downloading anything and fails when the working tree then differs:

```bash
cargo xtask generate-openapi
git status --porcelain
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

Live API tests call the real Rentman API. They are ignored by default, but a
normal `cargo test` automatically runs them when `RENTMAN_API_TOKEN` is set in
the environment or in a `.env` file in the crate root. Generate the token in
Rentman under Configuration → Integrations; `.env` is git-ignored and must
never be committed:

```bash
cp .env.example .env   # then fill in RENTMAN_API_TOKEN
cargo test
```

A token already set in the environment takes precedence over `.env`, even
when it is empty (an empty or whitespace-only token disables automatic live
tests). Only the crate-root `.env` is loaded, not files in parent directories.
Live tests still load `.env` for optional settings when an environment token is
present. If loading fails, they issue a sanitized warning and continue with that
token; without an environment token, a loading failure is fatal.
Cargo rebuilds the test binary when token availability changes. Without a token, the
live tests remain ignored and regular `cargo test` does not call the API. The
CI test workflow does not receive the token; live tests remain in the separate,
non-blocking workflow below.

The build script deliberately watches `.env` even when the file is absent.
Cargo then reruns the build script on each invocation and may rebuild dependent
targets unnecessarily, including in CI where tokens come from environment
variables rather than a `.env` file. We accept this performance cost so that
creating, changing, or removing `.env` refreshes live-test selection on the next
Cargo invocation without a manual clean. This is an incremental-build cost,
not a background file watcher or a change to which tests should run.
A lowest-priority follow-up tracks an optimization that preserves automatic
detection: [#61](https://github.com/Nebensound/Rentman.rs/issues/61).

Live tests only read, with one opt-in exception: when
`RENTMAN_LIVE_WRITE_EQUIPMENT_ID` names a dedicated, non-archived test
equipment item, a test reads its prices through a projection, writes the current price back
unchanged through `client.equipment().update(...)`, and checks against the
stored JSON text that no price field changed. Without the variable nothing is written.
Payments are not covered, because a created payment cannot be undone without
side effects; that needs a dedicated Rentman test account. In CI the
`live-api` workflow reads the ID from the optional `RENTMAN_LIVE_WRITE_EQUIPMENT_ID`
repository variable (a variable, not a secret, because the ID is not
confidential).

Every documented top-level collection has its own generated test in
`tests/live/generated_sweep.rs`: each reads the first item through the
resource layer (`client.crew().list().first(1)`) and fails individually when
the live payload no longer matches the typed models — this catches drift between Rentman's documentation and the
real API per endpoint. All tests share one client so the built-in rate limiter
applies across the parallel test threads. The tests are regenerated with
`cargo xtask generate-openapi --only tests` together with the contract files. CI
runs the live tests in the non-blocking `live-api` workflow (manual dispatch
plus a weekly schedule) using the `RENTMAN_API_TOKEN` repository secret.

Coverage gate:

```bash
cargo +nightly llvm-cov --workspace --locked \
  --fail-under-regions 100 \
  --fail-under-lines 100 \
  --fail-under-functions 100
```

## Migrating to the resource layer

The resource layer replaces the endpoint API (`client.endpoint::<E>()`,
`RentmanEndpointRequest`, `CollectionResponse`, `ItemResponse`) and the
hand-written helpers. Their equivalents:

```rust,ignore
use rentman_client::{PageSize, fields::equipment, model, projection};

// all_active_equipment(): a typed filter and a projection of the price fields.
projection! {
    pub struct EquipmentPrices from Equipment { id, price, list_price, subrental_costs }
}
let active: Vec<EquipmentPrices> = client
    .equipment()
    .list()
    .filter(equipment::IN_ARCHIVE.eq(false))
    .page_size(PageSize::MAX)
    .all_as()
    .await?;

// update_equipment_prices(id, &payload): `update` with the generated payload
// (only the fields that are set are sent; PUT is retried).
let payload = model::EquipmentPayload { price: Some(amount), ..Default::default() };
client.equipment().update(equipment_id, &payload).await?;

// all_invoices() / all_payments_by_invoice(): projections; group as needed.
projection! { pub struct InvoiceRow from Invoice { id, number, is_paid, price_invat } }
projection! { pub struct PaymentRow from Payment { id, invoice, moment, amount, description } }
let invoices: Vec<InvoiceRow> = client.invoices().list().all_as().await?;
let mut payments: std::collections::HashMap<model::InvoiceId, Vec<PaymentRow>> = Default::default();
for payment in client.payments().list().all_as::<PaymentRow>().await? {
    payments.entry(payment.invoice).or_default().push(payment);
}

// update_payment(id, &payload) / create_payment(invoice, &payload): `update`,
// and the nested `create`, which is sent exactly once.
client.payments().update(payment_id, &payment_payload).await?;
client.invoices().payments(invoice_id).create(&payment_payload).await?;
```

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
Manually dispatched CI runs on same-repository branches with an open PR to the
default branch use the same version-commit behavior, including the automated
OpenAPI update branch. Dispatches without such a PR still verify the committed
version without pushing a fix.

A manually raised version wins: when a commit sets a higher version in
`Cargo.toml` than the derived one, that version is kept, is not overwritten,
and the release publishes it; later versions then derive from its tag.

Outside pull requests the version job cannot push a fix, so it fails when the
`Cargo.toml` version does not match the effective version; the mismatch is
then resolved on a pull request branch.

Every merge into `main` triggers the `release` job after tests, coverage, the
MSRV build, and the package smoke test pass. It
verifies the package again from the merged commit, publishes it to crates.io without `--allow-dirty` (using the
`CARGO_REGISTRY_TOKEN` repository secret), and creates the matching `v*` git
tag and GitHub release, which in turn seeds the next version calculation.

## Coding Guidelines

The README is the canonical technical documentation for humans and AI. The
entire crate uses English: comments, documentation, logs, user-facing errors,
tests, scripts, generated helper text, and Rust identifiers.

Types are as narrow as reasonably possible. OpenAPI is the minimum contract, not
the upper bound of strict typing:

- API fields with a finite set of values are modeled as enums. Enums in
  response models are open: an undocumented value deserializes to
  `Unknown(String)` and serializes back unchanged, so a value Rentman adds
  later does not fail a whole collection. Request enums stay closed, because
  the client only sends documented values. A generated contract test per
  response enum (`tests/contract/generated_response_enums.rs`) fails when a
  documented value has no named variant.
- Timestamps are `DateTime<FixedOffset>`, not strings.
- Money-like values use exact decimal types, never `f32` or `f64`.
  Numeric fields use `Decimal` directly and read/write JSON numbers exactly
  through `serde-arbitrary-precision`, never through `f64`. More than 28
  decimal places are rounded silently; digits beyond the 96-bit mantissa are
  rejected.
- Identifiers, domain keys, URLs, and API tokens are dedicated types.
- `bool` is only acceptable for real on/off switches; domain states are modeled
  as enums.
- Conversion to and from the API representation happens at the boundary.
- New stringly typed code is a review failure.

Every line of the OpenAPI definition must be handled deliberately. Schemas,
fields, enum values, path/query parameters, request bodies, success responses,
pagination metadata, and every documented error status code must be represented
in the generated contract or in an explicit hand-written boundary type. The
public API exposes this as resources, models, and field constants, not as an
`openapi` or endpoint module.

Complete collections are read through one shared paging core (`src/paging.rs`);
no other code pages by itself. It follows Rentman's `next_page_url` unchanged
until it is `null` (the cursor carries `fields`, filters, and the page size),
and requests the next offset only when an older response has a full page
without `next_page_url`. A `next_page_url` that repeats an earlier page or
leaves the configured API origin (it would receive the API token) fails with
an `UnexpectedResponse` error. Page sizes are a validated `PageSize` (1 to
1500, default 300).

Every documented operation must be reachable through a resource; a contract
test compares `resource::REACHABLE_OPERATIONS` with the OpenAPI document. The
internal endpoint metadata records every documented path/query parameter
(`EndpointParameterSpec`), body, response, and error status; its contract test
lives next to it in `src/endpoint.rs` because the types are crate-internal.
Path parameters are typed in the resource methods (`get(Id<R>)`,
`appointments(Id<Crew>)`).

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
transport failures are tested against the local test server. Handwritten
parsers and conversions get property-based tests. Numeric JSON parsing and
serialization use `rust_decimal`'s
arbitrary-precision Serde implementation; a model-payload test verifies exact
digits survive a JSON round trip.
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
write method of the resource layer documents its retry behavior, and a test proves how many
requests reach the server on a transient failure.

### Toolchain and CI

The crate declares its MSRV (`rust-version`) and CI builds and tests with
exactly that toolchain; raising it is a minor version bump. `unsafe` code is
forbidden by a crate-level lint. Every cargo command in CI that resolves
dependencies (build, test, clippy, doc, coverage, package) uses `--locked`;
`cargo fmt` does not resolve dependencies and takes no `--locked`.

Required pull request checks are formatting, up-to-date generated files,
clippy with `-D warnings`, tests and documentation tests, `cargo doc` with
warnings as errors, the coverage gate, the MSRV build, and a package build with
a smoke test outside the source tree.

The branch rules require the status contexts `test` and `coverage`. The `test`
job is an aggregate gate over `version`, `checks` (formatting, generated files,
tests, rustdoc, and clippy), `coverage`, `msrv`, and `package`. It always runs and requires
explicit success from each job. The scheduled live API workflow remains a
separate, non-blocking check. Releases also wait for this gate.

Every check command in this README matches what CI runs. Third-party actions
are pinned to a commit SHA or an immutable version tag, jobs get explicit
minimal `permissions`, and the release publishes exactly the verified package.

### Deviations

A deliberate deviation from these rules is documented here with its rationale,
risk, and a linked follow-up issue. Open deviations are tracked in
[#14](https://github.com/Nebensound/Rentman.rs/issues/14).
