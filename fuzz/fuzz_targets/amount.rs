//! Fuzz target for exact amount parsing.
//!
//! For every input: parsing never panics. For every JSON number with a
//! moderate exponent: the result matches an independent arbitrary-precision
//! reference (the value rounded half away from zero at the largest scale, at
//! most 28, whose mantissa fits 96 bits; rejected when none fits). Every
//! accepted amount survives a JSON round trip with value and scale.

#![no_main]

use bigdecimal::{BigDecimal, RoundingMode, Signed, ToPrimitive, num_bigint::BigInt};
use libfuzzer_sys::fuzz_target;
use rentman_client::RentmanAmount;
use rust_decimal::Decimal;
use serde_json::value::RawValue;

/// Larger exponents make the reference computation slow; such inputs only get
/// the no-panic and round-trip checks.
const MAX_REFERENCE_EXPONENT: i64 = 1000;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let parsed = serde_json::from_str::<RentmanAmount>(text);

    if let Ok(amount) = parsed {
        let json = serde_json::to_string(&amount).expect("an amount always serializes");
        let again: RentmanAmount =
            serde_json::from_str(&json).expect("a serialized amount parses again");
        assert_eq!(again, amount, "round trip changed the value of {text:?}");
        assert_eq!(
            again.as_decimal().scale(),
            amount.as_decimal().scale(),
            "round trip changed the scale of {text:?}"
        );
    }

    let Some(number) = json_number(text) else {
        return;
    };
    let reference = reference_parse(number);
    match (&parsed, reference) {
        (Ok(amount), Some(expected)) => {
            assert_eq!(amount.as_decimal(), expected, "wrong value for {number:?}");
        }
        (Err(_), None) => {}
        (parsed, reference) => {
            panic!("{number:?}: parser {parsed:?}, reference {reference:?}")
        }
    }
});

/// The raw text of `text` if it is exactly one JSON number with an exponent
/// small enough for the reference computation.
fn json_number(text: &str) -> Option<&str> {
    let raw: &RawValue = serde_json::from_str(text).ok()?;
    let number = raw.get();
    if !number.starts_with(|c: char| c == '-' || c.is_ascii_digit()) {
        return None;
    }
    let exponent = match number.split_once(['e', 'E']) {
        Some((_, exponent)) => exponent.parse::<i64>().ok()?,
        None => 0,
    };
    // A signed range: `abs()` would overflow for `i64::MIN`.
    (-MAX_REFERENCE_EXPONENT..=MAX_REFERENCE_EXPONENT)
        .contains(&exponent)
        .then_some(number)
}

fn reference_parse(number: &str) -> Option<Decimal> {
    let exact: BigDecimal = number.parse().expect("a JSON number is a BigDecimal");
    let max_mantissa = BigInt::from((1_u128 << 96) - 1);
    (0..=28_i64).rev().find_map(|scale| {
        let rounded = exact.with_scale_round(scale, RoundingMode::HalfUp);
        let (mantissa, _) = rounded.as_bigint_and_exponent();
        if mantissa.abs() > max_mantissa {
            return None;
        }
        Some(Decimal::from_i128_with_scale(
            mantissa.to_i128()?,
            scale as u32,
        ))
    })
}
