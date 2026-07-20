//! Client for calling the Rentman API.
//!
//! The crate encapsulates authentication, pagination, rate limits, retry
//! handling, and the JSON formats used by the Rentman API. The public facade
//! uses narrow domain types following the Nebensound conventions: tokens are
//! wrapper types, IDs are newtypes, money-like values are exact decimals, and
//! wire formats are translated at the API boundary.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod endpoint;
pub mod model;

use anyhow::{Context, Result, anyhow};
use backon::{BackoffBuilder, ExponentialBuilder};
use chrono::{DateTime, FixedOffset, SecondsFormat, Utc};
use governor::{Quota, RateLimiter};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
/// Re-export of the URL type used for client configuration.
pub use reqwest::Url;
use reqwest::{
    Client as HttpClient, Method, StatusCode,
    header::{HeaderMap, RETRY_AFTER},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::HashMap, fmt, marker::PhantomData, num::NonZeroU32, sync::Arc, time::Duration,
};
use url::form_urlencoded;

const DEFAULT_RENTMAN_MIN_INTERVAL: Duration = Duration::from_millis(100);
const DEFAULT_PAGE_LIMIT: usize = 1500;

type DirectRateLimiter = RateLimiter<
    governor::state::NotKeyed,
    governor::state::InMemoryState,
    governor::clock::DefaultClock,
>;

/// HTTP client for calling the Rentman API.
#[derive(Clone)]
pub struct RentmanClient {
    base_url: Url,
    token: RentmanApiToken,
    http: HttpClient,
    limiter: Arc<CompositeRateLimiter>,
    retry_policy: RetryPolicy,
}

/// Request builder for a typed Rentman API endpoint.
pub struct RentmanEndpointRequest<E: endpoint::Endpoint> {
    client: RentmanClient,
    path_params: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body: Option<Value>,
    endpoint: PhantomData<E>,
}

/// API token for Rentman.
///
/// This is a dedicated type instead of a `String` so the token cannot be mixed
/// up with unrelated strings or tokens. `Debug` is redacted so the token does
/// not accidentally end up in logs.
#[derive(Clone)]
pub struct RentmanApiToken(String);

impl RentmanApiToken {
    /// Creates a Rentman API token.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Debug for RentmanApiToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RentmanApiToken(***)")
    }
}

/// Internal Rentman invoice ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(transparent)]
pub struct InvoiceId(u64);

impl InvoiceId {
    /// Creates a Rentman invoice ID.
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl fmt::Display for InvoiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Internal Rentman payment ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
pub struct PaymentId(u64);

impl PaymentId {
    /// Creates a Rentman payment ID.
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Rentman invoice number.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct InvoiceNumber(String);

impl InvoiceNumber {
    /// Creates a Rentman invoice number.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the invoice number as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InvoiceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Payment status of a Rentman invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(from = "bool")]
pub enum InvoicePaymentStatus {
    /// The invoice is paid.
    Paid,
    /// The invoice is open.
    Open,
}

impl From<bool> for InvoicePaymentStatus {
    fn from(is_paid: bool) -> Self {
        if is_paid { Self::Paid } else { Self::Open }
    }
}

/// Import source for Rentman payments written by this client.
///
/// Rentman knows additional values. The domain facade deliberately writes only
/// the values modeled here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentImportSource {
    /// Payment was imported through the public API.
    PublicApi,
    /// Payment is disabled or has no import source.
    None,
}

/// Exact decimal amount for Rentman payment payloads.
///
/// Rentman expects payment amounts as a bare number without a currency field.
/// This type makes the API boundary explicit and prevents floating point values
/// for money-like amounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RentmanAmount(Decimal);

impl RentmanAmount {
    /// Creates a Rentman amount from an exact decimal value.
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    /// Returns the exact decimal value.
    pub fn as_decimal(self) -> Decimal {
        self.0
    }
}

impl From<Decimal> for RentmanAmount {
    fn from(value: Decimal) -> Self {
        Self::new(value)
    }
}

impl Serialize for RentmanAmount {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&self.0, serializer)
    }
}

/// Rentman invoice with the fields needed for payment reconciliation.
#[derive(Debug, Clone, Deserialize)]
pub struct RentmanInvoice {
    /// Internal Rentman invoice ID.
    pub id: InvoiceId,
    /// Domain-level Rentman invoice number.
    pub number: InvoiceNumber,
    /// Payment status translated from Rentman's `is_paid` field.
    #[serde(rename = "is_paid")]
    pub payment_status: InvoicePaymentStatus,
}

/// Existing Rentman payment.
#[derive(Debug, Clone, Deserialize)]
pub struct RentmanPayment {
    /// Internal Rentman payment ID.
    pub id: PaymentId,
    /// Payment timestamp.
    pub moment: DateTime<FixedOffset>,
}

/// Payload for creating or updating a Rentman payment.
#[derive(Debug, Clone, Serialize)]
pub struct RentmanPaymentPayload {
    /// Payment timestamp serialized in Rentman's RFC3339 format with milliseconds.
    #[serde(serialize_with = "serialize_moment")]
    pub moment: DateTime<FixedOffset>,
    /// Payment import source.
    pub payment_import_source: PaymentImportSource,
    /// Payment amount. `None` omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RentmanAmount>,
    /// Optional payment description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn serialize_moment<S: serde::Serializer>(
    moment: &DateTime<FixedOffset>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&moment.to_rfc3339_opts(SecondsFormat::Millis, false))
}

