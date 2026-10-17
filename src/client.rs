//! The Rentman API client and its untyped request builder.

use crate::{
    RentmanApiToken, RentmanEndpointRequest, endpoint,
    transport::{
        CompositeRateLimiter, RetryPolicy, configured_duration_ms, send_with_retry,
        send_with_retry_unit,
    },
};
use anyhow::Result;
use reqwest::{Client as HttpClient, Method, Url};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

const DEFAULT_RENTMAN_MIN_INTERVAL: Duration = Duration::from_millis(100);

pub(crate) const DEFAULT_PAGE_LIMIT: usize = 1500;

/// HTTP client for calling the Rentman API.
#[derive(Clone)]
pub struct RentmanClient {
    pub(crate) base_url: Url,
    token: RentmanApiToken,
    http: HttpClient,
    limiter: Arc<CompositeRateLimiter>,
    pub(crate) retry_policy: RetryPolicy,
}

pub(crate) struct RentmanRequestBuilder {
    client: RentmanClient,
    request: reqwest::RequestBuilder,
}

impl RentmanRequestBuilder {
    pub(crate) fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);
        self
    }

    pub(crate) fn json<T: Serialize + ?Sized>(mut self, payload: &T) -> Self {
        self.request = self.request.json(payload);
        self
    }

    pub(crate) async fn send_json<T: for<'de> Deserialize<'de>>(self) -> Result<T> {
        let retry_policy = self.client.retry_policy;
        self.send_json_with(retry_policy).await
    }

    pub(crate) async fn send_json_with<T: for<'de> Deserialize<'de>>(
        self,
        retry_policy: RetryPolicy,
    ) -> Result<T> {
        send_with_retry(self.client.limiter, retry_policy, self.request).await
    }

    pub(crate) async fn send_unit(self) -> Result<()> {
        let retry_policy = self.client.retry_policy;
        self.send_unit_with(retry_policy).await
    }

    /// Sends a non-idempotent write exactly once.
    pub(crate) async fn send_unit_once(self) -> Result<()> {
        let retry_policy = self.client.retry_policy.single_attempt();
        self.send_unit_with(retry_policy).await
    }

    async fn send_unit_with(self, retry_policy: RetryPolicy) -> Result<()> {
        send_with_retry_unit(self.client.limiter, retry_policy, self.request).await
    }
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
        RentmanEndpointRequest::new(self.clone())
    }

    pub(crate) fn paginated_request(
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

    pub(crate) fn request(&self, method: Method, path: &str) -> RentmanRequestBuilder {
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            .expect("relative API path can be joined onto the base URL");
        self.request_url(method, url)
    }

    pub(crate) fn request_url(&self, method: Method, url: Url) -> RentmanRequestBuilder {
        RentmanRequestBuilder {
            client: self.clone(),
            request: self
                .http
                .request(method, url)
                .bearer_auth(self.token.expose()),
        }
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::{RentmanApiToken, RentmanClient, Url};

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
}
