//! Exact decimal amounts at the Rentman API boundary.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// Exact decimal amount at the Rentman API boundary.
///
/// Rentman reads and writes amounts as bare numbers without a currency field.
/// This type makes the API boundary explicit and prevents floating point values
/// for money-like amounts: JSON numbers are read and written from their exact
/// decimal text, never through `f64`. A value with more precision than
/// `Decimal` holds (more than 28 decimal places, or more digits than its
/// 96-bit mantissa) is rounded half away from zero to the nearest value that
/// fits, and a warning is logged through `tracing`. Only a value whose integer
/// part does not fit `Decimal` is rejected.
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
        parse_amount(raw.get()).map_err(serde::de::Error::custom)
    }
}

fn parse_amount(text: &str) -> Result<RentmanAmount, String> {
    let is_number = text.starts_with(|c: char| c == '-' || c.is_ascii_digit());
    if !is_number {
        return Err(format!("Rentman amount must be a JSON number, got {text}"));
    }
    match parse_decimal(text) {
        Some(ParsedDecimal::Exact(decimal)) => Ok(RentmanAmount(decimal)),
        Some(ParsedDecimal::Rounded(decimal)) => {
            tracing::warn!(
                amount = text,
                rounded = %decimal,
                "Rentman amount has more precision than Decimal holds and was rounded"
            );
            Ok(RentmanAmount(decimal))
        }
        None => Err(format!("Rentman amount {text} does not fit a decimal")),
    }
}

/// A parsed JSON number: either its exact value, or the nearest value that
/// fits `Decimal`.
enum ParsedDecimal {
    Exact(Decimal),
    Rounded(Decimal),
}

/// Parses a JSON number, or returns `None` when it is no number or its integer
/// part does not fit `Decimal`.
///
/// A number that fits `Decimal` as written keeps its scale, so `12.50` stays
/// `12.50` and `0.00` keeps two decimal places. Otherwise the number is
/// normalized as text into significant digits and a power of ten, and
/// `Decimal`'s bounds (96-bit mantissa, 28 decimal places) are checked only on
/// that normalized form. So `0.00000000000000000000000000001e1` and
/// `100000000000000000000000000000e-1` are exact. A value that still does not
/// fit, such as `1e-29` or one with more digits than the mantissa holds, is
/// rounded half away from zero instead of going through
/// `Decimal::from_scientific`, whose rounding is not documented.
fn parse_decimal(text: &str) -> Option<ParsedDecimal> {
    let (negative, unsigned) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (significand, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((significand, exponent)) => (significand, parse_exponent(exponent)?),
        None => (unsigned, 0),
    };
    let (integer, fraction) = significand.split_once('.').unwrap_or((significand, ""));
    let is_digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    if integer.is_empty() || !is_digits(integer) || !is_digits(fraction) {
        return None;
    }

    let digits = format!("{integer}{fraction}");
    // `i128` holds every scale and power that `i64` exponents and string
    // lengths produce.
    let written_scale = fraction.len() as i128 - i128::from(exponent);
    if let Some(decimal) = decimal_as_written(&digits, written_scale, negative) {
        return Some(ParsedDecimal::Exact(decimal));
    }

    // value = significant × 10^power, with leading and trailing zeros removed.
    let digits = digits.trim_start_matches('0');
    let significant = digits.trim_end_matches('0');
    if significant.is_empty() {
        // Zero is exact for any exponent.
        return Some(ParsedDecimal::Exact(Decimal::ZERO));
    }
    let trailing_zeros = digits.len() - significant.len();
    let power = trailing_zeros as i128 - written_scale;
    let sign = |magnitude: i128| if negative { -magnitude } else { magnitude };

    let magnitude = significant.parse::<i128>().ok();
    if power >= 0 {
        // An integer that does not fit cannot be rounded without changing its
        // order of magnitude.
        let factor = 10_i128.checked_pow(u32::try_from(power).ok()?)?;
        let mantissa = sign(magnitude?).checked_mul(factor)?;
        return Decimal::try_from_i128_with_scale(mantissa, 0)
            .ok()
            .map(ParsedDecimal::Exact);
    }
    let scale = -power;
    let exact = magnitude
        .zip(u32::try_from(scale).ok().filter(|scale| *scale <= 28))
        .and_then(|(magnitude, scale)| {
            Decimal::try_from_i128_with_scale(sign(magnitude), scale).ok()
        });
    match exact {
        Some(decimal) => Some(ParsedDecimal::Exact(decimal)),
        None => round_to_fit(significant, scale, negative).map(ParsedDecimal::Rounded),
    }
}

