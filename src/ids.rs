//! Credentials.

use std::fmt;

/// API token for Rentman.
///
/// This is a dedicated type instead of a `String` so the token cannot be mixed
/// up with unrelated strings or tokens. `Debug` is redacted so the token does
/// not accidentally end up in logs.
///
/// ```
/// use rentman_client::RentmanApiToken;
///
/// let token = RentmanApiToken::new("secret-value");
/// assert!(!format!("{token:?}").contains("secret-value"));
/// ```
#[derive(Clone)]
pub struct RentmanApiToken(String);

impl RentmanApiToken {
    /// Returns the secret token value for the `Authorization` header.
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }

    /// Creates a Rentman API token.
    ///
    /// ```
    /// use rentman_client::RentmanApiToken;
    ///
    /// let from_str = RentmanApiToken::new("token");
    /// let from_env = RentmanApiToken::new(std::env::var("RENTMAN_TOKEN").unwrap_or_default());
    /// # let _ = (from_str, from_env);
    /// ```
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Debug for RentmanApiToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RentmanApiToken(***)")
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::RentmanApiToken;

    #[test]
    fn redacts_token_in_debug_output() {
        assert_eq!(
            format!("{:?}", RentmanApiToken::new("secret")),
            "RentmanApiToken(***)"
        );
    }
}
