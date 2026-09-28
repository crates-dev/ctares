use super::*;

/// A PaymentIntent as returned by Stripe's REST API.
///
/// A PaymentIntent tracks one customer payment attempt from creation
/// through authentication to capture. The `next_action` field tells the
/// browser what it still owes the user, and `latest_charge` links to
/// the Charge Stripe created once the payment settled.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntent {
    /// Stripe's identifier for this intent.
    pub(super) id: String,
    /// The amount Stripe will capture, in minor units.
    pub(super) amount: i64,
    /// The ISO 4217 code the amount is denominated in.
    pub(super) currency: String,
    /// Where the intent sits in its lifecycle.
    pub(super) status: PaymentIntentStatus,
    /// What the customer must still do, if anything.
    pub(super) next_action: PaymentIntentNextAction,
    /// The charge created when the payment settled, when there is one.
    pub(super) latest_charge: Option<String>,
    /// The customer this intent charges, when one was supplied.
    pub(super) customer: Option<String>,
}