#[derive(Debug, Deserialize)]
struct PaymentListItem {
    id: PaymentId,
    moment: DateTime<FixedOffset>,
    invoice: String,
}

#[derive(Debug, Deserialize)]
struct RentmanListResponse<T> {
    #[serde(rename = "itemCount")]
    item_count: usize,
    limit: usize,
    data: Vec<T>,
    #[serde(default)]
    next_page_url: Option<String>,
}

impl RentmanClient {
    /// Creates a client using the default Rentman API base URL.
    pub fn new(token: RentmanApiToken) -> Self {
        Self::with_base_url(
            Url::parse("https://api.rentman.net").expect("default URL is valid"),
            token,
        )
    }

    /// Creates a client using an explicit base URL.
    ///
    /// This is mainly intended for tests, staging environments, and local
    /// proxies.
    pub fn with_base_url(mut base_url: Url, token: RentmanApiToken) -> Self {
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        Self {
            base_url,
            token,
            http: HttpClient::new(),
            retry_policy: RetryPolicy::from_env("RENTMAN", DEFAULT_RENTMAN_MIN_INTERVAL),
            limiter: Arc::new(CompositeRateLimiter::new(configured_duration_ms(
                "RENTMAN_REQUEST_MIN_INTERVAL_MS",
                DEFAULT_RENTMAN_MIN_INTERVAL,
            ))),
        }
    }

    /// Starts a request for any typed Rentman API endpoint.
    pub fn endpoint<E: endpoint::Endpoint>(&self) -> RentmanEndpointRequest<E> {
        RentmanEndpointRequest {
            client: self.clone(),
            path_params: Vec::new(),
            query_params: Vec::new(),
            body: None,
            endpoint: PhantomData,
        }
    }

    /// Loads all Rentman invoices through the documented collection endpoint.
    pub async fn all_invoices(&self) -> Result<Vec<RentmanInvoice>> {
        let mut invoices = Vec::new();
        let mut next_url = None;
        let mut offset = 0;

        loop {
            let response: RentmanListResponse<RentmanInvoice> = self
                .paginated_request(next_url.take(), "invoices", offset)
                .query(&[
                    ("fields", "id,number,is_paid".to_string()),
                    ("limit", DEFAULT_PAGE_LIMIT.to_string()),
                ])
                .send_json()
                .await
                .context("Rentman invoice list lookup failed")?;

            let should_fallback_to_offset = response.next_page_url.is_none()
                && response.item_count >= response.limit
                && response.limit > 0;
            offset += response.limit;
            next_url = parse_next_page_url(response.next_page_url.as_deref())?;
            invoices.extend(response.data);

            if next_url.is_none() && !should_fallback_to_offset {
                break;
            }
        }

        Ok(invoices)
    }

    /// Loads all Rentman payments and groups them by invoice.
    ///
    /// The payments for each invoice are sorted by ID to keep creation order
    /// stable.
    pub async fn all_payments_by_invoice(&self) -> Result<HashMap<InvoiceId, Vec<RentmanPayment>>> {
        let mut result: HashMap<InvoiceId, Vec<RentmanPayment>> = HashMap::new();
        let mut next_url = None;
        let mut offset = 0;

        loop {
            let response: RentmanListResponse<PaymentListItem> = self
                .paginated_request(next_url.take(), "payments", offset)
                .query(&[
                    ("fields", "id,moment,invoice".to_string()),
                    ("limit", DEFAULT_PAGE_LIMIT.to_string()),
                ])
                .send_json()
                .await
                .context("Rentman payment list lookup failed")?;

            let should_fallback_to_offset = response.next_page_url.is_none()
                && response.item_count >= response.limit
                && response.limit > 0;
            offset += response.limit;
            next_url = parse_next_page_url(response.next_page_url.as_deref())?;

            for item in response.data {
                let invoice_id = parse_invoice_reference(&item.invoice)?;
                result.entry(invoice_id).or_default().push(RentmanPayment {
                    id: item.id,
                    moment: item.moment,
                });
            }

            if next_url.is_none() && !should_fallback_to_offset {
                break;
            }
        }

        for payments in result.values_mut() {
            payments.sort_by_key(|payment| payment.id);
        }

        Ok(result)
    }

    /// Updates an existing Rentman payment.
    pub async fn update_payment(
        &self,
        payment_id: PaymentId,
        payload: &RentmanPaymentPayload,
    ) -> Result<()> {
        self.request(Method::PUT, &format!("payments/{payment_id}"))
            .json(payload)
            .send_unit()
            .await
            .with_context(|| format!("Rentman payment update failed for payment {payment_id}"))
    }

    /// Creates a payment on a Rentman invoice.
    pub async fn create_payment(
        &self,
        invoice_id: InvoiceId,
        payload: &RentmanPaymentPayload,
    ) -> Result<()> {
        self.request(Method::POST, &format!("invoices/{invoice_id}/payments"))
            .json(payload)
            .send_unit()
            .await
            .with_context(|| format!("Rentman payment create failed for invoice {invoice_id}"))
    }

    fn paginated_request(
        &self,
        next_url: Option<Url>,
        path: &str,
        offset: usize,
    ) -> RentmanRequestBuilder {
        match next_url {
            Some(url) => self.request_url(Method::GET, url),
            None => self
                .request(Method::GET, path)
                .query(&[("offset", offset.to_string())]),
        }
    }

    fn request(&self, method: Method, path: &str) -> RentmanRequestBuilder {
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            .expect("relative API path can be joined onto the base URL");
        self.request_url(method, url)
    }

