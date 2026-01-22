//! Error types for the Rentman API client.

use thiserror::Error;

/// Result type for Rentman API operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur when using the Rentman API client.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error {status}: {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from API
        message: String,
    },

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Authentication error
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {0} seconds")]
    RateLimitExceeded(u64),

    /// Invalid response from API
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl Error {
    /// Create a new API error.
    pub fn api_error(status: u16, message: impl Into<String>) -> Self {
        Self::ApiError {
            status,
            message: message.into(),
        }
    }

    /// Create a new configuration error.
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
    }

    /// Create a new authentication error.
    pub fn auth(message: impl Into<String>) -> Self {
        Self::AuthError(message.into())
    }
}
