/// The lifecycle state of a PaymentIntent.
///
/// Stripe advances a PaymentIntent through these states as the card
/// is authenticated. Only the `Succeeded` and `Cancel` states are
/// terminal.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    /// Stripe has created the intent and is waiting for a payment method.
    #[default]
    RequiresPaymentMethod,
    /// A payment method is attached and awaiting customer action.
    RequiresAction,
    /// The customer must confirm the payment on their own device.
    RequiresConfirmation,
    /// The payment is processing and the result is not yet final.
    Processing,
    /// Separate auth-and-capture flows park here until captured.
    RequiresCapture,
    /// The payment completed and the funds are captured.
    Succeeded,
    /// The payment can no longer succeed without a new attempt.
    Canceled,
}

/// What the customer must do to move a PaymentIntent forward.
///
/// The client reads this to decide whether to show a card form, open a
/// 3-D Secure challenge, or treat the payment as finished.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextAction {
    /// The customer must supply a payment method; show the card form.
    #[default]
    ShowCardForm,
    /// The bank requires a 3-D Secure challenge in the browser.
    RequireAction,
    /// The payment cleared and nothing else is required.
    None,
}
