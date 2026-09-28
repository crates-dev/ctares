use super::*;

#[test]
fn money_from_minor_keeps_the_wire_amount() {
    let amount: Money = Money::from_minor(109_900, Currency::Usd);
    assert_eq!(amount.get_amount(), 109_900);
    assert_eq!(amount.currency_code(), "USD");
}

#[test]
fn money_from_major_scales_two_decimal_currency() {
    let amount: Money = Money::from_major(1_099, Currency::Usd);
    assert_eq!(amount.get_amount(), 109_900);
}

#[test]
fn money_from_major_does_not_scale_zero_decimal_currency() {
    let amount: Money = Money::from_major(1_099, Currency::Jpy);
    assert_eq!(amount.get_amount(), 1_099);
}

#[test]
fn money_to_major_rescales_minor_units() {
    let amount: Money = Money::from_minor(200, Currency::Usd);
    assert_eq!(amount.to_major(), Decimal::new(200, 2));
}

#[test]
fn money_to_major_keeps_jpy_in_whole_yen() {
    let amount: Money = Money::from_minor(1_099, Currency::Jpy);
    assert_eq!(amount.to_major(), Decimal::new(1_099, 0));
}

#[test]
fn money_zero_is_zero_in_any_currency() {
    assert_eq!(Money::zero(Currency::Eur).get_amount(), 0);
    assert_eq!(Money::zero(Currency::Chf).get_amount(), 0);
}

#[test]
fn money_is_negative_detects_signed_amounts() {
    let negative: Money = Money::from_minor(-500, Currency::Usd);
    let positive: Money = Money::from_minor(500, Currency::Usd);
    assert!(negative.is_negative());
    assert!(!positive.is_negative());
}

#[test]
fn money_abs_removes_leading_minus() {
    let negative: Money = Money::from_minor(-500, Currency::Usd);
    assert_eq!(negative.abs().get_amount(), 500);
}

#[test]
fn money_abs_keeps_positive_amount_unchanged() {
    let positive: Money = Money::from_minor(500, Currency::Usd);
    assert_eq!(positive.abs().get_amount(), 500);
}

#[test]
fn money_checked_add_sums_same_currency() {
    let left: Money = Money::from_minor(200, Currency::Usd);
    let right: Money = Money::from_minor(300, Currency::Usd);
    let sum: Money = left.checked_add(right).unwrap();
    assert_eq!(sum.get_amount(), 500);
}

#[test]
fn money_checked_add_rejects_mismatched_currency() {
    let usd: Money = Money::from_minor(200, Currency::Usd);
    let eur: Money = Money::from_minor(200, Currency::Eur);
    let result: Result<Money, StripeParseError> = usd.checked_add(eur);
    assert!(matches!(
        result,
        Err(StripeParseError::CurrencyMismatch { .. })
    ));
}

#[test]
fn money_checked_add_reports_overflow() {
    let max: Money = Money::from_minor(i64::MAX, Currency::Usd);
    let one: Money = Money::from_minor(1, Currency::Usd);
    let result: Result<Money, StripeParseError> = max.checked_add(one);
    assert_eq!(result, Err(StripeParseError::AmountOverflow));
}

#[test]
fn money_with_currency_relabels_without_converting() {
    let usd: Money = Money::from_minor(200, Currency::Usd);
    let relabelled: Money = usd.with_currency(Currency::Jpy);
    assert_eq!(relabelled.get_amount(), 200);
    assert_eq!(relabelled.currency_code(), "JPY");
}

#[test]
fn money_display_renders_major_units_and_code() {
    let amount: Money = Money::from_minor(1_099, Currency::Usd);
    assert_eq!(format!("{}", amount), "10.99 USD");
}
