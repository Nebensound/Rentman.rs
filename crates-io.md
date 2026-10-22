# rentman-client

Typed Rust client for the [Rentman](https://rentman.io) rental management API.

Every documented Rentman API operation is reachable through a typed resource:
`client.crew().list().all()` returns `Vec<Crew>`, without response envelopes,
paging, or query strings. Authentication, retry
handling, and rate limiting are built into the client, and all API I/O is
async.

Every full resource object is a **read snapshot with an ID and a revision**, not
just an identified set of fields. For example, [`Crew`](model::Crew) contains
`id: CrewId` and `revision: Revision<Crew>` alongside its data. The ID answers
_which item?_; the revision answers _which observed version of that item?_
Keep both when storing an object, so later reads can detect changes without
comparing every data field. See [objects and revisions](#change-detection).

Start with [getting started](#getting-started), then see
[reading lists](#reading-lists), [filters and sorting](#filters-and-sorting),
[writing data](#creating-updating-and-deleting), [nested collections and
actions](#nested-collections-and-actions), [field selections](#selecting-response-fields),
[error handling](#handling-errors), [custom fields](#initializing-custom-fields),
and [change detection](#change-detection).
The reference is organized into [`RentmanClient`] entry points,
[`resource`] operations, [`model`] items and payloads, [`fields`] for filters
and sorting, and [`projection!`] for smaller response types. Every item in the
reference has an example. The generated
[resource name table](model#resource-names-and-api-collections) maps Rust names
to OpenAPI schemas and actual collection paths, including `Factuur` →
`Invoice`, `Quotation` → `Quote`, `Ledger` → `LedgerCode`, and the distinct
`Rate` and `CrewRate` resources.

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

## Getting started

The client is async and runs on the [Tokio](https://tokio.rs) runtime, which
its HTTP stack requires. Add both crates:

```bash
cargo add rentman-client
cargo add tokio --features macros,rt-multi-thread
```

Create one client with your Rentman API token and reach every resource
through it. Clones of a client share its connection pool and rate limit.

```rust,no_run
use rentman_client::{RentmanApiToken, RentmanClient, model};

#[tokio::main]
async fn main() -> rentman_client::Result<()> {
    let token = std::env::var("RENTMAN_TOKEN").expect("RENTMAN_TOKEN is set");
    let client = RentmanClient::new(RentmanApiToken::new(token));

    // All crew members: every page, no response envelope.
    let crew: Vec<model::Crew> = client.crew().list().all().await?;
    println!("{} crew members", crew.len());

    // One item, addressed by its typed ID.
    let member: model::Crew = client.crew().get(model::CrewId::new(12)).await?;
    // ID and revision belong to the returned snapshot; no extra request is needed.
    let id: model::CrewId = member.id;
    let revision: &rentman_client::Revision<model::Crew> = &member.revision;
    println!("{id} is at revision {revision}");
    Ok(())
}
```

## Field names

Rust names follow Rust conventions; the client translates them to Rentman's
names when it sends or reads data. The same rules apply everywhere, so a
field has the same name in a model, a payload, and a [`projection!`]:

| Rentman's name                               | Rust field                           | Filter/sort constant               |
| -------------------------------------------- | ------------------------------------ | ---------------------------------- |
| `firstname`                                  | `firstname`                          | `fields::crew::FIRSTNAME`          |
| `VAT_code`, `linkedItemType`                 | `vat_code`, `linked_item_type`       | `fields::contact::VAT_CODE`        |
| `type`, `in`, `ref`, `break` (Rust keywords) | `r#type`, `r#in`, `r#ref`, `r#break` | `fields::contact::TYPE`            |
| `updateHash`                                 | `revision`                           | none: revisions cannot be filtered |

Resources are singular Rust types (`model::Crew`, `model::Invoice`); their
client accessors are snake_case plurals (`client.crew()`,
`client.invoices()`, `client.task_statuses()`). Each field's documentation
names the Rentman field it maps to.

```rust
use rentman_client::model::{TaskStatus, TaskStatusType};

fn is_done(status: &TaskStatus) -> bool {
    status.r#type == Some(TaskStatusType::Done)
}
```

## Reading lists

[`list()`](resource::Collection::list) starts a request; nothing is sent
until you finish it with one of these:

| Method                                                                                                  | Returns                                       | Requests                                 |
| ------------------------------------------------------------------------------------------------------- | --------------------------------------------- | ---------------------------------------- |
| [`all()`](resource::ListRequest::all)                                                                   | every item                                    | one per page, following Rentman's paging |
| [`first(n)`](resource::ListRequest::first)                                                              | at most `n` items (1 to 1500)                 | exactly one                              |
| [`revisions()`](resource::ListRequest::revisions)                                                       | IDs and revisions only                        | one per page                             |
| [`all_as::<P>()`](resource::ListRequest::all_as), [`first_as::<P>(n)`](resource::ListRequest::first_as) | your [projection](#selecting-response-fields) | as above                                 |

[`page_size`](resource::ListRequest::page_size) sets how many items each
request returns (default 300, at most 1500), not how many `all` returns.

```rust
use rentman_client::{PageSize, RentmanClient, model};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
let every_contact: Vec<model::Contact> = client.contacts().list().all().await?;
let first_ten: Vec<model::Contact> = client.contacts().list().first(10).await?;
let fewer_requests = client.contacts().list().page_size(PageSize::MAX).all().await?;
# Ok(())
# }
```

## Filters and sorting

Each queryable field has a typed constant in [`fields`], one module per
resource. Its methods are named like Rust's comparison methods; the client
translates them into Rentman's query syntax. A filter with a value of the
wrong type, or a field of another resource, does not compile.

| Rust                                                 | Matches items whose field      | Rentman query                  |
| ---------------------------------------------------- | ------------------------------ | ------------------------------ |
| `eq(v)` / `ne(v)`                                    | equals / does not equal `v`    | `field=v` / `field[neq]=v`     |
| `lt(v)` / `le(v)`                                    | is less than / at most `v`     | `field[lt]=v` / `field[lte]=v` |
| `gt(v)` / `ge(v)`                                    | is greater than / at least `v` | `field[gt]=v` / `field[gte]=v` |
| `in_range(a..b)`, `in_range(a..=b)`, `in_range(a..)` | lies in a Rust range           | the matching bounds            |
| `is_none()` / `is_some()`                            | is null / is not null          | `field[isnull]=1` / `0`        |
| `asc()` / `desc()`                                   | sorts ascending / descending   | `sort=+field` / `sort=-field`  |

```rust
use chrono::DateTime;
use rentman_client::{Decimal, RentmanClient, fields::{contact, crew, task_status}, model};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
// Equality, with booleans and enums as Rust values.
let active = client.crew().list().filter(crew::ACTIVE.eq(true)).all().await?;
let british = client.contacts().list()
    .filter(contact::COUNTRY.eq(model::ContactCountry::Gb))
    .all().await?;

// Comparisons and ranges on numbers, decimals, and timestamps.
let nearby = client.contacts().list()
    .filter(contact::DISTANCE.le(Decimal::from(50)))
    .all().await?;
let middle = client.task_statuses().list()
    .filter(task_status::ORDER.in_range(2..5))
    .all().await?;
let since = DateTime::parse_from_rfc3339("2026-10-01T00:00:00+02:00").unwrap();
let recently_changed = client.crew().list().filter(crew::MODIFIED.gt(since)).all().await?;

// Null checks, several filters (all must match), and sorting.
let filed = client.contacts().list()
    .filter(contact::FOLDER.is_some())
    .filter(contact::NAME.ne("Test".to_string()))
    .sort(contact::NAME.asc())
    .all().await?;
# Ok(())
# }
```

Filter values use the field's value type, not `Option`: booleans are sent
as `1`/`0`, IDs and links as the numeric ID, enums as their documented text,
decimals exactly, and timestamps in RFC 3339. Several sort fields can be
declared, but Rentman applies only the first across pages. Fields Rentman
marks as `GENERATED FIELD` are computed by Rentman and can be neither
filtered nor sorted on, so they have no constant.

## Creating, updating, and deleting

Every resource has a `Payload` type for writes. Payloads with required
fields have a `new` constructor; the others implement `Default`. Optional
fields start as `None`, which leaves them out of the request: `None` never
sends null or clears a stored value.

```rust
use rentman_client::{RentmanClient, model::TaskStatusPayload};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
// `color` is required, so it is a constructor argument.
let mut payload = TaskStatusPayload::new("green".to_string());
payload.name = Some("Ready".to_string());
let status = client.task_statuses().create(&payload).await?;

// Update sends only the fields that are set; the others stay unchanged.
let mut change = TaskStatusPayload::new("blue".to_string());
change.name = Some("Done".to_string());
let updated = client.task_statuses().update(status.id, &change).await?;
assert_eq!(updated.id, status.id);

client.task_statuses().delete(status.id).await?;
# Ok(())
# }
```

`get`, `update`, `delete`, and list pages are retried on transient
failures. `create` and named actions are sent exactly once, because a
retry after a lost response would write twice; if one fails, check
[`Error::outcome`] and reconcile through `list`/`get` before creating
again (see [handling errors](#handling-errors)). Writes return the full
stored item; if Rentman's write response omits fields, the client reads the
item once more, and a failure of that read reports
[`RequestOutcome::Applied`]. Each collection offers only the operations
Rentman documents for it; calling another one does not compile.

## Nested collections and actions

Collections that belong to one item, such as a crew member's appointments,
are methods on the parent collection that take the parent's ID. They return
an ordinary collection with the same `list`, `create`, and projection methods:

```rust
use rentman_client::{RentmanClient, fields, model};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
let crew_id = model::CrewId::new(12);
let appointments: Vec<model::Appointment> =
    client.crew().appointments(crew_id).list().all().await?;

let project = model::ProjectId::new(7);
let subprojects = client.projects().subprojects(project).list().all().await?;
let crew_on_project = client.projects().project_crew(project).list().first(50).await?;
let files = client.equipment().files(model::EquipmentId::new(42)).list().all().await?;
# Ok(())
# }
```

Named actions are Rentman operations beyond create/update/delete. They take
an action payload and are sent exactly once:

```rust
use chrono::NaiveDate;
use rentman_client::{RentmanClient, model};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
let project = model::ProjectId::new(7);

// Make contact 3 the client of project 7, with its contact person.
let mut set_client = model::ProjectsSetClientPayload::new(model::ContactId::new(3));
set_client.contact_person = Some(model::ContactPersonId::new(9));
let reference = client.projects().set_client(project, &set_client).await?;

// Create a project from template 5, starting on a given date.
let start = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
let from_template = model::ProjectsCreateFromTemplatePayload::new(
    model::ProjectTemplateId::new(5),
    start,
);
let created = client.projects().create_from_template(&from_template).await?;

// Set the location of a subproject.
let location = model::SubprojectsSetLocationPayload::new(model::ContactId::new(4));
client.subprojects().set_location(model::SubprojectId::new(11), &location).await?;
# Ok(())
# }
```

The [resource name table](model#resource-names-and-api-collections) lists
every collection; each accessor, nested collection, and action in the
reference has its own example.

## Selecting response fields

Declare only the fields you need; the macro supplies their Rust types and
sends exactly those names to Rentman. Fields use the same
[names as the model](#field-names). No Serde dependency is needed. Read the
full [`projection!`] reference for nullability, links, revisions, and
custom fields.

```rust
use rentman_client::{RentmanClient, fields, model};

rentman_client::projection! {
    /// Name and email of a crew member.
    pub struct CrewName from Crew { id, firstname, lastname, email }
}

rentman_client::projection! {
    /// A task status with its enum, creator link, and revision.
    pub struct StatusSummary from TaskStatus { id, name, r#type, creator, revision }
}

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
let names: Vec<CrewName> = client.crew().list()
    .filter(fields::crew::ACTIVE.eq(true))
    .sort(fields::crew::LASTNAME.asc())
    .all_as::<CrewName>()
    .await?;
let one: CrewName = client.crew().get_as::<CrewName>(model::CrewId::new(12)).await?;
let first: Vec<CrewName> = client.crew().list().first_as::<CrewName>(10).await?;

let statuses = client.task_statuses().list().all_as::<StatusSummary>().await?;
for status in &statuses {
    let kind: &model::TaskStatusType = &status.r#type;
    let creator: Option<model::CrewId> = status.creator; // link read as a typed ID
    println!("{} {kind:?} by {creator:?} at {}", status.id, status.revision);
}

// Writes can return a projection too; the payload is unchanged.
let created = client.task_statuses()
    .create_as::<StatusSummary>(&model::TaskStatusPayload::new("green".to_string()))
    .await?;
# Ok(())
# }
```

## Handling errors

Match typed variants, not error strings. Errors expose the operation,
resource, HTTP status, and write outcome. `Unknown` means a write may have
been applied; `Applied` means it was acknowledged, even though later
processing failed.

```rust
use rentman_client::{ErrorKind, RentmanClient, RequestOutcome, model};

# async fn example(client: RentmanClient) -> rentman_client::Result<()> {
match client.crew().get(model::CrewId::new(12)).await {
    Ok(member) => println!("found {}", member.id),
    Err(error) if error.status().is_some_and(|status| status.as_u16() == 404) => {
        println!("Crew member not found");
    }
    Err(error) => {
        match error.kind() {
            ErrorKind::CustomField(problem) => println!("{problem}"),
            ErrorKind::Transport(_) => println!("network failure after retries"),
            _ => println!("{:?} failed for {:?}: {error}", error.operation(), error.resource()),
        }
        if error.outcome() == RequestOutcome::Unknown {
            println!("Reconcile before repeating a create operation");
        }
        return Err(error);
    }
}
# Ok(())
# }
```

## API values and Rust types

Work with the Rust types below directly. The client handles Rentman's JSON
representation; you do not need to parse resource links, format query
booleans, or convert monetary amounts through floating point yourself.

| Rust type                                                          | How to use it                                                                                                                                                                                                                                          | Rentman's representation                                                                                               |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| [`Id<R>`](Id), e.g. [`CrewId`](model::CrewId)                      | Pass to the matching resource's `get`, `update`, or `delete`. An ID identifies an item; it does not contain or automatically fetch it. Different resources have incompatible ID types.                                                                 | An item's ID is a JSON integer; a reference to it can be a path such as `/crew/12`. Both become the same Rust ID type. |
| [`ItemId`]                                                         | Use with the accompanying item-type field for references that can identify different resource kinds. Unlike `Id<R>`, it does not identify the resource by its Rust type.                                                                               | A numeric ID whose resource is specified separately.                                                                   |
| [`Decimal`]                                                        | Construct from integers or decimal strings, calculate without binary floats, and use for amounts and other decimal quantities. At most 28 decimal places are retained; additional places are rounded and out-of-range values rejected.                 | A JSON number, read from its exact text without an intermediate `f64`.                                                 |
| [`chrono::DateTime<chrono::FixedOffset>`]                          | Use Chrono's date/time operations; the value retains its UTC offset. Add `chrono` to your application's dependencies when naming or constructing these types.                                                                                          | A timestamp string.                                                                                                    |
| [`chrono::NaiveDate`]                                              | A calendar date with no time or timezone.                                                                                                                                                                                                              | A date string.                                                                                                         |
| [`Url`]                                                            | Use URL accessors such as `host_str` or `as_str`; construct with `Url::parse`.                                                                                                                                                                         | A URL string parsed by the client.                                                                                     |
| [`EmailAddress`](model::EmailAddress)                              | A dedicated email value; use its `as_str` accessor. An empty address means no address is set; this is not proof that a mailbox exists or is deliverable.                                                                                               | A string, possibly empty. Non-empty values must contain `@`.                                                           |
| Enums in [`model`], e.g. [`TaskStatusType`](model::TaskStatusType) | Match named variants. Include `Unknown(String)` when matching response enums so new values are retained. Request enums accept only their listed variants. The enum reference lists each wire value.                                                    | A documented string or numeric enum value.                                                                             |
| `Option<T>`                                                        | Handle `None` explicitly. In full models, non-core fields may be absent; projections may require explicitly selected keys. In write payloads, `None` leaves an optional field out rather than sending a null. Each field documents its presence rules. | A value can be absent or JSON null; these are not interchangeable for required keys.                                   |
| [`Revision<R>`](Revision)                                          | Compare opaque revisions to detect changes; do not parse them as counters or timestamps. See [change detection](#change-detection).                                                                                                                    | The `updateHash` string, exposed as `revision` on full models and projections.                                         |
| [`CustomFields`](model::CustomFields)                              | Register your account's definitions at initialization, then use `get::<T>` and `set` for application values. See the [complete setup](#initializing-custom-fields).                                                                                    | A `custom` object containing `custom_<id>` keys.                                                                       |
| `String`, `i64`, `bool`                                            | Ordinary Rust text, signed integers, and booleans. Typed filters accept the same underlying value type; boolean filters are formatted automatically.                                                                                                   | JSON strings, integers, and booleans; query booleans use `1`/`0`.                                                      |

Fields documented as `GENERATED FIELD` hold values Rentman computes. They
are read like any other field, but cannot be filtered or sorted on, and some
appear in list responses only when selected with a [`projection!`].

### Typed IDs and exact amounts

An ID alias is the same type as `Id<Resource>`, so an ID read from a link
can be passed straight to that resource. Construct decimal values without
floating-point literals:

```rust
use rentman_client::{Decimal, Id, model};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let crew_id: Id<model::Crew> = model::CrewId::new(12);
assert_eq!(crew_id.get(), 12);
assert_eq!(model::CrewId::from_link("/crew/12")?, crew_id);
let unit_price: Decimal = "12.50".parse()?;
let total = unit_price * Decimal::from(3);
assert_eq!(total, Decimal::new(3750, 2));
# Ok(())
# }
```

### Response enum values

Response enum variants are a local, typed vocabulary, including values
that Rentman may add later. Keeping the unknown text lets an application
report or store it without losing the rest of the item:

```rust
use rentman_client::model::TaskStatusType;

let status = TaskStatusType::Unknown("future-status".to_string());
let label = match status {
    TaskStatusType::Unknown(value) => value,
    known => format!("{known:?}"),
};
assert_eq!(label, "future-status");
```

Field descriptions, selectable values, payload requirements, resource names,
and examples are part of this crate's reference. Consult the Rust item you
are using rather than translating an OpenAPI schema by hand. Where Rentman
supplies no business description, the reference states the known type and
validation behavior; it does not invent meanings, units, or enum choices.

## Initializing custom fields

Custom fields belong to your Rentman account, not to the fixed API schema.
Register the fields your application uses **before reading the affected
resources**. Registration configures local response validation; it does not
create or modify field definitions in Rentman.

A definition combines a resource type, the numeric **custom-field definition
ID** (not an equipment or crew item ID), and Rentman's
[`ExtraInputFieldType`](model::ExtraInputFieldType). Definition `8` corresponds
to the JSON key `custom_8`. IDs in these examples are illustrative: substitute
the definitions from your account.

```rust
use rentman_client::{
    CustomFieldDefinition, CustomFieldRegistry, RentmanApiToken, RentmanClient, model,
};

# fn example() -> Result<(), Box<dyn std::error::Error>> {
let custom_fields = CustomFieldRegistry::new([
    CustomFieldDefinition::new::<model::Equipment>(8, model::ExtraInputFieldType::Text),
    CustomFieldDefinition::new::<model::Equipment>(9, model::ExtraInputFieldType::Price),
    CustomFieldDefinition::new::<model::Crew>(12, model::ExtraInputFieldType::Bool),
])?;
let client = RentmanClient::new(RentmanApiToken::new("token"))
    .with_custom_fields(custom_fields);

let equipment = client.equipment();
# let _ = equipment;
# Ok(())
# }
```

[`CustomFieldRegistry::new`] rejects duplicate `(resource, definition ID)`
pairs. The same ID may be registered for different resources.
[`RentmanClient::with_custom_fields`] replaces the client's registry rather
than merging it; pass all definitions together. It also works on a client
created with [`RentmanClient::with_config`]. Neither constructor discovers
custom fields automatically or loads their definitions from the environment.

### Finding your account's definitions

If you do not already know the definitions, first use an unconfigured client
to read [`RentmanClient::extra_input_fields`]. Inspect each definition's `id`,
`type` and `itemtype` to identify its value type and owning resource, then
build the registry and attach it to that client. The API model may omit
metadata, and response enums can contain unknown values: check these rather
than guessing a resource association.

```rust
use rentman_client::{RentmanApiToken, RentmanClient};

# async fn example() -> rentman_client::Result<()> {
let client = RentmanClient::new(RentmanApiToken::new("token"));
let definitions = client.extra_input_fields().list().all().await?;
for definition in definitions {
    println!("{definition:#?}");
}
// Use the verified definitions to build and attach the registry shown above.
# Ok(())
# }
```

For example, after verifying the target definition ID in this account,
validate the metadata and attach the registry before reading equipment.
Rentman's item-type enum uses its own names, such as `Materiaal`, rather
than the crate's `Equipment` resource name. This example assumes the account's
verified equipment definition has item type `Materiaal`:

```rust
use rentman_client::{CustomFieldDefinition, CustomFieldRegistry, RentmanApiToken, RentmanClient, model};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = RentmanClient::new(RentmanApiToken::new("token"));
let definitions = client.extra_input_fields().list().all().await?;
let definition = definitions.iter()
    .find(|definition| definition.id == model::ExtraInputFieldId::new(8))
    .ok_or("Expected custom-field definition 8 is absent")?;
if definition.itemtype != Some(model::ExtraInputFieldItemtype::Materiaal)
    || definition.r#type != Some(model::ExtraInputFieldType::Text)
{
    return Err("Definition 8 is not an equipment text field".into());
}
let client = client.with_custom_fields(CustomFieldRegistry::new([
    CustomFieldDefinition::new::<model::Equipment>(8, model::ExtraInputFieldType::Text),
])?);
let equipment_id = model::EquipmentId::new(42);
let equipment = client.equipment().get(equipment_id).await?;
let custom = equipment.custom.ok_or("Equipment response has no custom object")?;
let label: Option<String> = custom.get(8)?;
println!("Equipment label: {label:?}");

let mut changed = custom;
changed.set(8, &"Warehouse B")?;
let mut payload = model::EquipmentPayload::default();
payload.custom = Some(changed);
let updated = client.equipment().update(equipment_id, &payload).await?;
# Ok(())
# }
```

You can instead keep verified definitions in application configuration;
there is no metadata request when constructing a registry. The API does not
provide the allowed choices for enum custom fields.

### Validation and Rust values

Without registrations for a resource, its custom values are not retained.
With registrations, every registered `custom_<id>` key must be present in
the returned `custom` object; unregistered keys are removed. A present JSON
`null` is accepted for every field type and differs from a missing key.
Read nullable values as `Option<T>`.

| Rentman field type                           | Accepted non-null JSON value   | Typical application Rust type                                        |
| -------------------------------------------- | ------------------------------ | -------------------------------------------------------------------- |
| `Bool`                                       | Boolean                        | `bool`                                                               |
| `Int`                                        | Number with no fractional part | An integer type with sufficient range                                |
| `Double`, `Price`                            | Number                         | [`Decimal`]                                                          |
| `Text`, `Longtext`, `Html`, `Color`, `Phone` | String                         | `String`                                                             |
| `Date`, `Datetime`, `Url`                    | String                         | `String`, or an application Serde type                               |
| `Enum`                                       | String                         | `String`, or an application Serde enum                               |
| `Item`, `Unknown(_)`                         | Any JSON shape                 | [`CustomFieldValue`](model::CustomFieldValue) or an application type |

Registry validation checks JSON categories, not date/URL syntax or allowed
enum choices. `Item` and unknown types have no additional shape validation.
Numbers are stored as `Decimal`, with the same precision limits described
in the [type table](#api-values-and-rust-types); converting to an application integer can still fail on range limits.
Application-defined Serde types require your own `serde` dependency.

[`CustomFields::get`](model::CustomFields::get) deserializes a retained value
into your chosen Rust type. [`CustomFields::set`](model::CustomFields::set)
only modifies the local object; it does not send an HTTP request:

```rust
use rentman_client::{CustomFieldValueError, model::CustomFields};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut custom = CustomFields::default();
custom.set(8, &"Warehouse A")?;
assert_eq!(custom.get::<String>(8)?, "Warehouse A");

custom.set(8, &Option::<String>::None)?;
assert_eq!(custom.get::<Option<String>>(8)?, None);
assert!(matches!(custom.get::<String>(99), Err(CustomFieldValueError::Missing { id: 99 })));

let mut payload = rentman_client::model::EquipmentPayload::default();
payload.custom = Some(custom);
// Send this payload with client.equipment().update(equipment_id, &payload).
assert!(payload.custom.is_some());
# Ok(())
# }
```

For a fetched item, first handle its optional `custom` member, then call
`get::<T>(id)` on that object. Failed application conversions return
[`CustomFieldValueError::Json`]. Response validation errors are available as
[`ErrorKind::CustomField`], with [`CustomFieldError::MissingValue`] or
[`CustomFieldError::TypeMismatch`]; registry duplicates return
[`CustomFieldError::DuplicateDefinition`] during initialization.

In a [`projection!`] select `custom` as a whole, never an individual
`custom_8` field. Registered custom fields are requested and validated even
when the projection does not expose `custom`. Narrow projections therefore
do not bypass the registry's requirements.

## Change detection

### Objects carry their version

Every **full resource model** returned by `get`, list reads, or full-result
writes includes `id: Id<R>` and `revision: Revision<R>` as well as its data.
You do not opt in, wrap the object, or request its revision separately.
Rentman supplies the revision as `updateHash`; the crate exposes it as
`item.revision`. If the required revision is missing or empty, decoding fails
rather than returning an unversioned full object.

The ID stays the same when an item is edited; its revision changes. Together,
the ID and revision identify the observed version of that item. This makes
it possible to reuse an unchanged stored object, avoid unnecessary processing
or database updates, and fetch full data only for new or changed items.

Revisions are opaque: compare them for equality **for the same item ID**.
They are not timestamps, ordered version numbers, or globally unique IDs.
They describe the state read from Rentman, not a live object: changing a Rust
field locally does not update its revision. Nor does a revision automatically
make a write conditional or prevent concurrent edits.

```rust
use rentman_client::{Revisioned, model};

fn same_observed_version(previous: &model::Crew, current: &model::Crew) -> bool {
    previous.id() == current.id() && previous.revision() == current.revision()
}
```

The [`Revisioned`] trait exposes this pair generically; every full resource
model implements it. A bare `Id<R>` or linked ID is only an identity, not a
versioned object. Write payloads describe changes to send, not read snapshots.
Custom [`projection!`] structs contain only declared fields: select `id` and
`revision` explicitly when the projection needs both.

### Synchronizing stored snapshots

[`Revisions<R>`](Revisions) is a compact ID-to-revision index. Build it from
previously stored full objects with `stored.iter().collect()`, or load saved
ID/revision pairs. You can persist a revision with [`Revision::as_str`] and
restore it with [`Revision::new`]. Use an empty index on the first sync.

`list().revisions()` reads the same ID/revision information across every page
without loading full item bodies. `changes_since` compares that current index
with the previous one:

```rust
use rentman_client::{RentmanClient, Revisioned, Revisions, model};

# async fn example(client: RentmanClient, stored: Vec<model::Crew>) -> rentman_client::Result<()> {
// The full objects stored by the previous successful sync already have revisions.
let known: Revisions<model::Crew> = stored.iter().collect();

let current = client.crew().list().revisions().await?;
let changes = current.changes_since(&known);
for id in changes.created.iter().chain(&changes.updated) {
    let member = client.crew().get(*id).await?;
    // Store `member` and `member.revision()` …
}
for id in &changes.deleted {
    // … and remove what is gone.
}
# Ok(())
# }
```

`created` means an ID is new to this listing; `updated` means an existing
ID's revision changed; `deleted` means an ID left the listing. An ID leaving
a filtered listing may only have stopped matching the filter, not been
deleted from Rentman. Compare complete listings with the same filters and
account. These are differences between reads, not a history of every edit
or an atomic snapshot of an account being modified concurrently. Only advance
your stored sync state after the necessary reads and local writes succeed.

The comparison also works entirely offline:

```rust
use rentman_client::{Revision, Revisions, model::CrewId};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut previous = Revisions::new();
previous.insert(CrewId::new(12), Revision::new("version-a")?);
previous.insert(CrewId::new(13), Revision::new("old-item")?);

let mut current = Revisions::new();
current.insert(CrewId::new(12), Revision::new("version-b")?);
current.insert(CrewId::new(14), Revision::new("new-item")?);

let changes = current.changes_since(&previous);
assert_eq!(changes.created, vec![CrewId::new(14)]);
assert_eq!(changes.updated, vec![CrewId::new(12)]);
assert_eq!(changes.deleted, vec![CrewId::new(13)]);
# Ok(())
# }
```

## What you get

- `RentmanClient` owns authentication, retry handling, rate limiting, and the
  async HTTP transport. Retries honor `Retry-After` and use exponential
  backoff; rate limiting keeps a minimum interval between requests and stays
  within Rentman's daily request budget.
- Every documented collection, nested collection, and named action is
  reachable from the client and offers only the operations Rentman documents
  for it.
- Models are typed throughout: IDs, links, timestamps, exact decimals, and
  enums that keep values Rentman adds later. Response models require the core
  `id`, `updateHash`, `created`, `modified`, and `creator` keys; `id` and
  `updateHash` are non-null. Other declared response fields are optional, and
  unknown response keys are ignored. Request payloads enforce their documented
  required fields.
- Registered custom fields are checked against their Rentman types;
  unregistered custom keys are removed from responses.
- The API token is a dedicated `RentmanApiToken` type whose `Debug` output is
  redacted, so the secret does not leak into logs.

## Configuration

`RentmanClient::new` uses the defaults below and reads nothing from the
environment. Pass a `RentmanClientConfig` to `RentmanClient::with_config` to
change them, built in code or read explicitly with
`RentmanClientConfig::from_env()`:

```rust
use rentman_client::{RentmanApiToken, RentmanClient, RentmanClientConfig, Url};

# fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = RentmanClient::with_config(
    Url::parse("https://api.rentman.net")?,
    RentmanApiToken::new("token"),
    RentmanClientConfig::from_env()?,
)?;
# Ok(())
# }
```

| Variable                          | Default | Valid values | Effect                                        |
| --------------------------------- | ------- | ------------ | --------------------------------------------- |
| `RENTMAN_API_MAX_ATTEMPTS`        | `4`     | integer ≥ 1  | Attempts per request, including the first.    |
| `RENTMAN_RETRY_BASE_DELAY_MS`     | `100`   | integer ≥ 0  | First retry delay; backoff doubles up to 16×. |
| `RENTMAN_REQUEST_MIN_INTERVAL_MS` | `100`   | integer ≥ 1  | Minimum interval between requests.            |

Unset or empty variables keep the default. A variable set to an invalid value
makes `from_env()` fail with a `ConfigError` naming the variable and value; it
never falls back silently.

## License

MIT
