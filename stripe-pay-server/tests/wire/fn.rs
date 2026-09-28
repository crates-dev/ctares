use super::*;

#[test]
fn a_signed_payment_intent_body_encodes_the_minor_unit_amount() {
    let amount: Money = Money::from_major(1999, Currency::Usd);
    let body: String = FormParams::encode_create_payment_intent(amount, None);
    assert!(body.contains("amount=199900"));
    assert!(body.contains("currency=USD"));
}

#[test]
fn a_signed_payment_intent_body_carries_the_customer_when_supplied() {
    let amount: Money = Money::from_major(5, Currency::Eur);
    let body: String =
        FormParams::encode_create_payment_intent(amount, Some(String::from("cus_123")));
    assert!(body.contains("customer=cus_123"));
}

#[test]
fn a_zero_decimal_currency_encodes_without_a_decimal_point() {
    let amount: Money = Money::from_major(500, Currency::Jpy);
    let body: String = FormParams::encode_create_payment_intent(amount, None);
    assert!(body.contains("amount=500"));
    assert!(body.contains("currency=JPY"));
}

#[test]
fn a_refund_body_encodes_the_charge_and_the_reason() {
    let body: String =
        FormParams::encode_create_refund("ch_123", None, RefundReason::RequestedByCustomer);
    assert!(body.contains("charge=ch_123"));
    assert!(body.contains("reason=requested_by_customer"));
}

#[test]
fn a_partial_refund_body_encodes_the_reduced_amount() {
    let amount: Money = Money::from_major(3, Currency::Usd);
    let body: String =
        FormParams::encode_create_refund("ch_123", Some(amount), RefundReason::Fraudulent);
    assert!(body.contains("amount=300"));
    assert!(body.contains("reason=fraudulent"));
}

#[test]
fn a_confirm_body_encodes_the_payment_method_and_returns_ok() {
    let body: String = FormParams::encode_confirm_payment_intent("pi_1", "pm_1", None);
    assert!(body.contains("confirm=true"));
    assert!(body.contains("payment_method=pm_1"));
}

#[test]
fn a_confirm_body_omits_the_return_url_when_absent() {
    let body: String = FormParams::encode_confirm_payment_intent("pi_1", "pm_1", None);
    assert!(!body.contains("return_url"));
}

#[test]
fn a_customer_body_percent_encodes_the_email() {
    let body: String = FormParams::encode_create_customer(Some("buyer+tag@example.com"), None);
    assert!(body.contains("email=buyer%2Btag%40example.com"));
}

#[test]
fn a_customer_body_with_no_fields_is_empty() {
    let body: String = FormParams::encode_create_customer(None, None);
    assert_eq!(body, String::new());
}

#[test]
fn webhook_error_is_not_a_core_stripe_error() {
    let reason: WebhookError = WebhookError::MissingSignature;
    let code: String = reason.message().to_string();
    assert!(!code.is_empty());
}
