# rentman-client

Typed Rust client for the [Rentman](https://rentman.io) rental management API.

Every documented Rentman API operation is reachable through a typed resource:
`client.crew().list().all()` returns `Vec<Crew>`, without response envelopes,
paging, or query strings. Authentication, retry
handling, and rate limiting are built into the client, and all API I/O is
async.

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

```bash
cargo add rentman-client
```

Create a client with your Rentman API token and work with Rentman data
through its resources:

```rust
use rentman_client::{PageSize, RentmanApiToken, RentmanClient, fields, model};

# async fn example() -> rentman_client::Result<()> {
let client = RentmanClient::new(RentmanApiToken::new("token"));

// All crew members: every page, no response envelope.
let crew: Vec<model::Crew> = client.crew().list().all().await?;

// One item and a nested collection.
let crew_id = model::CrewId::new(12);
let member: model::Crew = client.crew().get(crew_id).await?;
let appointments: Vec<model::Appointment> = client
    .crew()
    .appointments(crew_id)
    .list()
    .page_size(PageSize::MAX)
    .all()
    .await?;

// Writes: `create` is sent exactly once, `update` and `delete` are retried.
let status = client
    .task_statuses()
    .create(&model::TaskStatusPayload::new("green".to_string()))
    .await?;
client.task_statuses().delete(status.id).await?;

// Typed filters, sorting, and an own field selection.
rentman_client::projection! {
    /// Name and email of a crew member.
    pub struct CrewName from Crew { id, firstname, lastname, email }
}
let names: Vec<CrewName> = client
    .crew()
    .list()
    .filter(fields::crew::ACTIVE.eq(true))
    .sort(fields::crew::LASTNAME.asc())
    .all_as::<CrewName>()
    .await?;
# Ok(())
# }
```

## What you get

- `RentmanClient` owns authentication, retry handling, rate limiting, and the
  async HTTP transport. Retries honor `Retry-After` and use exponential
  backoff; rate limiting keeps a minimum interval between requests and stays
  within Rentman's daily request budget.
- Resources: every documented collection is reachable from the client
  (`client.crew()`, nested `client.crew().appointments(id)`) and offers only
  the operations Rentman documents for it: `list()` with `all()`, `first(n)`
  and `page_size(n)`, `get`, `create`, `update`, `delete`, and named actions
  such as `client.projects().set_client(id, &payload)`. IDs are typed
  (`Id<Crew>`, alias `CrewId`), also in link fields such as `/crew/12`.
- `list()` follows Rentman's `next_page_url` through every page; filters
  (`fields::contact::COUNTRY.eq(...)`) and sorting are typed per resource and
  value type, and `projection!` declares an own struct whose fields are the
  `fields=` selection.
- `model` contains typed models for every documented schema: timestamps are
  `DateTime<FixedOffset>`, numbers are exact `RentmanAmount` decimals read
  from the JSON text (values beyond `Decimal`'s precision are rounded with a
  `tracing` warning), enums accept values Rentman adds later as `Unknown`,
  and identifiers and tokens are dedicated types — never bare strings or
  floats.
- `create` and actions are sent exactly once, because a retry after a lost
  response would write twice; `get`, `update`, `delete`, and list pages are
  retried.
- Errors are a typed `rentman_client::Error`: match on `kind()` (HTTP status,
  transport, decode, unexpected response, invalid request), read the failed
  `operation()` and `resource()`, and use `outcome()` to learn whether a
  failed write may have been applied (`NotApplied`, `Applied`, `Unknown`).

The API token is a dedicated `RentmanApiToken` type whose `Debug` output is
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

| Variable | Default | Valid values | Effect |
| --- | --- | --- | --- |
| `RENTMAN_API_MAX_ATTEMPTS` | `4` | integer ≥ 1 | Attempts per request, including the first. |
| `RENTMAN_RETRY_BASE_DELAY_MS` | `100` | integer ≥ 0 | First retry delay; backoff doubles up to 16×. |
| `RENTMAN_REQUEST_MIN_INTERVAL_MS` | `100` | integer ≥ 1 | Minimum interval between requests. |

Unset or empty variables keep the default. A variable set to an invalid value
makes `from_env()` fail with a `ConfigError` naming the variable and value; it
never falls back silently.

## License

MIT