/// Rounds `significant × 10^-scale` half away from zero to the largest scale
/// (at most 28) at which `Decimal` can hold it, or `None` when not even the
/// integer part fits.
fn round_to_fit(significant: &str, scale: i128, negative: bool) -> Option<Decimal> {
    let length = significant.len() as i128;
    for target in (0..=scale.min(28)).rev() {
        let dropped = scale - target;
        let (kept, next_digit) = if dropped > length {
            ("", b'0')
        } else {
            let split = (length - dropped) as usize;
            let next_digit = significant.as_bytes().get(split).copied().unwrap_or(b'0');
            (&significant[..split], next_digit)
        };
        let kept: i128 = match kept {
            "" => 0,
            digits => match digits.parse() {
                Ok(kept) => kept,
                Err(_) => continue,
            },
        };
        // Saturating: a saturated value does not fit `Decimal` either.
        let magnitude = kept.saturating_add(i128::from(next_digit >= b'5'));
        if magnitude == 0 {
            return Some(Decimal::ZERO);
        }
        let mantissa = if negative { -magnitude } else { magnitude };
        if let Ok(decimal) = Decimal::try_from_i128_with_scale(mantissa, target as u32) {
            return Some(decimal);
        }
    }
    None
}

/// Parses a JSON exponent (optional sign, then digits). An exponent beyond
/// `i64` is saturated: every exponent that large has the same outcome (zero
/// stays exact, a tiny value rounds to zero, a huge one is rejected), so the
/// saturated value keeps those cases reachable instead of failing the parse.
fn parse_exponent(text: &str) -> Option<i64> {
    let (negative, digits) = match text.as_bytes().first() {
        Some(b'-') => (true, &text[1..]),
        Some(b'+') => (false, &text[1..]),
        _ => (false, text),
    };
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    // Validated digits only fail to parse by overflowing.
    let magnitude = digits.parse::<i64>().unwrap_or(i64::MAX);
    Some(if negative { -magnitude } else { magnitude })
}

