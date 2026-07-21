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

# async fn example() -> anyhow::Result<()> {
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
- High-level helpers such as `all_invoices()` and `all_payments_by_invoice()`
  handle pagination (`next_page_url` with offset fallback) for common
  workflows.

The API token is a dedicated `RentmanApiToken` type whose `Debug` output is
redacted, so the secret does not leak into logs.

## Configuration

The client reads optional environment variables:

| Variable | Default | Effect |
| --- | --- | --- |
| `RENTMAN_API_MAX_ATTEMPTS` | `4` | Attempts per request before giving up. |
| `RENTMAN_RETRY_BASE_DELAY_MS` | `100` | Base delay for the retry backoff. |
| `RENTMAN_REQUEST_MIN_INTERVAL_MS` | `100` | Minimum interval between requests. |

## License

MIT
