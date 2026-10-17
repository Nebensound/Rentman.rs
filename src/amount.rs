//! Exact decimal amounts at the Rentman API boundary.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// Exact decimal amount at the Rentman API boundary.
///
/// Rentman reads and writes amounts as bare numbers without a currency field.
/// This type makes the API boundary explicit and prevents floating point values
/// for money-like amounts: JSON numbers are read and written from their exact
/// decimal text, never through `f64`. Values with more than 28 significant
/// digits are rejected instead of being rounded.
///
/// The type deliberately has no currency and no arithmetic: Rentman's API has
/// no currency field per amount, the currency is an account setting. It is a
/// wire-boundary type, not a money type for domain logic. Convert it into your
/// own currency-carrying money type at the boundary, so amounts of different
/// currencies or meanings (net and gross) cannot be mixed. See "Deviations" in
/// the [README](https://github.com/Nebensound/Rentman.rs#deviations).
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
        // `Decimal`'s own serde-float support would go through `f64`; the raw
        // value writes the exact decimal text as a JSON number instead.
        RawValue::from_string(self.0.to_string())
            .expect("a decimal always formats as a valid JSON number")
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RentmanAmount {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        parse_exact_amount(raw.get()).map_err(serde::de::Error::custom)
    }
}

fn parse_exact_amount(text: &str) -> Result<RentmanAmount, String> {
    let is_number = text.starts_with(|c: char| c == '-' || c.is_ascii_digit());
    if !is_number {
        return Err(format!("Rentman amount must be a JSON number, got {text}"));
    }
    let parsed = match text.split_once(['e', 'E']) {
        Some((significand, exponent)) => parse_exact_scientific(significand, exponent),
        None => Decimal::from_str_exact(trim_fraction_zeros(text)).ok(),
    };
    parsed
        .map(RentmanAmount)
        .ok_or_else(|| format!("Rentman amount {text} is not an exact decimal"))
}

/// Drops trailing zeros of a fractional part, which do not change the value
/// but would count against the 28 decimal places of `Decimal`.
fn trim_fraction_zeros(number: &str) -> &str {
    if number.contains('.') {
        number.trim_end_matches('0').trim_end_matches('.')
    } else {
        number
    }
}