/// The decimal with exactly the written digits and scale, if `Decimal` can
/// hold it without normalizing.
fn decimal_as_written(digits: &str, scale: i128, negative: bool) -> Option<Decimal> {
    let scale = u32::try_from(scale).ok().filter(|scale| *scale <= 28)?;
    let magnitude: i128 = match digits.trim_start_matches('0') {
        "" => 0,
        significant => significant.parse().ok()?,
    };
    let mantissa = if negative { -magnitude } else { magnitude };
    Decimal::try_from_i128_with_scale(mantissa, scale).ok()
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::capture_warnings;
    use bigdecimal::{BigDecimal, RoundingMode, Signed, ToPrimitive, num_bigint::BigInt};

    use crate::{RentmanAmount, model::EquipmentPayload};

    use proptest::prelude::*;

    use rust_decimal::Decimal;

    #[test]
    fn exposes_amount_decimal_without_float_rounding() {
        let decimal = Decimal::new(425, 1);
        let amount = RentmanAmount::from(decimal);

        assert_eq!(amount.as_decimal(), decimal);
    }

    #[test]
    fn reads_amounts_exactly() {
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
            // Significand beyond `Decimal`'s scale, final value within it.
            "0.00000000000000000000000000001e1",
        ] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert_eq!(amount.as_decimal(), smallest, "{text}");
        }
        for (text, expected) in [("10.0", Decimal::new(10, 0)), ("100", Decimal::new(100, 0))] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert_eq!(amount.as_decimal(), expected, "{text}");
        }
        for text in [
            "0e-50",
            "0.0e5000000000",
            "-0e3",
            // Zero is exact even with an exponent beyond `i64`.
            "0e9223372036854775808",
            "0e-99999999999999999999",
        ] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert!(amount.as_decimal().is_zero(), "{text}");
        }
    }

    #[test]
    fn keeps_the_written_scale_when_it_fits() {
        for (text, mantissa, scale) in [
            ("12.50", 1250, 2),
            ("0.00", 0, 2),
            ("-0.050", -50, 3),
            ("1.5e2", 150, 0),
            ("1.5e+2", 150, 0),
            ("125e-1", 125, 1),
        ] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            let decimal = amount.as_decimal();
            assert_eq!(decimal, Decimal::new(mantissa, scale), "{text}");
            assert_eq!(decimal.scale(), scale, "{text}");
            assert_eq!(serde_json::to_string(&amount).unwrap(), decimal.to_string());
        }
        // Written with 29 decimal places, so it is normalized to 28.
        let normalized: RentmanAmount =
            serde_json::from_str("0.00000000000000000000000000010").unwrap();
        assert_eq!(normalized.as_decimal().scale(), 28);
    }

    #[test]
    fn normalizes_before_checking_decimal_bounds() {
        // Coefficient beyond 96 bits, final value 1e28 within them.
        let amount: RentmanAmount =
            serde_json::from_str("100000000000000000000000000000e-1").unwrap();
        assert_eq!(
            amount.as_decimal(),
            Decimal::from_i128_with_scale(10_i128.pow(28), 0)
        );
        let negative: RentmanAmount =
            serde_json::from_str("-0.000000000000000000000000000010e1").unwrap();
        assert_eq!(negative.as_decimal(), Decimal::new(-1, 28));
    }

    #[test]
    fn rejects_text_that_is_not_a_json_number() {
        for text in [
            "-", "-.5", "-+5", "1.2.3", "1x", "1.x", "1e", "1e+", "1e5x", "1e--5",
        ] {
            assert!(parse_amount(text).is_err(), "{text}");
        }
    }

    /// `Decimal` values within the 96-bit mantissa and 28 decimal places.
    fn any_decimal() -> impl Strategy<Value = Decimal> {
        let max_mantissa = (1_i128 << 96) - 1;
        (-max_mantissa..=max_mantissa, 0_u32..=28)
            .prop_map(|(mantissa, scale)| Decimal::from_i128_with_scale(mantissa, scale))
    }

    #[test]
    fn rejects_amounts_whose_integer_part_does_not_fit() {
        for text in [
            // Non-zero value with an exponent beyond `i64`.
            "1e99999999999999999999",
            "1e9223372036854775808",
            // Exponent outside `u32`, power of ten or product outside `i128`.
            "1e5000000000",
            "1e39",
            "20e38",
            // Integer beyond the 96 bits of `Decimal`.
            "1e30",
            // Product outside `i128` although the power of ten fits.
            "99e37",
            // Rounding the largest mantissa up leaves the 96 bits.
            "79228162514264337593543950335.5",
            // Integer part beyond 96 bits, whatever is dropped after the point.
            "1234567890123456789012345678901234567890.5",
            "12345678901234567890123456789012345.1234567890",
        ] {
            let error = serde_json::from_str::<RentmanAmount>(text)
                .unwrap_err()
                .to_string();
            assert!(error.contains("does not fit a decimal"), "{text}: {error}");
        }
    }

    #[test]
    fn rounds_over_precise_amounts_half_away_from_zero() {
        let smallest = Decimal::new(1, 28);
        for (text, expected) in [
            (
                "0.12345678901234567890123456789e0",
                Decimal::from_i128_with_scale(1234567890123456789012345679, 28),
            ),
            (
                "0.123456789012345678901234567890123456789",
                Decimal::from_i128_with_scale(1234567890123456789012345679, 28),
            ),
            ("1e-29", Decimal::ZERO),
            ("4e-29", Decimal::ZERO),
            ("5e-29", smallest),
            ("-5e-29", -smallest),
            ("1e-5000000000", Decimal::ZERO),
            // Exponent beyond `i64`: still rounds to zero.
            ("1e-9223372036854775809", Decimal::ZERO),
            ("1e-99999999999999999999", Decimal::ZERO),
            ("1e-9223372036854775808", Decimal::ZERO),
            // 30 digits: rounded to one decimal place so the mantissa fits.
            (
                "1234567890123456789012345678.95",
                Decimal::from_i128_with_scale(12345678901234567890123456790, 1),
            ),
        ] {
            let amount: RentmanAmount = serde_json::from_str(text).unwrap();
            assert_eq!(amount.as_decimal(), expected, "{text}");
            assert!(
                matches!(parse_decimal(text), Some(ParsedDecimal::Rounded(_))),
                "{text} is reported as rounded"
            );
        }
    }

    #[test]
    fn logs_a_warning_only_for_rounded_amounts() {
        let warnings = capture_warnings();

        parse_amount("12.50").unwrap();
        assert!(warnings.events().is_empty(), "exact amounts log nothing");
        parse_amount("5e-29").unwrap();

        let events = warnings.events();
        assert_eq!(events.len(), 1, "{events:?}");
        assert!(events[0].contains("was rounded"), "{events:?}");
        assert!(events[0].contains("amount=\"5e-29\""), "{events:?}");
        assert!(
            events[0].contains("rounded=0.0000000000000000000000000001"),
            "{events:?}"
        );
    }

    fn reference_parse(text: &str) -> Option<(Decimal, bool)> {
        let exact: BigDecimal = text.parse().expect("generated text is a number");
        let max_mantissa = BigInt::from((1_u128 << 96) - 1);
        (0..=28_i64).rev().find_map(|scale| {
            let rounded = exact.with_scale_round(scale, RoundingMode::HalfUp);
            let (mantissa, _) = rounded.as_bigint_and_exponent();
            if mantissa.abs() > max_mantissa {
                return None;
            }
            let decimal = Decimal::from_i128_with_scale(mantissa.to_i128()?, scale as u32);
            Some((decimal, rounded == exact))
        })
    }

    proptest! {
        #[test]
        fn amount_round_trips_through_json(value in any_decimal()) {
            let json = serde_json::to_string(&RentmanAmount::new(value)).unwrap();
            let parsed: RentmanAmount = serde_json::from_str(&json).unwrap();
            // `Decimal` equality ignores the scale, so compare it separately.
            prop_assert_eq!(parsed.as_decimal(), value, "{}", json);
            prop_assert_eq!(parsed.as_decimal().scale(), value.scale(), "{}", json);
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

            prop_assert_eq!(parse_amount(&shifted).unwrap().as_decimal(), expected);
            prop_assert_eq!(parse_amount(&plain).unwrap().as_decimal(), expected);
        }

        #[test]
        fn positive_exponents_multiply_exactly(
            mantissa in any::<i64>(),
            exponent in 0_u32..=9,
        ) {
            let expected = Decimal::from(mantissa) * Decimal::from(10_i64.pow(exponent));
            let text = format!("{mantissa}e{exponent}");

            prop_assert_eq!(parse_amount(&text).unwrap().as_decimal(), expected);
        }

        #[test]
        fn parsing_matches_an_arbitrary_precision_reference(
            text in "-?[0-9]{1,45}(\\.[0-9]{1,45})?([eE][-+]?[0-9]{1,3})?",
        ) {
            // Differential test: an independent `BigDecimal` computation
            // decides the expected value, and whether it is exact or rounded.
            let reference = reference_parse(&text);
            let parsed = parse_decimal(&text).map(|parsed| match parsed {
                ParsedDecimal::Exact(decimal) => (decimal, true),
                ParsedDecimal::Rounded(decimal) => (decimal, false),
            });
            prop_assert_eq!(parsed, reference, "{}", text);
        }

        #[test]
        fn number_like_input_never_panics(
            text in "-?[0-9]{1,40}(\\.[0-9]{0,40})?([eE][-+]?[0-9]{1,25})?",
        ) {
            if let Ok(amount) = parse_amount(&text) {
                // Whatever is accepted is the exact value: it formats back to
                // a number that parses to the same decimal.
                let again = parse_amount(&amount.as_decimal().to_string()).unwrap();
                prop_assert_eq!(again, amount);
            }
        }

        #[test]
        fn arbitrary_input_never_panics(text in any::<String>()) {
            let _ = parse_amount(&text);
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

        // Generated payloads write amounts exactly as well.
        let payload = EquipmentPayload {
            list_price: Some(amount),
            ..EquipmentPayload::default()
        };
        assert_eq!(
            serde_json::to_string(&payload).unwrap(),
            r#"{"list_price":0.1234567890123456789}"#
        );
    }
}