    fn request_url(&self, method: Method, url: Url) -> RentmanRequestBuilder {
        RentmanRequestBuilder {
            client: self.clone(),
            request: self.http.request(method, url).bearer_auth(&self.token.0),
        }
    }
}

impl<E: endpoint::Endpoint> RentmanEndpointRequest<E> {
    /// Sets a raw path parameter used by the endpoint path template.
    ///
    /// Prefer typed helpers such as `id` when the endpoint exposes one.
    pub fn path_param(mut self, name: impl Into<String>, value: impl fmt::Display) -> Self {
        self.path_params.push((name.into(), value.to_string()));
        self
    }

    /// Adds query parameters to the request.
    // Coverage is disabled because this generic serde adapter is monomorphized
    // per caller; request-building behavior is covered by non-generic helpers.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Result<Self> {
        self.query_params.extend(serialize_query_params(query)?);
        Ok(self)
    }

    /// Adds one query parameter to the request.
    pub fn query_param(mut self, name: impl Into<String>, value: impl fmt::Display) -> Self {
        self.query_params.push((name.into(), value.to_string()));
        self
    }

    /// Sets the typed JSON request body.
    // Coverage is disabled because this generic serde adapter is monomorphized
    // per endpoint; concrete body handling is covered after JSON conversion.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn body(mut self, body: E::Request) -> Result<Self>
    where
        E::Request: Serialize,
    {
        self.body =
            Some(serde_json::to_value(body).context("Rentman request body is invalid JSON value")?);
        Ok(self)
    }

    /// Sends the request and returns the typed Rentman API response.
    // Coverage is disabled because this generic async glue is monomorphized per
    // endpoint; all branching is delegated to non-generic helpers below.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn send(self) -> Result<E::Response>
    where
        E: endpoint::ExecutableEndpoint,
        E::Request: Serialize,
        E::Response: DeserializeOwned,
    {
        send_typed_endpoint(
            &self.client,
            E::SPEC,
            &self.path_params,
            &self.query_params,
            self.body,
        )
        .await
    }
}

impl<E: endpoint::EndpointWithId> RentmanEndpointRequest<E> {
    /// Sets the documented integer `{id}` path parameter.
    pub fn id(self, id: model::RentmanId) -> Self {
        self.path_param("id", id)
    }
}

struct RentmanRequestBuilder {
    client: RentmanClient,
    request: reqwest::RequestBuilder,
}

impl RentmanRequestBuilder {
    fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);
        self
    }

    fn json<T: Serialize + ?Sized>(mut self, payload: &T) -> Self {
        self.request = self.request.json(payload);
        self
    }

    async fn send_json<T: for<'de> Deserialize<'de>>(self) -> Result<T> {
        let client = self.client;
        send_with_retry(client.limiter, client.retry_policy, self.request).await
    }

    async fn send_unit(self) -> Result<()> {
        let client = self.client;
        send_with_retry_unit(client.limiter, client.retry_policy, self.request).await
    }
}

fn parse_invoice_reference(reference: &str) -> Result<InvoiceId> {
    reference
        .strip_prefix("/invoices/")
        .and_then(|id| id.parse::<u64>().ok())
        .map(InvoiceId::new)
        .with_context(|| format!("Unexpected Rentman invoice reference: {reference}"))
}

fn parse_next_page_url(next_page_url: Option<&str>) -> Result<Option<Url>> {
    next_page_url
        .map(Url::parse)
        .transpose()
        .context("Rentman next_page_url is not a valid URL")
}

// Coverage is disabled because this generic async adapter is monomorphized per
// response type; request construction and JSON body selection are non-generic.
#[cfg_attr(coverage_nightly, coverage(off))]
async fn send_typed_endpoint<T: DeserializeOwned>(
    client: &RentmanClient,
    spec: endpoint::EndpointSpec,
    path_params: &[(String, String)],
    query_params: &[(String, String)],
    body: Option<Value>,
) -> Result<T> {
    build_endpoint_request(client, spec, path_params, query_params, body)?
        .send_json()
        .await
        .with_context(|| format!("Rentman endpoint {} failed", spec.operation_id))
}

fn build_endpoint_request(
    client: &RentmanClient,
    spec: endpoint::EndpointSpec,
    path_params: &[(String, String)],
    query_params: &[(String, String)],
    body: Option<Value>,
) -> Result<RentmanRequestBuilder> {
    let method =
        Method::from_bytes(spec.method.as_bytes()).expect("generated HTTP method is valid");
    let mut url = endpoint_url(spec, &client.base_url, path_params)?;
    append_query_params(&mut url, query_params);

    let mut request = client.request_url(method, url);
    match (spec.request, body) {
        (endpoint::EndpointRequestSpec::None, None) => {}
        (endpoint::EndpointRequestSpec::None, Some(_)) => {
            return Err(anyhow!(
                "Rentman endpoint {} does not accept a JSON request body",
                spec.operation_id
            ));
        }
        (endpoint::EndpointRequestSpec::Json(_), Some(body)) => {
            request = request.json(&body);
        }
        (endpoint::EndpointRequestSpec::Json(schema), None) => {
            return Err(anyhow!(
                "Rentman endpoint {} requires a {schema} JSON request body",
                spec.operation_id
            ));
        }
    }

    Ok(request)
}

// Coverage is disabled because serde's generic serializer shape is
// caller-specific; parsed query pairs are covered by the non-generic parser.
#[cfg_attr(coverage_nightly, coverage(off))]
fn serialize_query_params<T: Serialize + ?Sized>(query: &T) -> Result<Vec<(String, String)>> {
    let encoded = serde_urlencoded::to_string(query).context("Rentman query is invalid")?;
    Ok(parse_query_params(&encoded))
}

