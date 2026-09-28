use super::*;

#[test]
fn payment_intent_status_wire_string_matches_stripe() {
    let status: PaymentIntentStatus = PaymentIntentStatus::RequiresPaymentMethod;
    assert_eq!(status.as_str(), "requires_payment_method");
    let succeeded: PaymentIntentStatus = PaymentIntentStatus::Succeeded;
    assert_eq!(succeeded.as_str(), "succeeded");
}

#[test]
fn payment_intent_only_success_and_cancel_are_terminal() {
    assert!(PaymentIntentStatus::Succeeded.is_terminal());
    assert!(PaymentIntentStatus::Canceled.is_terminal());
    assert!(!PaymentIntentStatus::Processing.is_terminal());
    assert!(!PaymentIntentStatus::RequiresAction.is_terminal());
}

#[test]
fn payment_intent_next_action_flags_a_bank_challenge() {
    assert!(PaymentIntentNextAction::RequireAction.requires_client_action());
    assert!(!PaymentIntentNextAction::ShowCardForm.requires_client_action());
    assert!(!PaymentIntentNextAction::None.requires_client_action());
}

#[test]
fn payment_intent_round_trips_its_amount_and_currency() {
    let intent: PaymentIntent = PaymentIntent::new(
        String::from("pi_3Abc123"),
        Money::from_minor(2_000, Currency::Usd),
        PaymentIntentStatus::Succeeded,
        PaymentIntentNextAction::None,
    );
    let amount: Money = intent.get_amount().unwrap();
    assert_eq!(amount.get_amount(), 2_000);
    assert_eq!(amount.currency_code(), "USD");
    assert_eq!(intent.get_id(), "pi_3Abc123");
}

#[test]
fn payment_intent_reports_success_only_when_succeeded() {
    let succeeded: PaymentIntent = PaymentIntent::new(
        String::from("pi_1"),
        Money::from_minor(1, Currency::Usd),
        PaymentIntentStatus::Succeeded,
        PaymentIntentNextAction::None,
    );
    let processing: PaymentIntent = PaymentIntent::new(
        String::from("pi_2"),
        Money::from_minor(1, Currency::Usd),
        PaymentIntentStatus::Processing,
        PaymentIntentNextAction::None,
    );
    assert!(succeeded.is_succeeded());
    assert!(!processing.is_succeeded());
}

#[test]
fn payment_intent_requires_action_only_for_a_challenge() {
    let challenged: PaymentIntent = PaymentIntent::new(
        String::from("pi_1"),
        Money::from_minor(1, Currency::Usd),
        PaymentIntentStatus::RequiresAction,
        PaymentIntentNextAction::RequireAction,
    );
    assert!(challenged.requires_action());
}

#[test]
fn payment_intent_deserializes_a_stripe_response_body() {
    let body: &str = r#"{
        "id": "pi_3Abc123",
        "amount": 2000,
        "currency": "USD",
        "status": "succeeded",
        "next_action": "none",
        "latest_charge": "ch_1Def789",
        "customer": "cus_9Xyz456"
    }"#;
    let intent: PaymentIntent = serde_json::from_str(body).unwrap();
    assert_eq!(intent.get_id(), "pi_3Abc123");
    assert_eq!(intent.get_status(), PaymentIntentStatus::Succeeded);
    assert_eq!(intent.get_latest_charge().as_deref(), Some("ch_1Def789"));
    assert_eq!(intent.get_customer().as_deref(), Some("cus_9Xyz456"));
    assert_eq!(intent.get_amount().unwrap().get_amount(), 2_000);
}

#[test]
fn charge_status_wire_string_matches_stripe() {
    let status: ChargeStatus = ChargeStatus::Succeeded;
    assert_eq!(status.as_str(), "succeeded");
    let authorized: ChargeStatus = ChargeStatus::Authorized;
    assert_eq!(authorized.as_str(), "authorized");
}

#[test]
fn charge_only_succeeded_status_is_captured() {
    assert!(ChargeStatus::Succeeded.is_captured());
    assert!(!ChargeStatus::Authorized.is_captured());
    assert!(!ChargeStatus::Pending.is_captured());
    assert!(!ChargeStatus::Failed.is_captured());
}

#[test]
fn charge_round_trips_its_amount_and_method_kind() {
    let charge: Charge = Charge::new(
        String::from("ch_1Def789"),
        Money::from_minor(2_000, Currency::Usd),
        ChargeStatus::Succeeded,
        PaymentMethodKind::Card,
    );
    assert_eq!(charge.get_id(), "ch_1Def789");
    assert_eq!(charge.get_amount().unwrap().get_amount(), 2_000);
    assert_eq!(charge.get_payment_method_kind(), PaymentMethodKind::Card);
    assert!(charge.is_captured());
    assert!(!charge.is_disputed());
}

#[test]
fn charge_deserializes_a_stripe_response_body() {
    let body: &str = r#"{
        "id": "ch_1Def789",
        "amount": 2000,
        "currency": "USD",
        "status": "succeeded",
        "payment_method_kind": "card",
        "disputed": true,
        "payment_intent": "pi_3Abc123"
    }"#;
    let charge: Charge = serde_json::from_str(body).unwrap();
    assert_eq!(charge.get_status(), ChargeStatus::Succeeded);
    assert!(charge.is_disputed());
    assert_eq!(charge.get_payment_intent().as_deref(), Some("pi_3Abc123"));
}

#[test]
fn refund_reason_wire_string_matches_stripe() {
    let reason: RefundReason = RefundReason::RequestedByCustomer;
    assert_eq!(reason.as_str(), "requested_by_customer");
    let duplicate: RefundReason = RefundReason::Duplicate;
    assert_eq!(duplicate.as_str(), "duplicate");
}

#[test]
fn refund_marks_duplicate_and_fraudulent_as_involuntary() {
    assert!(RefundReason::Duplicate.is_involuntary());
    assert!(RefundReason::Fraudulent.is_involuntary());
    assert!(!RefundReason::RequestedByCustomer.is_involuntary());
    assert!(!RefundReason::OrderCancellation.is_involuntary());
}

#[test]
fn refund_round_trips_its_charge_and_amount() {
    let refund: Refund = Refund::new(
        String::from("re_2Ghi012"),
        String::from("ch_1Def789"),
        Money::from_minor(500, Currency::Usd),
        RefundReason::Fraudulent,
    );
    assert_eq!(refund.get_id(), "re_2Ghi012");
    assert_eq!(refund.get_charge(), "ch_1Def789");
    assert_eq!(refund.get_amount().unwrap().get_amount(), 500);
    assert!(refund.is_succeeded());
    assert!(refund.is_involuntary());
}

#[test]
fn refund_deserializes_a_stripe_response_body() {
    let body: &str = r#"{
        "id": "re_2Ghi012",
        "charge": "ch_1Def789",
        "amount": 500,
        "currency": "USD",
        "reason": "duplicate",
        "succeeded": false
    }"#;
    let refund: Refund = serde_json::from_str(body).unwrap();
    assert_eq!(refund.get_reason(), RefundReason::Duplicate);
    assert!(!refund.is_succeeded());
}
