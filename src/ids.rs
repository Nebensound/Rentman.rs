//! Identifiers and credentials.

use serde::Deserialize;
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

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::{InvoiceNumber, RentmanApiToken};

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
}
