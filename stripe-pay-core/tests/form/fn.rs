use super::*;

#[test]
fn form_percent_encode_escapes_bracketed_metadata_key() {
    let encoded: String = percent_encode("metadata[order_id]");
    assert_eq!(encoded, "metadata%5Border_id%5D");
}

#[test]
fn form_percent_encode_escapes_reserved_characters() {
    let encoded: String = percent_encode("a=b&c");
    assert_eq!(encoded, "a%3Db%26c");
}

#[test]
fn form_percent_encode_maps_space_to_plus() {
    let encoded: String = percent_encode("order 42");
    assert_eq!(encoded, "order+42");
}

#[test]
fn form_percent_encode_keeps_unreserved_characters() {
    let encoded: String = percent_encode("order-id_42.v1~");
    assert_eq!(encoded, "order-id_42.v1~");
}

#[test]
fn form_percent_encode_uses_uppercase_hex_digits() {
    let encoded: String = percent_encode("/");
    assert_eq!(encoded, "%2F");
}

#[test]
fn form_metadata_key_wraps_the_bare_key_in_brackets() {
    let key: String = metadata_key("order_id");
    assert_eq!(key, "metadata[order_id]");
}

#[test]
fn form_expand_key_includes_the_list_index() {
    let first: String = expand_key(0);
    let second: String = expand_key(1);
    assert_eq!(first, "expand[0]");
    assert_eq!(second, "expand[1]");
}

#[test]
fn form_builder_encodes_sorted_key_value_pairs() {
    let body: String = FormParams::new()
        .with(String::from("currency"), String::from("USD"))
        .with(String::from("amount"), String::from("2000"))
        .encode();
    assert_eq!(body, "amount=2000&currency=USD");
}

#[test]
fn form_builder_replaces_a_duplicate_key() {
    let params: FormParams = FormParams::new()
        .with(String::from("amount"), String::from("2000"))
        .with(String::from("amount"), String::from("3000"));
    assert_eq!(params.len(), 1);
    assert_eq!(params.encode(), "amount=3000");
}

#[test]
fn form_builder_reports_field_count_and_emptiness() {
    let empty: FormParams = FormParams::new();
    assert!(empty.is_empty());
    let filled: FormParams = FormParams::new().with(String::from("a"), String::from("b"));
    assert_eq!(filled.len(), 1);
    assert!(!filled.is_empty());
}

#[test]
fn form_builder_adds_metadata_with_bracket_notation() {
    let params: FormParams = FormParams::new()
        .with_metadata(String::from("order_id"), String::from("42"))
        .unwrap();
    assert_eq!(params.encode(), "metadata%5Border_id%5D=42");
}

#[test]
fn form_builder_rejects_overlong_metadata_key() {
    let long_key: String = "k".repeat(41);
    let result: Result<FormParams, StripeError> =
        FormParams::new().with_metadata(long_key, String::from("v"));
    assert!(matches!(result, Err(StripeError::Malformed(_))));
}

#[test]
fn form_builder_rejects_empty_metadata_key() {
    let result: Result<FormParams, StripeError> =
        FormParams::new().with_metadata(String::new(), String::from("v"));
    assert!(matches!(result, Err(StripeError::Malformed(_))));
}

#[test]
fn form_builder_rejects_overlong_metadata_value() {
    let long_value: String = "v".repeat(501);
    let result: Result<FormParams, StripeError> =
        FormParams::new().with_metadata(String::from("k"), long_value);
    assert!(matches!(result, Err(StripeError::Malformed(_))));
}

#[test]
fn form_builder_accepts_metadata_value_at_the_documented_limit() {
    let long_value: String = "v".repeat(500);
    let params: FormParams = FormParams::new()
        .with_metadata(String::from("k"), long_value)
        .unwrap();
    assert_eq!(params.len(), 1);
}

#[test]
fn form_create_payment_intent_encodes_amount_and_currency() {
    let body: String = encode_create_payment_intent(Money::from_minor(2_000, Currency::Usd), None);
    assert_eq!(body, "amount=2000&currency=USD");
}

#[test]
fn form_create_payment_intent_includes_the_customer_when_given() {
    let body: String = encode_create_payment_intent(
        Money::from_minor(2_000, Currency::Usd),
        Some(String::from("cus_9Xyz456")),
    );
    assert_eq!(body, "amount=2000&currency=USD&customer=cus_9Xyz456");
}

#[test]
fn form_create_refund_omits_amount_for_a_full_refund() {
    let body: String = encode_create_refund("ch_1Def789", None, RefundReason::RequestedByCustomer);
    assert_eq!(body, "charge=ch_1Def789&reason=requested_by_customer");
}

#[test]
fn form_create_refund_includes_a_partial_amount() {
    let body: String = encode_create_refund(
        "ch_1Def789",
        Some(Money::from_minor(500, Currency::Usd)),
        RefundReason::Fraudulent,
    );
    assert_eq!(body, "amount=500&charge=ch_1Def789&reason=fraudulent");
}
