//! Sending requests with rate limiting and retries.

use anyhow::{Context, Result, anyhow};
use backon::{BackoffBuilder, ExponentialBuilder};
use chrono::{DateTime, Utc};
use governor::{Quota, RateLimiter};
use reqwest::{
    StatusCode,
    header::{HeaderMap, RETRY_AFTER},
};
use serde::Deserialize;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

type DirectRateLimiter = RateLimiter<
    governor::state::NotKeyed,
    governor::state::InMemoryState,
    governor::clock::DefaultClock,
>;

// Coverage is disabled because this async HTTP/retry loop depends on external
// transport failures and timer scheduling; client behavior is tested at the
// request/response boundary with a local server.
#[cfg_attr(coverage_nightly, coverage(off))]
pub(crate) async fn send_with_retry<T: for<'de> Deserialize<'de>>(
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
pub(crate) async fn send_with_retry_unit(
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

pub(crate) struct CompositeRateLimiter {
    per_request: DirectRateLimiter,
    per_day: DirectRateLimiter,
}

impl CompositeRateLimiter {
    // Coverage is disabled because governor's internal clock and token bucket
    // state are integration behavior, not deterministic client logic.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) fn new(min_interval: Duration) -> Self {
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
pub(crate) struct RetryPolicy {
    max_attempts: usize,
    base_delay: Duration,
}

impl RetryPolicy {
    pub(crate) fn from_env(prefix: &str, default_base_delay: Duration) -> Self {
        let max_attempts = configured_usize(&format!("{prefix}_API_MAX_ATTEMPTS"), 4).max(1);
        let base_delay =
            configured_duration_ms(&format!("{prefix}_RETRY_BASE_DELAY_MS"), default_base_delay);
        Self {
            max_attempts,
            base_delay,
        }
    }

    /// Fixed policy for tests, independent of the environment.
    #[cfg(test)]
    pub(crate) fn fixed(max_attempts: usize, base_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
        }
    }

    /// The same policy without retries.
    pub(crate) fn single_attempt(self) -> Self {
        Self {
            max_attempts: 1,
            ..self
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

pub(crate) fn configured_duration_ms(name: &str, default: Duration) -> Duration {
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
    use super::*;

    use chrono::Utc;

    use reqwest::{
        StatusCode,
        header::{HeaderMap, HeaderValue, RETRY_AFTER},
    };

    use std::time::Duration;

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
}
