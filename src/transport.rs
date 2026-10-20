//! Sending requests with rate limiting and retries.

use crate::{
    RentmanClientConfig,
    error::{Error, ErrorKind, InvalidRequest, RequestOutcome, Result},
};
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

/// Sends a request and decodes the JSON response, retrying transient failures.
pub(crate) async fn send_with_retry<T: for<'de> Deserialize<'de>>(
    limiter: Arc<CompositeRateLimiter>,
    retry_policy: RetryPolicy,
    request: reqwest::RequestBuilder,
) -> Result<T> {
    let response = send_with_retries(&limiter, retry_policy, request).await?;
    // Rentman already answered with a success status: a broken body now means
    // the request was applied, not that its outcome is unknown.
    let body = response
        .bytes()
        .await
        .map_err(|error| Error::new(ErrorKind::Transport(error)).after_success())?;
    decode_json_response(&body)
}

/// Sends a request without a response body, retrying transient failures.
pub(crate) async fn send_with_retry_unit(
    limiter: Arc<CompositeRateLimiter>,
    retry_policy: RetryPolicy,
    request: reqwest::RequestBuilder,
) -> Result<()> {
    send_with_retries(&limiter, retry_policy, request)
        .await
        .map(drop)
}

/// Sends a request until it succeeds, fails permanently, or the retry policy
/// is exhausted, and returns the successful response.
async fn send_with_retries(
    limiter: &CompositeRateLimiter,
    retry_policy: RetryPolicy,
    request: reqwest::RequestBuilder,
) -> Result<reqwest::Response> {
    // Invalid requests (e.g. a malformed header) fail here, before any attempt.
    let (http, request) = request.build_split();
    let request = request.map_err(|error| Error::invalid_request(InvalidRequest::Http(error)))?;
    let mut backoff = retry_policy.backoff();
    let mut attempts = 0;
    let mut uncertain = false;

    loop {
        limiter.acquire().await;
        attempts += 1;

        let attempt = request
            .try_clone()
            .expect("the client never builds streaming request bodies, so requests can be cloned");
        let (error, retry_after) = match http.execute(attempt).await {
            Ok(response) if response.status().is_success() => return Ok(response),
            Ok(response) => {
                let status = response.status();
                let retry_after = retry_after_delay(response.headers());
                let body = response.text().await.unwrap_or_default();
                let error = Error::new(ErrorKind::Status { status, body });
                if !is_retryable_status(status) {
                    return Err(error.after_uncertain_attempt(uncertain));
                }
                (error, retry_after)
            }
            Err(error) if is_retryable_transport_error(&error) => {
                (Error::new(ErrorKind::Transport(error)), None)
            }
            Err(error) => {
                return Err(
                    Error::new(ErrorKind::Transport(error)).after_uncertain_attempt(uncertain)
                );
            }
        };

        let Some(delay) = retry_delay(retry_policy, attempts, retry_after, &mut backoff) else {
            return Err(error.after_uncertain_attempt(uncertain));
        };
        uncertain |= error.outcome() != RequestOutcome::NotApplied;
        tracing::warn!(
            attempt = attempts,
            max_attempts = retry_policy.max_attempts,
            delay_ms = delay.as_millis(),
            "Rentman API request will be retried: {error:#}"
        );
        tokio::time::sleep(delay).await;
    }
}

/// Transport failures where the request may not have reached Rentman or the
/// response got lost, so repeating an idempotent request is safe.
fn is_retryable_transport_error(error: &reqwest::Error) -> bool {
    error.is_timeout() || error.is_connect() || error.is_request()
}

/// Returns how long to wait before the next attempt, or `None` when the policy
/// allows no further attempt. A `Retry-After` from Rentman wins over backoff.
fn retry_delay(
    retry_policy: RetryPolicy,
    attempts: usize,
    retry_after: Option<Duration>,
    backoff: &mut backon::ExponentialBackoff,
) -> Option<Duration> {
    if attempts >= retry_policy.max_attempts {
        return None;
    }
    Some(
        retry_after
            .or_else(|| backoff.next())
            .unwrap_or(retry_policy.base_delay),
    )
}

fn decode_json_response<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(json_decode_body(body))
        .map_err(|error| Error::new(ErrorKind::Decode(error)))
}

fn json_decode_body(body: &[u8]) -> &[u8] {
    if body.is_empty() { b"null" } else { body }
}

pub(crate) struct CompositeRateLimiter {
    per_request: DirectRateLimiter,
    per_day: DirectRateLimiter,
}

impl CompositeRateLimiter {
    pub(crate) fn new(min_interval: Duration) -> Self {
        Self {
            per_request: RateLimiter::direct(quota_with_burst(min_interval, 1)),
            per_day: RateLimiter::direct(quota_with_burst(
                Duration::from_secs(24 * 60 * 60) / 50_000,
                50_000,
            )),
        }
    }

