//! Validated client configuration.

use reqwest::Url;
use std::{num::NonZeroUsize, time::Duration};

/// Environment variable read by [`RentmanClientConfig::from_env`] for
/// [`RentmanClientConfig::max_attempts`].
pub const MAX_ATTEMPTS_VAR: &str = "RENTMAN_API_MAX_ATTEMPTS";
/// Environment variable read by [`RentmanClientConfig::from_env`] for
/// [`RentmanClientConfig::retry_base_delay`], in milliseconds.
pub const RETRY_BASE_DELAY_VAR: &str = "RENTMAN_RETRY_BASE_DELAY_MS";
/// Environment variable read by [`RentmanClientConfig::from_env`] for
/// [`RentmanClientConfig::min_request_interval`], in milliseconds.
pub const MIN_REQUEST_INTERVAL_VAR: &str = "RENTMAN_REQUEST_MIN_INTERVAL_MS";

const DEFAULT_MAX_ATTEMPTS: NonZeroUsize = NonZeroUsize::new(4).expect("4 is not zero");
const DEFAULT_RETRY_BASE_DELAY: Duration = Duration::from_millis(100);
const DEFAULT_MIN_REQUEST_INTERVAL: Duration = Duration::from_millis(100);

/// Invalid client setting.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid Rentman client setting {setting} = {value:?}: {reason}")]
pub struct ConfigError {
    setting: &'static str,
    value: String,
    reason: &'static str,
}

impl ConfigError {
    fn new(setting: &'static str, value: impl Into<String>, reason: &'static str) -> Self {
        Self {
            setting,
            value: value.into(),
            reason,
        }
    }

    /// Name of the setting, e.g. the environment variable.
    pub fn setting(&self) -> &'static str {
        self.setting
    }

    /// The rejected value as given.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Why the value was rejected.
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

/// Retry and rate-limit settings of a [`RentmanClient`](crate::RentmanClient).
///
/// [`Default`] gives 4 attempts, a 100 ms retry base delay, and at least
/// 100 ms between requests. Nothing is read from the environment unless
/// [`RentmanClientConfig::from_env`] is called.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RentmanClientConfig {
    max_attempts: NonZeroUsize,
    retry_base_delay: Duration,
    min_request_interval: Duration,
}

impl Default for RentmanClientConfig {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            retry_base_delay: DEFAULT_RETRY_BASE_DELAY,
            min_request_interval: DEFAULT_MIN_REQUEST_INTERVAL,
        }
    }
}

impl RentmanClientConfig {
    /// Reads the settings from the environment variables
    /// [`MAX_ATTEMPTS_VAR`], [`RETRY_BASE_DELAY_VAR`], and
    /// [`MIN_REQUEST_INTERVAL_VAR`].
    ///
    /// Unset or empty variables keep their default. A variable that is set to
    /// an invalid value is an error; it never falls back to the default.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(env_var)
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        if let Some(value) = set_value(&lookup, MAX_ATTEMPTS_VAR) {
            let attempts = value.trim().parse::<NonZeroUsize>().map_err(|_| {
                ConfigError::new(MAX_ATTEMPTS_VAR, &value, "must be an integer >= 1")
            })?;
            config = config.with_max_attempts(attempts);
        }
        if let Some(value) = set_value(&lookup, RETRY_BASE_DELAY_VAR) {
            config = config.with_retry_base_delay(parse_millis(RETRY_BASE_DELAY_VAR, &value)?);
        }
        if let Some(value) = set_value(&lookup, MIN_REQUEST_INTERVAL_VAR) {
            let interval = parse_millis(MIN_REQUEST_INTERVAL_VAR, &value)?;
            config = config
                .with_min_request_interval(interval)
                .map_err(|error| {
                    ConfigError::new(MIN_REQUEST_INTERVAL_VAR, &value, error.reason)
                })?;
        }
        Ok(config)
    }

    /// Sets how often a request is sent at most, including the first attempt.
    pub fn with_max_attempts(mut self, max_attempts: NonZeroUsize) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Sets the first retry delay; later retries back off exponentially up to
    /// 16 times this value. A `Retry-After` from Rentman takes precedence.
    pub fn with_retry_base_delay(mut self, delay: Duration) -> Self {
        self.retry_base_delay = delay;
        self
    }

    /// Sets the minimum time between two requests of the client.
    ///
    /// Fails for a zero interval, which would disable the rate limit.
    pub fn with_min_request_interval(mut self, interval: Duration) -> Result<Self, ConfigError> {
        if interval.is_zero() {
            return Err(ConfigError::new(
                "min_request_interval",
                format!("{interval:?}"),
                "must be greater than zero",
            ));
        }
        self.min_request_interval = interval;
        Ok(self)
    }

    /// How often a request is sent at most, including the first attempt.
    pub fn max_attempts(&self) -> NonZeroUsize {
        self.max_attempts
    }

    /// First retry delay.
    pub fn retry_base_delay(&self) -> Duration {
        self.retry_base_delay
    }

    /// Minimum time between two requests of the client.
    pub fn min_request_interval(&self) -> Duration {
        self.min_request_interval
    }
}

