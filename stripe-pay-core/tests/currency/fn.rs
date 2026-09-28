use super::*;

#[test]
fn currency_code_returns_uppercase_iso_code() {
    let usd: Currency = Currency::Usd;
    let jpy: Currency = Currency::Jpy;
    assert_eq!(usd.code(), "USD");
    assert_eq!(jpy.code(), "JPY");
}

#[test]
fn currency_exponent_distinguishes_zero_decimal_currencies() {
    let usd: Currency = Currency::Usd;
    let jpy: Currency = Currency::Jpy;
    assert_eq!(usd.exponent(), 2);
    assert_eq!(jpy.exponent(), 0);
}

#[test]
fn currency_minor_units_divisor_is_one_for_jpy() {
    let usd: Currency = Currency::Usd;
    let jpy: Currency = Currency::Jpy;
    assert_eq!(usd.minor_units_divisor(), 100);
    assert_eq!(jpy.minor_units_divisor(), 1);
}

#[test]
fn currency_parses_iso_code_case_insensitively() {
    let upper: Result<Currency, StripeParseError> = "USD".parse();
    let lower: Result<Currency, StripeParseError> = "usd".parse();
    let mixed: Result<Currency, StripeParseError> = "uSd".parse();
    assert_eq!(upper.unwrap(), Currency::Usd);
    assert_eq!(lower.unwrap(), Currency::Usd);
    assert_eq!(mixed.unwrap(), Currency::Usd);
}

#[test]
fn currency_rejects_unknown_code() {
    let parsed: Result<Currency, StripeParseError> = "XYZ".parse();
    assert!(matches!(parsed, Err(StripeParseError::UnknownCurrency(_))));
}

#[test]
fn currency_display_renders_iso_code() {
    let gbp: Currency = Currency::Gbp;
    assert_eq!(format!("{}", gbp), "GBP");
}