    async fn acquire(&self) {
        self.per_request.until_ready().await;
        self.per_day.until_ready().await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RetryPolicy {
    max_attempts: usize,
    base_delay: Duration,
}

impl RetryPolicy {
    pub(crate) fn from_config(config: &RentmanClientConfig) -> Self {
        Self {
            max_attempts: config.max_attempts().get(),
            base_delay: config.retry_base_delay(),
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
        .expect("the configuration rejects a zero request interval")
        .allow_burst(NonZeroU32::new(burst).expect("burst must be greater than zero"))
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

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::*;

    use chrono::Utc;

    use reqwest::{
        StatusCode,
        header::{HeaderMap, HeaderValue, RETRY_AFTER},
    };

    use std::time::Duration;

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

    fn fast_policy(max_attempts: usize) -> RetryPolicy {
        RetryPolicy {
            max_attempts,
            base_delay: Duration::from_millis(1),
        }
    }

    fn fast_limiter() -> CompositeRateLimiter {
        CompositeRateLimiter::new(Duration::from_millis(1))
    }

    fn get(server: &TestServer) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(format!("{}/items", server.base_url()))
    }

    #[tokio::test]
    async fn retries_transient_status_until_success() {
        // The first answer asks for a 1 s pause, far above the 1 ms backoff.
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "later").replacen(
                "\r\n",
                "\r\nRetry-After: 1\r\n",
                1,
            ),
            text_response("HTTP/1.1 502 Bad Gateway", "later"),
            json_response(r#"{"ok":true}"#),
        ]);

        let warnings = capture_warnings();

        let started = std::time::Instant::now();
        let response = send_with_retries(&fast_limiter(), fast_policy(3), get(&server))
            .await
            .unwrap();
        let elapsed = started.elapsed();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            elapsed >= Duration::from_secs(1),
            "the loop must wait for Retry-After, took {elapsed:?}"
        );
        let events = warnings.events();
        assert_eq!(events.len(), 2, "{events:?}");
        assert!(events[0].contains("attempt=1"), "{events:?}");
        assert!(events[0].contains("max_attempts=3"), "{events:?}");
        assert!(events[0].contains("delay_ms=1000"), "{events:?}");
        assert!(
            events[0].contains("will be retried: Rentman API returned 503"),
            "{events:?}"
        );
        assert!(events[1].contains("attempt=2"), "{events:?}");
        assert_eq!(response.text().await.unwrap(), r#"{"ok":true}"#);
        for _ in 0..3 {
            assert_request(&server.next_request(), "GET", "/items");
        }
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn returns_last_error_when_attempts_are_exhausted() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "first"),
            text_response("HTTP/1.1 429 Too Many Requests", "second"),
            json_response(r#"{"unused":true}"#),
        ]);

        let error = send_with_retries(&fast_limiter(), fast_policy(2), get(&server))
            .await
            .unwrap_err();