fn parse_query_params(encoded: &str) -> Vec<(String, String)> {
    form_urlencoded::parse(encoded.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

fn append_query_params(url: &mut Url, query_params: &[(String, String)]) {
    let mut pairs = url.query_pairs_mut();
    for (name, value) in query_params {
        pairs.append_pair(name, value);
    }
}

fn endpoint_url(
    spec: endpoint::EndpointSpec,
    base_url: &Url,
    path_params: &[(String, String)],
) -> Result<Url> {
    let mut path = spec.path.to_string();
    for (name, value) in path_params {
        let placeholder = format!("{{{name}}}");
        if !path.contains(&placeholder) {
            return Err(anyhow!(
                "Rentman endpoint {} has no {{{name}}} path parameter",
                spec.operation_id
            ));
        }
        let encoded = utf8_percent_encode(value, NON_ALPHANUMERIC).to_string();
        path = path.replace(&placeholder, &encoded);
    }

    if path.contains('{') || path.contains('}') {
        return Err(anyhow!(
            "Rentman endpoint {} is missing path parameters for {}",
            spec.operation_id,
            spec.path
        ));
    }

    base_url
        .join(path.trim_start_matches('/'))
        .context("generated Rentman endpoint path is valid")
}

// Coverage is disabled because this async HTTP/retry loop depends on external
// transport failures and timer scheduling; client behavior is tested at the
// request/response boundary with a local server.
#[cfg_attr(coverage_nightly, coverage(off))]
async fn send_with_retry<T: for<'de> Deserialize<'de>>(
    limiter: Arc<CompositeRateLimiter>,
    retry_policy: RetryPolicy,
    request: reqwest::RequestBuilder,
) -> Result<T> {
    let mut next_request = Some(request);
    let mut last_error = None;
    let mut backoff = retry_policy.backoff();

    for attempt in 0..retry_policy.max_attempts {
        limiter.acquire().await;

        let request = next_request
            .take()
            .ok_or_else(|| anyhow!("request could not be cloned for retry"))?;
        next_request = request.try_clone();

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) if error.is_timeout() || error.is_connect() || error.is_request() => {
                last_error = Some(anyhow!("Rentman API request failed: {error}"));
                sleep_before_retry("Rentman", retry_policy, attempt, None, &mut backoff).await;
                continue;
            }
            Err(error) => return Err(error.into()),
        };

        if !response.status().is_success() {
            let status = response.status();
            let retry_after = retry_after_delay(response.headers());
            let body = response.text().await.unwrap_or_default();
            let error = anyhow!("Rentman API returned {status}: {body}");
            if is_retryable_status(status) {
                last_error = Some(error);
                sleep_before_retry("Rentman", retry_policy, attempt, retry_after, &mut backoff)
                    .await;
                continue;
            }
            return Err(error);
        }

        let body = response.bytes().await?;
        return decode_json_response(&body);
    }

    Err(last_error.unwrap_or_else(|| anyhow!("Rentman API max retries exceeded")))
}

// Coverage is disabled because serde's generic deserializer shape is
// response-specific; empty-body normalization is covered separately.
#[cfg_attr(coverage_nightly, coverage(off))]
fn decode_json_response<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(json_decode_body(body))
        .context("Rentman API response body is invalid JSON")
}

fn json_decode_body(body: &[u8]) -> &[u8] {
    if body.is_empty() { b"null" } else { body }
}

// Coverage is disabled for the same reason as `send_with_retry`: the unit
// variant differs only in response decoding and still depends on async IO.
#[cfg_attr(coverage_nightly, coverage(off))]
async fn send_with_retry_unit(
    limiter: Arc<CompositeRateLimiter>,
    retry_policy: RetryPolicy,
    request: reqwest::RequestBuilder,
) -> Result<()> {
    let mut next_request = Some(request);
    let mut last_error = None;
    let mut backoff = retry_policy.backoff();

    for attempt in 0..retry_policy.max_attempts {
        limiter.acquire().await;

        let request = next_request
            .take()
            .ok_or_else(|| anyhow!("request could not be cloned for retry"))?;
        next_request = request.try_clone();

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) if error.is_timeout() || error.is_connect() || error.is_request() => {
                last_error = Some(anyhow!("Rentman API request failed: {error}"));
                sleep_before_retry("Rentman", retry_policy, attempt, None, &mut backoff).await;
                continue;
            }
            Err(error) => return Err(error.into()),
        };

        if !response.status().is_success() {
            let status = response.status();
            let retry_after = retry_after_delay(response.headers());
            let body = response.text().await.unwrap_or_default();
            let error = anyhow!("Rentman API returned {status}: {body}");
            if is_retryable_status(status) {
                last_error = Some(error);
                sleep_before_retry("Rentman", retry_policy, attempt, retry_after, &mut backoff)
                    .await;
                continue;
            }
            return Err(error);
        }

        return Ok(());
    }

    Err(last_error.unwrap_or_else(|| anyhow!("Rentman API max retries exceeded")))
}

struct CompositeRateLimiter {
    per_request: DirectRateLimiter,
    per_day: DirectRateLimiter,
}