/// Applies the exponent to an exactly parsed significand with checked integer
/// arithmetic. `Decimal::from_scientific` would round an over-precise
/// significand instead of rejecting it.
fn parse_exact_scientific(significand: &str, exponent: &str) -> Option<Decimal> {
    let significand = Decimal::from_str_exact(trim_fraction_zeros(significand)).ok()?;
    if significand.is_zero() {
        // Zero is exact for any exponent.
        return Some(Decimal::ZERO);
    }
    let exponent: i64 = exponent.parse().ok()?;
    let mut scale = i64::from(significand.scale()).checked_sub(exponent)?;
    if scale >= 0 {
        // Trailing powers of ten do not change the value: `10e-29` is
        // exactly `1e-28`. A non-zero mantissa has at most 38 of them.
        let mut mantissa = significand.mantissa();
        while scale > 28 && mantissa % 10 == 0 {
            mantissa /= 10;
            scale -= 1;
        }
        Decimal::try_from_i128_with_scale(mantissa, u32::try_from(scale).ok()?).ok()
    } else {
        let factor = 10_i128.checked_pow(u32::try_from(-scale).ok()?)?;
        Decimal::try_from_i128_with_scale(significand.mantissa().checked_mul(factor)?, 0).ok()
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    use crate::{EquipmentId, EquipmentPricePayload, RentmanAmount};

    use proptest::prelude::*;

    use rust_decimal::Decimal;

    #[test]
    fn exposes_amount_decimal_without_float_rounding() {
        let decimal = Decimal::new(425, 1);
        let amount = RentmanAmount::from(decimal);

        assert_eq!(amount.as_decimal(), decimal);
    }

    #[test]
    fn reads_amounts_exactly_and_exposes_equipment_ids() {
        let amount: RentmanAmount = serde_json::from_str("19.99").unwrap();
        assert_eq!(amount.as_decimal(), Decimal::new(1999, 2));
        let scientific: RentmanAmount = serde_json::from_str("1.5e2").unwrap();
        assert_eq!(scientific.as_decimal(), Decimal::new(150, 0));
        let small: RentmanAmount = serde_json::from_str("1.5E-2").unwrap();
        assert_eq!(small.as_decimal(), Decimal::new(15, 3));
        let negative: RentmanAmount = serde_json::from_str("-0.01").unwrap();
        assert_eq!(negative.as_decimal(), Decimal::new(-1, 2));
        assert!(serde_json::from_str::<RentmanAmount>("\"expensive\"").is_err());
        assert!(serde_json::from_str::<RentmanAmount>("true").is_err());
        assert!(serde_json::from_str::<RentmanAmount>("").is_err());
        let error = serde_json::from_str::<RentmanAmount>("0.12345678901234567890123456789")
            .unwrap_err()
            .to_string();
        assert!(error.contains("is not an exact decimal"), "{error}");
        assert_eq!(EquipmentId::new(2163).to_string(), "2163");
    }

    #[test]
    fn accepts_exact_amounts_with_trailing_zeros() {
        let smallest = Decimal::new(1, 28);
        for text in [
            "1e-28",
            "1.0e-28",
            "10e-29",
            "1000e-31",
            "0.00000000000000000000000000010",
        ] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert_eq!(amount.as_decimal(), smallest, "{text}");
        }
        for (text, expected) in [("10.0", Decimal::new(10, 0)), ("100", Decimal::new(100, 0))] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert_eq!(amount.as_decimal(), expected, "{text}");
        }
        for text in ["0e-50", "0.0e5000000000", "-0e3"] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert!(amount.as_decimal().is_zero(), "{text}");
        }
    }

    #[test]
    fn rejects_scientific_amounts_that_are_not_exact() {
        for text in [
            // Over-precise significand: would be rounded by `from_scientific`.
            "0.12345678901234567890123456789e0",
            // Exponent outside `i64`.
            "1e99999999999999999999",
            // Scale computation overflows.
            "1e-9223372036854775808",
            // Scale outside `u32`, or beyond the 28 decimal places of `Decimal`.
            "1e-5000000000",
            "1e-29",
            // Exponent outside `u32`, power of ten or product outside `i128`.
            "1e5000000000",
            "1e39",
            "20e38",
            // Integer beyond the 96 bits of `Decimal`.
            "1e30",
        ] {
            let error = serde_json::from_str::<RentmanAmount>(text)
                .unwrap_err()
                .to_string();
            assert!(error.contains("is not an exact decimal"), "{text}: {error}");
        }
    }

    /// `Decimal` values within the 96-bit mantissa and 28 decimal places.
    fn any_decimal() -> impl Strategy<Value = Decimal> {
        let max_mantissa = (1_i128 << 96) - 1;
        (-max_mantissa..=max_mantissa, 0_u32..=28)
            .prop_map(|(mantissa, scale)| Decimal::from_i128_with_scale(mantissa, scale))
    }

    proptest! {
        #[test]
        fn amount_round_trips_through_json(value in any_decimal()) {
            let json = serde_json::to_string(&RentmanAmount::new(value)).unwrap();
            let parsed: RentmanAmount = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(parsed.as_decimal(), value, "{}", json);
        }

        #[test]
        fn scientific_and_plain_notation_parse_to_the_same_amount(
            mantissa in any::<i64>(),
            scale in 0_u32..=28,
            extra_zeros in 0_u32..=5,
            upper_case in any::<bool>(),
        ) {
            let expected = Decimal::from_i128_with_scale(i128::from(mantissa), scale);
            let e = if upper_case { 'E' } else { 'e' };
            // `{m}{zeros}e-{scale + zeros}` is the same value with trailing
            // powers of ten moved into the exponent.
            let zeros = "0".repeat(extra_zeros as usize);
            let shifted = format!("{mantissa}{zeros}{e}-{}", scale + extra_zeros);
            let plain = expected.to_string();

            prop_assert_eq!(parse_exact_amount(&shifted).unwrap().as_decimal(), expected);
            prop_assert_eq!(parse_exact_amount(&plain).unwrap().as_decimal(), expected);
        }

        #[test]
        fn positive_exponents_multiply_exactly(
            mantissa in any::<i64>(),
            exponent in 0_u32..=9,
        ) {
            let expected = Decimal::from(mantissa) * Decimal::from(10_i64.pow(exponent));
            let text = format!("{mantissa}e{exponent}");

            prop_assert_eq!(parse_exact_amount(&text).unwrap().as_decimal(), expected);
        }

        #[test]
        fn over_precise_amounts_are_rejected_not_rounded(
            digits in "[0-9]{28,40}",
            last in 1_u8..=9,
            negative in any::<bool>(),
        ) {
            // At least 29 decimal places with a non-zero last digit cannot be
            // represented exactly by `Decimal`.
            let sign = if negative { "-" } else { "" };
            let text = format!("{sign}0.{digits}{last}");

            let error = parse_exact_amount(&text).unwrap_err();
            prop_assert!(error.contains("is not an exact decimal"), "{}", error);
        }

        #[test]
        fn number_like_input_never_panics(
            text in "-?[0-9]{1,40}(\\.[0-9]{0,40})?([eE][-+]?[0-9]{1,25})?",
        ) {
            if let Ok(amount) = parse_exact_amount(&text) {
                // Whatever is accepted is the exact value: it formats back to
                // a number that parses to the same decimal.
                let again = parse_exact_amount(&amount.as_decimal().to_string()).unwrap();
                prop_assert_eq!(again, amount);
            }
        }

        #[test]
        fn arbitrary_input_never_panics(text in any::<String>()) {
            let _ = parse_exact_amount(&text);
            let _ = serde_json::from_str::<RentmanAmount>(&text);
        }
    }

    #[test]
    fn keeps_amounts_beyond_f64_precision_exact() {
        // 19 significant digits: an `f64` round trip would lose the tail.
        let text = "0.1234567890123456789";
        let expected: Decimal = text.parse().unwrap();

        let amount: RentmanAmount = serde_json::from_str(text).unwrap();
        assert_eq!(amount.as_decimal(), expected);
        assert_eq!(serde_json::to_string(&amount).unwrap(), text);

        let payload = EquipmentPricePayload {
            list_price: Some(amount),
            ..EquipmentPricePayload::default()
        };
        assert_eq!(
            serde_json::to_string(&payload).unwrap(),
            r#"{"list_price":0.1234567890123456789}"#
        );
    }
}
