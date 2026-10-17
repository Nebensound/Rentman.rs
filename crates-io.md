# rentman-client

Typed Rust client for the [Rentman](https://rentman.io) rental management API.

Every documented Rentman API endpoint is callable through a typed endpoint
definition with typed request and response models. Authentication, retry
handling, and rate limiting are built into the client, and all API I/O is
async.

## Getting started

```bash
cargo add rentman-client
```

Create a client with your Rentman API token and call any documented endpoint:

```rust
use rentman_client::{RentmanApiToken, RentmanClient, endpoint, model};

# async fn example() -> rentman_client::Result<()> {
let client = RentmanClient::new(RentmanApiToken::new("token"));

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

## What you get

- `RentmanClient` owns authentication, retry handling, rate limiting, and the
  async HTTP transport. Retries honor `Retry-After` and use exponential
  backoff; rate limiting keeps a minimum interval between requests and stays
  within Rentman's daily request budget.
- `endpoint` defines every documented Rentman API operation, including typed
  path and query parameter metadata and the documented error statuses.
- `model` contains typed models for every documented schema: timestamps are
  `DateTime<FixedOffset>`, money-like values are exact decimals, identifiers
  and tokens are dedicated types — never bare strings or floats.
- High-level helpers such as `all_invoices()`, `all_payments_by_invoice()`
  and `all_active_equipment()` handle pagination (`next_page_url` with offset
  fallback) for common workflows; amounts are exact `RentmanAmount` decimals
  (values beyond `Decimal`'s precision are rounded with a `tracing` warning).
- `send_once()` sends a typed request without retries, for non-idempotent
  writes where a lost response must not cause a second write.
  `create_payment()` always sends exactly once.
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