impl CompositeRateLimiter {
    // Coverage is disabled because governor's internal clock and token bucket
    // state are integration behavior, not deterministic client logic.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn new(min_interval: Duration) -> Self {
        Self {
            per_request: RateLimiter::direct(quota_with_burst(min_interval, 1)),
            per_day: RateLimiter::direct(quota_with_burst(
                Duration::from_secs(24 * 60 * 60) / 50_000,
                50_000,
            )),
        }
    }

    // Coverage is disabled because this waits on governor's async clock; rate
    // limit configuration is covered through construction and request tests.
    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn acquire(&self) {
        self.per_request.until_ready().await;
        self.per_day.until_ready().await;
    }
}

#[derive(Debug, Clone, Copy)]
struct RetryPolicy {
    max_attempts: usize,
    base_delay: Duration,
}

impl RetryPolicy {
    fn from_env(prefix: &str, default_base_delay: Duration) -> Self {
        let max_attempts = configured_usize(&format!("{prefix}_API_MAX_ATTEMPTS"), 4).max(1);
        let base_delay =
            configured_duration_ms(&format!("{prefix}_RETRY_BASE_DELAY_MS"), default_base_delay);
        Self {
            max_attempts,
            base_delay,
        }
    }

    fn backoff(self) -> backon::ExponentialBackoff {
        ExponentialBuilder::default()
            .with_min_delay(self.base_delay)
            .with_max_delay(self.base_delay.saturating_mul(16))
            .with_max_times(self.max_attempts.saturating_sub(1))
            .build()
    }
}

fn quota_with_burst(replenish_1_per: Duration, burst: u32) -> Quota {
    Quota::with_period(replenish_1_per)
        .expect("API minimum interval must be greater than zero")
        .allow_burst(NonZeroU32::new(burst).expect("burst must be greater than zero"))
}

fn configured_usize(name: &str, default: usize) -> usize {
    parse_usize(std::env::var(name).ok(), default)
}

fn parse_usize(value: Option<String>, default: usize) -> usize {
    value
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn configured_duration_ms(name: &str, default: Duration) -> Duration {
    parse_duration_ms(std::env::var(name).ok(), default)
}

fn parse_duration_ms(value: Option<String>, default: Duration) -> Duration {
    value
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(default)
}

fn is_retryable_status(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::CONFLICT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status == StatusCode::TOO_EARLY
        || status.is_server_error()
}

fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?.to_str().ok()?.trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let retry_at = DateTime::parse_from_rfc2822(value).ok()?;
    let now = Utc::now();
    (retry_at.with_timezone(&Utc) - now).to_std().ok()
}

