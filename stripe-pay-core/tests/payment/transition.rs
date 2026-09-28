use super::*;

#[test]
fn payment_intent_requires_capture_uses_the_stripe_wire_string() {
    let status: PaymentIntentStatus = PaymentIntentStatus::RequiresCapture;
    assert_eq!(status.as_str(), "requires_capture");
}

#[test]
fn payment_intent_can_move_from_requires_payment_method_to_requires_confirmation() {
    let from: PaymentIntentStatus = PaymentIntentStatus::RequiresPaymentMethod;
    let to: PaymentIntentStatus = PaymentIntentStatus::RequiresConfirmation;
    assert!(from.can_transition_to(&to));
}

#[test]
fn payment_intent_can_move_from_requires_action_to_processing() {
    let from: PaymentIntentStatus = PaymentIntentStatus::RequiresAction;
    let to: PaymentIntentStatus = PaymentIntentStatus::Processing;
    assert!(from.can_transition_to(&to));
}

#[test]
fn payment_intent_cannot_leave_a_terminal_state() {
    let succeeded: PaymentIntentStatus = PaymentIntentStatus::Succeeded;
    let canceled: PaymentIntentStatus = PaymentIntentStatus::Canceled;
    let processing: PaymentIntentStatus = PaymentIntentStatus::Processing;
    assert!(!succeeded.can_transition_to(&processing));
    assert!(!canceled.can_transition_to(&processing));
}

#[test]
fn payment_intent_confirm_failure_returns_to_requires_payment_method() {
    let from: PaymentIntentStatus = PaymentIntentStatus::RequiresConfirmation;
    let to: PaymentIntentStatus = PaymentIntentStatus::RequiresPaymentMethod;
    assert!(from.can_transition_to(&to));
}

#[test]
fn payment_intent_can_move_from_requires_capture_to_succeeded() {
    let from: PaymentIntentStatus = PaymentIntentStatus::RequiresCapture;
    let to: PaymentIntentStatus = PaymentIntentStatus::Succeeded;
    assert!(from.can_transition_to(&to));
}

#[test]
fn payment_intent_action_never_returns_to_requires_action_from_processing() {
    let from: PaymentIntentStatus = PaymentIntentStatus::Processing;
    let to: PaymentIntentStatus = PaymentIntentStatus::RequiresAction;
    assert!(!from.can_transition_to(&to));
}

#[test]
fn card_brand_round_trips_its_wire_string() {
    let brand: CardBrand = CardBrand::Mastercard;
    assert_eq!(brand.as_str(), "mastercard");
    assert_eq!(brand.to_string(), "mastercard");
}

#[test]
fn card_brand_falls_back_to_other_for_an_unlisted_network() {
    let parsed: Result<CardBrand, StripeParseError> = "maestro".parse();
    assert_eq!(parsed.unwrap_or(CardBrand::Other), CardBrand::Other);
}

#[test]
fn card_details_expose_only_the_four_safe_fields() {
    let card: CardDetails = CardDetails::new(CardBrand::Visa, String::from("4242"), 12, 2030);
    assert_eq!(card.get_brand(), CardBrand::Visa);
    assert_eq!(card.get_last4(), "4242");
    assert_eq!(card.get_exp_month(), 12);
    assert_eq!(card.get_exp_year(), 2030);
    assert!(card.has_four_digit_tail());
}

#[test]
fn card_details_reject_a_tail_that_is_not_four_digits() {
    let card: CardDetails =
        CardDetails::new(CardBrand::Visa, String::from("4242424242424242"), 12, 2030);
    assert!(!card.has_four_digit_tail());
}
