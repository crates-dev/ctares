use super::*;

#[test]
fn identifier_parse_detects_payment_intent() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("pi_3Abc123");
    assert_eq!(parsed.unwrap().get_kind(), StripeIdKind::PaymentIntent);
}

#[test]
fn identifier_parse_detects_customer() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("cus_9Xyz456");
    assert_eq!(parsed.unwrap().get_kind(), StripeIdKind::Customer);
}

#[test]
fn identifier_parse_detects_charge() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("ch_1Def789");
    assert_eq!(parsed.unwrap().get_kind(), StripeIdKind::Charge);
}

#[test]
fn identifier_parse_detects_refund() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("re_2Ghi012");
    assert_eq!(parsed.unwrap().get_kind(), StripeIdKind::Refund);
}

#[test]
fn identifier_parse_rejects_unknown_prefix() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("zz_1Nope");
    assert!(matches!(parsed, Err(StripeParseError::UnrecognisedId(_))));
}

#[test]
fn identifier_parse_rejects_bare_prefix_without_underscore() {
    let parsed: Result<StripeId, StripeParseError> = StripeId::parse("pi");
    assert!(matches!(parsed, Err(StripeParseError::UnrecognisedId(_))));
}

#[test]
fn identifier_preserves_the_original_wire_value() {
    let parsed: StripeId = StripeId::parse("pi_3Abc123").unwrap();
    assert_eq!(parsed.get_raw(), "pi_3Abc123");
}

#[test]
fn identifier_display_renders_the_wire_value() {
    let parsed: StripeId = StripeId::parse("cus_9Xyz456").unwrap();
    assert_eq!(format!("{}", parsed), "cus_9Xyz456");
}

#[test]
fn identifier_kind_object_name_matches_stripe_resource() {
    assert_eq!(StripeIdKind::PaymentIntent.object_name(), "payment_intent");
    assert_eq!(
        StripeIdKind::CheckoutSession.object_name(),
        "checkout.session"
    );
    assert_eq!(StripeIdKind::Event.object_name(), "event");
}

#[test]
fn identifier_kind_matches_prefix_requires_underscore() {
    assert!(StripeIdKind::PaymentIntent.matches_prefix("pi_3Abc"));
    assert!(!StripeIdKind::PaymentIntent.matches_prefix("pi3Abc"));
}

#[test]
fn identifier_new_trusts_the_caller_and_stores_the_kind() {
    let mismatched: StripeId = StripeId::new(StripeIdKind::Charge, String::from("pi_wrong"));
    assert_eq!(mismatched.get_kind(), StripeIdKind::Charge);
    assert!(!mismatched.has_valid_prefix());
    let matching: StripeId = StripeId::new(StripeIdKind::Charge, String::from("ch_1Def"));
    assert!(matching.has_valid_prefix());
}

#[test]
fn identifier_parsed_value_always_has_a_matching_prefix() {
    let parsed: StripeId = StripeId::parse("re_2Ghi012").unwrap();
    assert!(parsed.has_valid_prefix());
}