// Coverage is disabled because this function intentionally waits on async
// timers; retry classification and Retry-After parsing are tested separately.
#[cfg_attr(coverage_nightly, coverage(off))]
async fn sleep_before_retry(
    api: &'static str,
    retry_policy: RetryPolicy,
    attempt: usize,
    retry_after: Option<Duration>,
    backoff: &mut backon::ExponentialBackoff,
) {
    if attempt + 1 >= retry_policy.max_attempts {
        return;
    }

    let delay = retry_after
        .or_else(|| backoff.next())
        .unwrap_or(retry_policy.base_delay);
    tracing::warn!(
        api,
        attempt = attempt + 1,
        max_attempts = retry_policy.max_attempts,
        delay_ms = delay.as_millis(),
        "API request will be retried"
    );
    tokio::time::sleep(delay).await;
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::{
        InvoiceId, InvoiceNumber, InvoicePaymentStatus, PaymentId, PaymentImportSource,
        RentmanAmount, RentmanApiToken, RentmanClient, RentmanPaymentPayload, Url,
        endpoint::{
            CreatePaymentEndpoint, DeleteAccessoryEndpoint, GetAccessoryCollectionEndpoint,
            GetAccessoryItemEndpoint,
        },
        is_retryable_status,
        model::RentmanId,
        model::{
            EmailAddress, ItemResponse, NoContent, NoRequest, PaymentRequest,
            PaymentRequestPaymentImportSource, ResourceReference,
        },
        parse_duration_ms, parse_usize, retry_after_delay,
    };
    use chrono::{DateTime, Utc};
    use reqwest::{
        StatusCode,
        header::{HeaderMap, HeaderValue, RETRY_AFTER},
    };
    use rust_decimal::Decimal;
    use serde::Serialize;
    use serde_json::Value;
    use std::{
        collections::BTreeMap,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        sync::mpsc::{self, Receiver},
        thread,
        time::Duration,
    };

    #[test]
    fn parses_configured_values_with_fallback() {
        assert_eq!(parse_usize(Some("7".into()), 4), 7);
        assert_eq!(parse_usize(Some("not-a-number".into()), 4), 4);
        assert_eq!(parse_usize(None, 4), 4);

        let default = Duration::from_millis(50);
        assert_eq!(
            parse_duration_ms(Some("250".into()), default),
            Duration::from_millis(250)
        );
        assert_eq!(parse_duration_ms(Some("nope".into()), default), default);
        assert_eq!(parse_duration_ms(None, default), default);
    }

    #[test]
    fn configures_default_and_custom_base_urls() {
        let default_client = RentmanClient::new(RentmanApiToken::new("token"));
        assert_eq!(default_client.base_url.as_str(), "https://api.rentman.net/");

        let custom_client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );
        assert_eq!(custom_client.base_url.as_str(), "http://example.test/");
    }

    #[test]
    fn normalizes_base_urls_without_trailing_slash() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/api").unwrap(),
            RentmanApiToken::new("token"),
        );
        assert_eq!(client.base_url.as_str(), "http://example.test/api/");
    }

    #[test]
    fn exposes_invoice_number_as_str_and_display() {
        let number = InvoiceNumber::new("1312");
        assert_eq!(number.as_str(), "1312");
        assert_eq!(number.to_string(), "1312");
    }

    #[test]
    fn redacts_token_in_debug_output() {
        assert_eq!(
            format!("{:?}", RentmanApiToken::new("secret")),
            "RentmanApiToken(***)"
        );
    }

    #[test]
    fn exposes_amount_decimal_without_float_rounding() {
        let decimal = Decimal::new(425, 1);
        let amount = RentmanAmount::from(decimal);

        assert_eq!(amount.as_decimal(), decimal);
    }

    #[test]
    fn classifies_transient_http_statuses_for_retry() {
        assert!(is_retryable_status(StatusCode::REQUEST_TIMEOUT));
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::BAD_GATEWAY));
        assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE));
        assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
        assert!(!is_retryable_status(StatusCode::UNAUTHORIZED));
        assert!(!is_retryable_status(StatusCode::FORBIDDEN));
    }

    #[test]
    fn parses_retry_after_seconds() {
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, HeaderValue::from_static("2"));

        assert_eq!(retry_after_delay(&headers).unwrap().as_secs(), 2);
    }

    #[test]
    fn parses_retry_after_http_date() {
        let future = (Utc::now() + chrono::Duration::seconds(90)).to_rfc2822();
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, HeaderValue::from_str(&future).unwrap());
        let delay = retry_after_delay(&headers).unwrap();
        assert!(delay.as_secs() > 60 && delay.as_secs() <= 90);

        let past = (Utc::now() - chrono::Duration::seconds(90)).to_rfc2822();
        headers.insert(RETRY_AFTER, HeaderValue::from_str(&past).unwrap());
        assert!(retry_after_delay(&headers).is_none());

        headers.insert(RETRY_AFTER, HeaderValue::from_static("not-a-date"));
        assert!(retry_after_delay(&headers).is_none());
        assert!(retry_after_delay(&HeaderMap::new()).is_none());

        headers.insert(RETRY_AFTER, HeaderValue::from_bytes(&[0xFF]).unwrap());
        assert!(retry_after_delay(&headers).is_none());
    }

    #[tokio::test]
    async fn reads_all_invoices_from_next_page_url_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1,"limit":1500,"next_page_url":"{base}/page-2","data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"next_page_url":null,"data":[{"id":2,"number":"RE-2","is_paid":true}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let invoices = client.all_invoices().await.unwrap();

        assert_eq!(invoices.len(), 2);
        assert_eq!(invoices[0].id, InvoiceId::new(1));
        assert_eq!(invoices[0].number, InvoiceNumber::new("RE-1"));
        assert_eq!(invoices[0].payment_status, InvoicePaymentStatus::Open);
        assert_eq!(invoices[1].id, InvoiceId::new(2));
        assert_eq!(invoices[1].number, InvoiceNumber::new("RE-2"));
        assert_eq!(invoices[1].payment_status, InvoicePaymentStatus::Paid);

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/invoices");
        assert_bearer_token(&first_request);
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));
        assert_eq!(
            first_request.query_value("fields").as_deref(),
            Some("id,number,is_paid")
        );
        assert_eq!(first_request.query_value("limit").as_deref(), Some("1500"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/page-2");
        assert_bearer_token(&second_request);
    }

    #[tokio::test]
    async fn reads_all_invoices_from_offset_fallback_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1500,"limit":1500,"data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"data":[{"id":2,"number":"RE-2","is_paid":true}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let invoices = client.all_invoices().await.unwrap();

        assert_eq!(invoices.len(), 2);
        assert_eq!(invoices[0].id, InvoiceId::new(1));
        assert_eq!(invoices[1].id, InvoiceId::new(2));

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/invoices");
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/invoices");
        assert_eq!(
            second_request.query_value("offset").as_deref(),
            Some("1500")
        );
    }

    #[tokio::test]
    async fn adds_context_to_invoice_list_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client.all_invoices().await.unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman invoice list lookup failed")
        );
        assert_request(&server.next_request(), "GET", "/invoices");
    }

    #[tokio::test]
    async fn rejects_invalid_invoice_next_page_urls() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":"not-a-url","data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client.all_invoices().await.unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman next_page_url is not a valid URL")
        );
        assert_request(&server.next_request(), "GET", "/invoices");
    }

    #[tokio::test]
    async fn groups_all_payments_by_invoice_sorted_by_id() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":3,"limit":1500,"next_page_url":null,"data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1"},{"id":10,"moment":"2026-07-04T00:00:00.000+02:00","invoice":"/invoices/1"},{"id":12,"moment":"2026-07-06T00:00:00.000+02:00","invoice":"/invoices/2"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let payments = client.all_payments_by_invoice().await.unwrap();

        assert_eq!(payments.len(), 2);
        let first_invoice = &payments[&InvoiceId::new(1)];
        assert_eq!(first_invoice.len(), 2);
        assert_eq!(first_invoice[0].id, PaymentId::new(10));
        assert_eq!(first_invoice[1].id, PaymentId::new(11));
        assert_eq!(
            first_invoice[0].moment,
            DateTime::parse_from_rfc3339("2026-07-04T00:00:00.000+02:00").unwrap()
        );
        assert_eq!(payments[&InvoiceId::new(2)][0].id, PaymentId::new(12));

        let request = server.next_request();
        assert_request(&request, "GET", "/payments");
        assert_bearer_token(&request);
        assert_eq!(request.query_value("offset").as_deref(), Some("0"));
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,moment,invoice")
        );
        assert_eq!(request.query_value("limit").as_deref(), Some("1500"));
    }

    #[tokio::test]
    async fn reads_all_payments_from_offset_fallback_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1500,"limit":1500,"data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1"}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"data":[{"id":12,"moment":"2026-07-06T00:00:00.000+02:00","invoice":"/invoices/2"}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let payments = client.all_payments_by_invoice().await.unwrap();

        assert_eq!(payments.len(), 2);
        assert_eq!(payments[&InvoiceId::new(1)][0].id, PaymentId::new(11));
        assert_eq!(payments[&InvoiceId::new(2)][0].id, PaymentId::new(12));

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/payments");
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/payments");
        assert_eq!(
            second_request.query_value("offset").as_deref(),
            Some("1500")
        );
    }

    #[tokio::test]
    async fn adds_context_to_payment_list_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client.all_payments_by_invoice().await.unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman payment list lookup failed")
        );
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn rejects_invalid_payment_next_page_urls() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":"not-a-url","data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client.all_payments_by_invoice().await.unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman next_page_url is not a valid URL")
        );
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn rejects_unexpected_invoice_references() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":null,"data":[{"id":10,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/broken/1"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client.all_payments_by_invoice().await.unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Unexpected Rentman invoice reference: /broken/1")
        );
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn updates_existing_payment() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 200 OK")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        client
            .update_payment(PaymentId::new(10), &payment_payload())
            .await
            .unwrap();

        let request = server.next_request();
        assert_request(&request, "PUT", "/payments/10");
        assert_bearer_token(&request);
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(body["payment_import_source"], "publicapi");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
    }

    #[tokio::test]
    async fn creates_missing_payment() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 200 OK")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap();

        let request = server.next_request();
        assert_request(&request, "POST", "/invoices/1/payments");
        assert_bearer_token(&request);
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(body["payment_import_source"], "publicapi");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
    }

    #[tokio::test]
    async fn executes_typed_endpoint_with_path_query_body_and_response() {
        let server = spawn_server(vec![json_response(
            r#"{"data":{"id":99,"created":null,"modified":null,"creator":null,"displayname":"Payment 99","invoice":"/invoices/7","moment":"2026-07-05T00:00:00.000+02:00","amount":42.5,"description":"Manual booking","payment_import_source":"publicapi"},"itemCount":1,"limit":1}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );
        let payload = PaymentRequest {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            amount: Some(Decimal::new(425, 1)),
            description: Some("Manual booking".to_string()),
            payment_import_source: Some(PaymentRequestPaymentImportSource::Publicapi),
        };

        let response: ItemResponse<_> = client
            .endpoint::<CreatePaymentEndpoint>()
            .id(RentmanId(7))
            .query(&[("limit", "1")])
            .unwrap()
            .query_param("fields", "id,moment,invoice,amount,description")
            .body(payload)
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(response.data.id.to_string(), "99");
        assert_eq!(response.data.invoice.as_str(), "/invoices/7");
        assert_eq!(response.data.description, "Manual booking");

        let request = server.next_request();
        assert_request(&request, "POST", "/invoices/7/payments");
        assert_bearer_token(&request);
        assert_eq!(request.query_value("limit").as_deref(), Some("1"));
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,moment,invoice,amount,description")
        );
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00+02:00");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
        assert_eq!(body["payment_import_source"], "publicapi");
    }

    #[tokio::test]
    async fn executes_typed_endpoint_without_response_body() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 204 No Content")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let response = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .send()
            .await
            .unwrap();

        assert_eq!(response, NoContent);
        let request = server.next_request();
        assert_request(&request, "DELETE", "/accessories/10");
        assert_bearer_token(&request);
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_without_required_body() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<CreatePaymentEndpoint>()
            .id(RentmanId(7))
            .send()
            .await
            .unwrap_err();

        assert!(error.to_string().contains(
            "Rentman endpoint createPayment requires a PaymentRequest JSON request body"
        ));
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_body_when_endpoint_has_no_body() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .body(NoRequest)
            .unwrap()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint deleteAccessory does not accept a JSON request body")
        );
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_missing_path_parameter() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<GetAccessoryItemEndpoint>()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint getAccessoryItem is missing path parameters")
        );
    }

    #[tokio::test]
    async fn rejects_unknown_typed_endpoint_path_parameter() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .path_param("unknown", 10)
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint deleteAccessory has no {unknown} path parameter")
        );
    }

    #[tokio::test]
    async fn adds_context_to_payment_update_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .update_payment(PaymentId::new(10), &payment_payload())
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman payment update failed for payment 10")
        );
        assert_request(&server.next_request(), "PUT", "/payments/10");
    }

    #[tokio::test]
    async fn adds_context_to_payment_create_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman payment create failed for invoice 1")
        );
        assert_request(&server.next_request(), "POST", "/invoices/1/payments");
    }

    #[test]
    fn serializes_payload_in_rentman_wire_format() {
        let json = serde_json::to_value(payment_payload()).unwrap();

        assert_eq!(json["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(json["payment_import_source"], "publicapi");
        assert_eq!(json["amount"], serde_json::json!(42.5));
        assert_eq!(json["description"], "Manual booking");

        let disabled = RentmanPaymentPayload {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            payment_import_source: PaymentImportSource::None,
            amount: None,
            description: None,
        };
        let json = serde_json::to_value(&disabled).unwrap();

        assert_eq!(json["payment_import_source"], "none");
        assert!(json.get("amount").is_none());
        assert!(json.get("description").is_none());
    }

    #[test]
    fn typed_query_reports_serialization_errors() {
        struct BrokenQuery;

        impl Serialize for BrokenQuery {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("broken query"))
            }
        }

        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = match client
            .endpoint::<GetAccessoryCollectionEndpoint>()
            .query(&BrokenQuery)
        {
            Ok(_) => panic!("broken query should fail"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("Rentman query is invalid"));
    }

    #[test]
    fn typed_models_reject_invalid_json_types() {
        let error = serde_json::from_str::<ResourceReference>("42").unwrap_err();
        assert!(error.to_string().contains("invalid type"));

        let error = serde_json::from_str::<EmailAddress>("42").unwrap_err();
        assert!(error.to_string().contains("invalid type"));
    }

    #[tokio::test]
    async fn typed_endpoint_errors_include_operation_context() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<GetAccessoryCollectionEndpoint>()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint getAccessoryCollection failed")
        );
        assert_request(&server.next_request(), "GET", "/accessories");
    }

    fn payment_payload() -> RentmanPaymentPayload {
        RentmanPaymentPayload {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            payment_import_source: PaymentImportSource::PublicApi,
            amount: Some(RentmanAmount::new(Decimal::new(425, 1))),
            description: Some("Manual booking".to_string()),
        }
    }

    struct TestServer {
        base_url: String,
        requests: Receiver<RecordedRequest>,
    }

    impl TestServer {
        fn base_url(&self) -> &str {
            &self.base_url
        }

        fn next_request(&self) -> RecordedRequest {
            self.requests
                .recv_timeout(Duration::from_secs(2))
                .expect("test server recorded request")
        }
    }

    #[derive(Debug)]
    struct RecordedRequest {
        method: String,
        target: String,
        path: String,
        headers: BTreeMap<String, String>,
        body: Vec<u8>,
    }

    impl RecordedRequest {
        fn header(&self, name: &str) -> Option<&str> {
            self.headers
                .get(&name.to_ascii_lowercase())
                .map(String::as_str)
        }

        fn query_value(&self, name: &str) -> Option<String> {
            let url = Url::parse(&format!("http://example.test{}", self.target)).unwrap();
            url.query_pairs()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.into_owned())
        }

        fn json_body(&self) -> Value {
            serde_json::from_slice(&self.body).expect("request body is valid JSON")
        }
    }

    fn spawn_server(responses: Vec<String>) -> TestServer {
        let listener = TcpListener::bind("127.0.0.1:0").expect("test server binds");
        let address = listener.local_addr().expect("test server has address");
        let base_url = format!("http://{address}");
        let (request_tx, request_rx) = mpsc::channel();
        let responses = responses
            .into_iter()
            .map(|response| response.replace("{base}", &base_url))
            .map(refresh_content_length)
            .collect::<Vec<_>>();

        thread::spawn(move || {
            for response in responses {
                let (mut stream, _) = listener.accept().expect("test server accepts request");
                let request = read_request(&mut stream);
                request_tx
                    .send(request)
                    .expect("test server sends recorded request");
                stream
                    .write_all(response.as_bytes())
                    .expect("test server writes response");
            }
        });

        TestServer {
            base_url,
            requests: request_rx,
        }
    }

    fn read_request(stream: &mut TcpStream) -> RecordedRequest {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];

        loop {
            let bytes = stream.read(&mut buffer).expect("test server reads request");
            if bytes == 0 {
                break;
            }

            request.extend_from_slice(&buffer[..bytes]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }

        let header_end = request
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("request contains headers");
        let mut body = request[(header_end + 4)..].to_vec();
        let headers = String::from_utf8_lossy(&request[..header_end]);
        let mut lines = headers.lines();
        let start_line = lines.next().expect("request has start line");
        let mut start_line_parts = start_line.split_whitespace();
        let method = start_line_parts.next().expect("request has method");
        let target = start_line_parts.next().expect("request has target");
        let path = target.split('?').next().expect("request has path");

        let headers = lines
            .filter_map(|line| {
                let (name, value) = line.split_once(':')?;
                Some((name.trim().to_ascii_lowercase(), value.trim().to_string()))
            })
            .collect::<BTreeMap<_, _>>();
        let content_length = headers
            .get("content-length")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or_default();

        while body.len() < content_length {
            let bytes = stream.read(&mut buffer).expect("test server reads body");
            if bytes == 0 {
                break;
            }
            body.extend_from_slice(&buffer[..bytes]);
        }
        body.truncate(content_length);

        RecordedRequest {
            method: method.to_string(),
            target: target.to_string(),
            path: path.to_string(),
            headers,
            body,
        }
    }

    fn assert_request(request: &RecordedRequest, method: &str, path: &str) {
        assert_eq!(request.method, method);
        assert_eq!(request.path, path);
    }

    fn assert_bearer_token(request: &RecordedRequest) {
        assert_eq!(request.header("authorization"), Some("Bearer token"));
    }

    fn json_response(body: &str) -> String {
        text_response("HTTP/1.1 200 OK", body)
    }

    fn empty_response(status_line: &str) -> String {
        text_response(status_line, "")
    }

    fn text_response(status_line: &str, body: &str) -> String {
        format!(
            "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn refresh_content_length(response: String) -> String {
        let Some((headers, body)) = response.split_once("\r\n\r\n") else {
            return response;
        };
        let headers = headers
            .lines()
            .map(|line| {
                if line.to_ascii_lowercase().starts_with("content-length:") {
                    format!("Content-Length: {}", body.len())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\r\n");

        format!("{headers}\r\n\r\n{body}")
    }
}