        assert_eq!(error.status(), Some(StatusCode::TOO_MANY_REQUESTS));
        // A 429 alone means "not applied", but the earlier 503 attempt may
        // have been applied.
        assert_eq!(error.outcome(), RequestOutcome::Unknown);
        assert_eq!(
            error.to_string(),
            "Rentman API returned 429 Too Many Requests: second"
        );
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn does_not_retry_permanent_status() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "broken"),
            json_response(r#"{"unused":true}"#),
        ]);

        let error = send_with_retries(&fast_limiter(), fast_policy(3), get(&server))
            .await
            .unwrap_err();

        assert!(matches!(
            error.kind(),
            ErrorKind::Status { status: StatusCode::BAD_REQUEST, body } if body == "broken"
        ));
        assert_eq!(error.outcome(), RequestOutcome::NotApplied);
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn retries_connections_closed_without_response() {
        // An empty response closes the connection before any status line.
        let server = spawn_server(vec![
            String::new(),
            empty_response("HTTP/1.1 204 No Content"),
        ]);

        send_with_retry_unit(Arc::new(fast_limiter()), fast_policy(2), get(&server))
            .await
            .unwrap();

        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn retries_requests_that_time_out() {
        // The first answer comes long after the client's 200 ms timeout.
        let server = spawn_delayed_server(vec![
            (Duration::from_secs(1), json_response(r#"{"late":true}"#)),
            (Duration::ZERO, json_response(r#"{"ok":true}"#)),
        ]);
        let request = reqwest::Client::builder()
            .timeout(Duration::from_millis(200))
            .build()
            .unwrap()
            .get(format!("{}/items", server.base_url()));

        let response = send_with_retries(&fast_limiter(), fast_policy(2), request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await.unwrap(), r#"{"ok":true}"#);
        assert_request(&server.next_request(), "GET", "/items");
        assert_request(&server.next_request(), "GET", "/items");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn reports_transport_failure_after_last_attempt() {
        // Bind and drop a listener to get a port nothing listens on.
        let address = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap();
        let request = reqwest::Client::new().get(format!("http://{address}/items"));

        let error = send_with_retries(&fast_limiter(), fast_policy(2), request)
            .await
            .unwrap_err();

        let ErrorKind::Transport(transport) = error.kind() else {
            panic!("expected a transport error, got {error:?}");
        };
        assert!(transport.is_connect(), "{transport:#}");
        assert_eq!(error.outcome(), RequestOutcome::NotApplied);
    }

    #[tokio::test]
    async fn does_not_retry_requests_that_cannot_be_built() {
        let request = reqwest::Client::new()
            .get("http://127.0.0.1:9/items")
            .header("x-invalid", "line\nbreak");

        let error = send_with_retries(&fast_limiter(), fast_policy(3), request)
            .await
            .unwrap_err();

        assert_eq!(error.outcome(), RequestOutcome::NotApplied);
        let ErrorKind::InvalidRequest(InvalidRequest::Http(error)) = error.kind() else {
            panic!("expected an invalid HTTP request, got {error:?}");
        };
        assert!(error.is_builder(), "{error:#}");
    }

    #[test]
    fn retry_delay_prefers_retry_after_and_stops_at_max_attempts() {
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
        };
        let mut backoff = policy.backoff();

        assert_eq!(
            retry_delay(policy, 1, Some(Duration::from_secs(7)), &mut backoff),
            Some(Duration::from_secs(7))
        );
        assert_eq!(
            retry_delay(policy, 2, None, &mut backoff),
            Some(Duration::from_millis(100)),
            "first backoff step equals the base delay"
        );
        assert_eq!(
            retry_delay(policy, 3, Some(Duration::from_secs(7)), &mut backoff),
            None
        );
    }

    #[test]
    fn retry_delay_falls_back_to_base_delay_when_backoff_is_exhausted() {
        let policy = RetryPolicy {
            max_attempts: 5,
            base_delay: Duration::from_millis(30),
        };
        let mut backoff = policy.single_attempt().backoff();

        assert_eq!(
            retry_delay(policy, 1, None, &mut backoff),
            Some(Duration::from_millis(30))
        );
    }

    #[test]
    fn single_attempt_policy_never_waits_for_a_retry() {
        let policy = fast_policy(4).single_attempt();
        let mut backoff = policy.backoff();

        assert_eq!(retry_delay(policy, 1, None, &mut backoff), None);
    }

    #[tokio::test]
    async fn rate_limiter_spaces_requests_by_the_minimum_interval() {
        let limiter = CompositeRateLimiter::new(Duration::from_millis(50));

        let started = std::time::Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        limiter.acquire().await;

        assert!(
            started.elapsed() >= Duration::from_millis(100),
            "three requests need at least two intervals, took {:?}",
            started.elapsed()
        );
    }

    #[tokio::test]
    async fn does_not_retry_permanent_transport_errors() {
        let server = spawn_server(vec![
            "HTTP/1.1 302 Found\r\nLocation: /elsewhere\r\nContent-Length: 0\r\n\r\n".to_string(),
            json_response(r#"{"unused":true}"#),
        ]);
        // A redirect beyond the client's limit is a permanent client-side error.
        let request = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(0))
            .build()
            .unwrap()
            .get(format!("{}/items", server.base_url()));

        let error = send_with_retries(&fast_limiter(), fast_policy(3), request)
            .await
            .unwrap_err();

        let ErrorKind::Transport(error) = error.kind() else {
            panic!("expected a transport error, got {error:?}");
        };
        assert!(error.is_redirect(), "{error:#}");
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn decodes_json_bodies_and_reports_every_failure() {
        let server = spawn_server(vec![
            json_response(r#"{"id":7}"#),
            empty_response("HTTP/1.1 200 OK"),
            text_response("HTTP/1.1 200 OK", "not json"),
            // The connection closes in the middle of a chunked body.
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n9\r\n{\"id\"".to_string(),
            text_response("HTTP/1.1 404 Not Found", "missing"),
        ]);
        let limiter = Arc::new(fast_limiter());

        let value: serde_json::Value =
            send_with_retry(limiter.clone(), fast_policy(1), get(&server))
                .await
                .unwrap();
        assert_eq!(value, serde_json::json!({"id": 7}));

        let empty: serde_json::Value =
            send_with_retry(limiter.clone(), fast_policy(1), get(&server))
                .await
                .unwrap();
        assert_eq!(
            empty,
            serde_json::Value::Null,
            "an empty body decodes as JSON null"
        );

        let error =
            send_with_retry::<serde_json::Value>(limiter.clone(), fast_policy(1), get(&server))
                .await
                .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)), "{error:?}");
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert!(
            error
                .to_string()
                .starts_with("Rentman API response body is invalid JSON: "),
            "{error}"
        );

        let error = send_with_retry::<serde_json::Value>(limiter, fast_policy(1), get(&server))
            .await
            .unwrap_err();
        // The 200 status already arrived, so the request was applied.
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        let ErrorKind::Transport(error) = error.kind() else {
            panic!("expected a transport error, got {error:?}");
        };
        assert!(error.is_body() || error.is_decode(), "{error:#}");

        let error = send_with_retry::<serde_json::Value>(
            Arc::new(fast_limiter()),
            fast_policy(1),
            get(&server),
        )
        .await
        .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::NOT_FOUND));
    }
}
