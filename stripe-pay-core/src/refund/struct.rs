use super::*;

/// A Refund as returned by Stripe's REST API.
///
/// A refund returns some or all of a charge's money. `amount` is the
/// part being returned by this refund alone, so summing the refunds
/// attached to one charge reconstructs the total refunded.
#[derive(Clone, Debug, Eq, Getter, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Refund {
    /// Stripe's identifier for this refund.
    pub(super) id: String,
    /// The charge this refund belongs to.
    pub(super) charge: String,
    /// The amount returned, in minor units.
    #[get(skip)]
    pub(super) amount: i64,
    /// The ISO 4217 code the amount is denominated in.
    #[get(skip)]
    pub(super) currency: String,
    /// Why the money was returned.
    #[get(type(copy))]
    pub(super) reason: RefundReason,
    /// Whether Stripe has fully processed the refund.
    #[get(type(copy))]
    pub(super) succeeded: bool,
}