/// Accepts only base URLs that relative API paths can be joined onto.
pub(crate) fn validate_base_url(base_url: &Url) -> Result<(), ConfigError> {
    let is_http = matches!(base_url.scheme(), "http" | "https");
    if !is_http || base_url.cannot_be_a_base() {
        return Err(ConfigError::new(
            "base_url",
            base_url.as_str(),
            "must be an http or https URL",
        ));
    }
    Ok(())
}

/// Reads an environment variable; a non-Unicode value is returned lossily so
/// that it is reported as invalid instead of being ignored.
fn env_var(name: &str) -> Option<String> {
    std::env::var_os(name).map(|value| value.to_string_lossy().into_owned())
}

/// Returns the variable's value unless it is unset or blank.
fn set_value(lookup: &impl Fn(&str) -> Option<String>, name: &str) -> Option<String> {
    lookup(name).filter(|value| !value.trim().is_empty())
}

fn parse_millis(setting: &'static str, value: &str) -> Result<Duration, ConfigError> {
    value
        .trim()
        .parse::<u64>()
        .map(Duration::from_millis)
        .map_err(|_| ConfigError::new(setting, value, "must be a whole number of milliseconds"))
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(vars: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        move |name| vars.get(name).cloned()
    }

    #[test]
    fn unset_and_blank_variables_keep_the_defaults() {
        assert_eq!(
            RentmanClientConfig::from_lookup(lookup(&[])),
            Ok(RentmanClientConfig::default())
        );
        let blank = lookup(&[
            (MAX_ATTEMPTS_VAR, ""),
            (RETRY_BASE_DELAY_VAR, "  "),
            (MIN_REQUEST_INTERVAL_VAR, ""),
        ]);
        assert_eq!(
            RentmanClientConfig::from_lookup(blank),
            Ok(RentmanClientConfig::default())
        );

        let config = RentmanClientConfig::default();
        assert_eq!(config.max_attempts().get(), 4);
        assert_eq!(config.retry_base_delay(), Duration::from_millis(100));
        assert_eq!(config.min_request_interval(), Duration::from_millis(100));
    }

    #[test]
    fn reads_valid_values() {
        let config = RentmanClientConfig::from_lookup(lookup(&[
            (MAX_ATTEMPTS_VAR, " 2 "),
            (RETRY_BASE_DELAY_VAR, "0"),
            (MIN_REQUEST_INTERVAL_VAR, "250"),
        ]))
        .unwrap();

        assert_eq!(config.max_attempts().get(), 2);
        assert_eq!(config.retry_base_delay(), Duration::ZERO);
        assert_eq!(config.min_request_interval(), Duration::from_millis(250));
    }

    #[test]
    fn rejects_invalid_values_naming_setting_and_value() {
        for (name, value, reason) in [
            (MAX_ATTEMPTS_VAR, "0", "must be an integer >= 1"),
            (MAX_ATTEMPTS_VAR, "three", "must be an integer >= 1"),
            (
                RETRY_BASE_DELAY_VAR,
                "-5",
                "must be a whole number of milliseconds",
            ),
            (
                MIN_REQUEST_INTERVAL_VAR,
                "1.5",
                "must be a whole number of milliseconds",
            ),
            (MIN_REQUEST_INTERVAL_VAR, "0", "must be greater than zero"),
        ] {
            let error = RentmanClientConfig::from_lookup(lookup(&[(name, value)])).unwrap_err();

            assert_eq!(error.setting(), name);
            assert_eq!(error.value(), value);
            assert_eq!(error.reason(), reason);
            assert_eq!(
                error.to_string(),
                format!("invalid Rentman client setting {name} = {value:?}: {reason}")
            );
        }
    }

    #[test]
    fn builder_rejects_a_zero_request_interval() {
        let error = RentmanClientConfig::default()
            .with_min_request_interval(Duration::ZERO)
            .unwrap_err();

        assert_eq!(error.setting(), "min_request_interval");
        assert_eq!(error.reason(), "must be greater than zero");
    }

    #[test]
    fn reads_the_process_environment() {
        assert!(env_var("PATH").is_some());
        assert_eq!(
            env_var("RENTMAN_CLIENT_TEST_VARIABLE_THAT_IS_NEVER_SET"),
            None
        );
        assert_eq!(
            RentmanClientConfig::from_env(),
            RentmanClientConfig::from_lookup(env_var)
        );
    }

    #[test]
    fn accepts_only_http_base_urls() {
        assert_eq!(
            validate_base_url(&Url::parse("https://api.rentman.net/").unwrap()),
            Ok(())
        );
        assert_eq!(
            validate_base_url(&Url::parse("http://127.0.0.1:8080/api").unwrap()),
            Ok(())
        );
        for url in ["ftp://example.test/", "mailto:api@example.test"] {
            let error = validate_base_url(&Url::parse(url).unwrap()).unwrap_err();
            assert_eq!(error.setting(), "base_url");
            assert_eq!(error.value(), url);
        }
    }
}
