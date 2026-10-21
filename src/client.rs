//! The Rentman API client and its internal request builder.

use crate::{
    RentmanApiToken,
    custom_fields::{CustomFieldRegistry, CustomFieldsResource},
    transport::{CompositeRateLimiter, RetryPolicy, send_with_retry, send_with_retry_unit},
};
use crate::{
    config::{ConfigError, RentmanClientConfig, validate_base_url},
    error::Result,
};
use reqwest::{Client as HttpClient, Method, Url};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const DEFAULT_BASE_URL: &str = "https://api.rentman.net";

/// HTTP client for calling the Rentman API.
#[derive(Clone)]
pub struct RentmanClient {
    pub(crate) base_url: Url,
    token: RentmanApiToken,
    http: HttpClient,
    /// Client for requests that change data. It does not follow redirects:
    /// after a redirect, a failure on a later hop would hide that the first
    /// server may already have applied the write.
    http_writes: HttpClient,
    limiter: Arc<CompositeRateLimiter>,
    pub(crate) retry_policy: RetryPolicy,
    pub(crate) custom_fields: Arc<CustomFieldRegistry>,
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

    async fn send_unit_with(self, retry_policy: RetryPolicy) -> Result<()> {
        send_with_retry_unit(self.client.limiter, retry_policy, self.request).await
    }
}

impl RentmanClient {
    /// Creates a client for the Rentman API with the default configuration.
    ///
    /// Nothing is read from the environment; use [`Self::with_config`] with
    /// [`RentmanClientConfig::from_env`] for that.
    pub fn new(token: RentmanApiToken) -> Self {
        Self::build(
            Url::parse(DEFAULT_BASE_URL).expect("default URL is valid"),
            token,
            RentmanClientConfig::default(),
        )
    }

    /// Creates a client for an explicit base URL with the default
    /// configuration, e.g. for tests, staging environments, or local proxies.
    ///
    /// Fails unless the base URL is an `http` or `https` URL.
    pub fn with_base_url(base_url: Url, token: RentmanApiToken) -> Result<Self, ConfigError> {
        Self::with_config(base_url, token, RentmanClientConfig::default())
    }

    /// Creates a client for an explicit base URL and configuration.
    ///
    /// Fails unless the base URL is an `http` or `https` URL.
    pub fn with_config(
        base_url: Url,
        token: RentmanApiToken,
        config: RentmanClientConfig,
    ) -> Result<Self, ConfigError> {
        validate_base_url(&base_url)?;
        Ok(Self::build(base_url, token, config))
    }

    fn build(mut base_url: Url, token: RentmanApiToken, config: RentmanClientConfig) -> Self {
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        Self {
            base_url,
            token,
            http: HttpClient::new(),
            http_writes: HttpClient::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("an HTTP client without redirects builds like the default client"),
            retry_policy: RetryPolicy::from_config(&config),
            limiter: Arc::new(CompositeRateLimiter::new(config.min_request_interval())),
            custom_fields: Arc::new(CustomFieldRegistry::default()),
        }
    }

    /// Registers the account-specific custom fields that responses must contain.
    pub fn with_custom_fields(mut self, custom_fields: CustomFieldRegistry) -> Self {
        self.custom_fields = Arc::new(custom_fields);
        self
    }

    pub(crate) fn apply_custom_fields<R: CustomFieldsResource>(&self, item: &mut R) -> Result<()> {
        self.custom_fields
            .apply(R::COLLECTION, item.custom_fields_mut())
    }

    pub(crate) fn custom_fields_registered<R: crate::id::Resource>(&self) -> bool {
        self.custom_fields.has_fields(R::COLLECTION)
    }

    pub(crate) fn request(&self, method: Method, path: &str) -> RentmanRequestBuilder {
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            // The base URL was validated as an http(s) URL, which can be a base.
            .expect("relative API path can be joined onto the base URL");
        self.request_url(method, url)
    }

    pub(crate) fn request_url(&self, method: Method, url: Url) -> RentmanRequestBuilder {
        RentmanRequestBuilder {
            client: self.clone(),
            request: if method.is_safe() {
                &self.http
            } else {
                &self.http_writes
            }
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
    use crate::{RentmanApiToken, RentmanClient, RentmanClientConfig, Url, transport::RetryPolicy};
    use std::{num::NonZeroUsize, time::Duration};

    #[test]
    fn configures_default_and_custom_base_urls() {
        let default_client = RentmanClient::new(RentmanApiToken::new("token"));
        assert_eq!(default_client.base_url.as_str(), "https://api.rentman.net/");

        let custom_client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();
        assert_eq!(custom_client.base_url.as_str(), "http://example.test/");
    }

    #[test]
    fn normalizes_base_urls_without_trailing_slash() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/api").unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();
        assert_eq!(client.base_url.as_str(), "http://example.test/api/");
    }

    #[test]
    fn rejects_base_urls_that_api_paths_cannot_be_joined_onto() {
        let error = match RentmanClient::with_base_url(
            Url::parse("mailto:api@example.test").unwrap(),
            RentmanApiToken::new("token"),
        ) {
            Ok(_) => panic!("mailto URL must be rejected"),
            Err(error) => error,
        };
        assert_eq!(error.setting(), "base_url");
    }

    #[test]
    fn applies_the_configuration() {
        let config = RentmanClientConfig::default()
            .with_max_attempts(NonZeroUsize::new(2).unwrap())
            .with_retry_base_delay(Duration::from_millis(7));
        let client = RentmanClient::with_config(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
            config,
        )
        .unwrap();

        assert_eq!(client.retry_policy, RetryPolicy::from_config(&config));
    }
}
