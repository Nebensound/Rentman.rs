//! Credentials.

use std::fmt;

/// API token for Rentman.
///
/// This is a dedicated type instead of a `String` so the token cannot be mixed
/// up with unrelated strings or tokens. `Debug` is redacted so the token does
/// not accidentally end up in logs.
#[derive(Clone)]
pub struct RentmanApiToken(String);

impl RentmanApiToken {
    /// Returns the secret token value for the `Authorization` header.
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }

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
