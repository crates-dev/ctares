use super::*;

/// A Charge as returned by Stripe's REST API.
///
/// A charge is the moment money actually moves or is reserved. It is
/// created by a PaymentIntent rather than requested directly, so its
/// identifier is what a refund references.
#[derive(Clone, Debug, Eq, Getter, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Charge {
    /// Stripe's identifier for this charge.
    pub(super) id: String,
    /// The amount captured, in minor units.
    #[get(skip)]
    pub(super) amount: i64,
    /// The ISO 4217 code the amount is denominated in.
    #[get(skip)]
    pub(super) currency: String,
    /// Whether the charge moved money.
    #[get(type(copy))]
    pub(super) status: ChargeStatus,
    /// How the customer paid.
    #[get(type(copy))]
    pub(super) payment_method_kind: PaymentMethodKind,
    /// Whether Stripe considers the charge under dispute.
    #[get(type(copy))]
    pub(super) disputed: bool,
    /// The PaymentIntent that created this charge.
    #[get(type(clone))]
    pub(super) payment_intent: Option<String>,
}
