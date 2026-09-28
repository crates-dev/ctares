use super::*;

#[test]
fn confirm_payment_intent_encodes_the_three_required_fields() {
    let body: String = encode_confirm_payment_intent("pi_3Abc", "pm_3Xyz", None);
    assert!(body.contains("confirm=true"));
    assert!(body.contains("payment_intent=pi_3Abc"));
    assert!(body.contains("payment_method=pm_3Xyz"));
}

#[test]
fn confirm_payment_intent_includes_the_return_url_when_supplied() {
    let body: String =
        encode_confirm_payment_intent("pi_3Abc", "pm_3Xyz", Some("https://example.com/done"));
    assert!(body.contains("return_url=https%3A%2F%2Fexample.com%2Fdone"));
}

#[test]
fn create_customer_encodes_email_and_description() {
    let body: String = encode_create_customer(Some("buyer@example.com"), Some("checkout buyer"));
    assert!(body.contains("email=buyer%40example.com"));
    assert!(body.contains("description=checkout+buyer"));
}

#[test]
fn create_customer_omits_absent_optional_fields() {
    let body: String = encode_create_customer(None, None);
    assert_eq!(body, String::new());
}
