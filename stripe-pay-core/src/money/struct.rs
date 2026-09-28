use super::*;

/// A currency amount stored in Stripe's smallest denomination.
///
/// Stripe never transmits a floating point amount, so `Money` keeps an
/// integer minor-unit count plus the currency that determines how the
/// value converts back to a major-unit decimal. Two USD is `200`
/// minor units; two JPY is `2`, because JPY has no minor unit.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct Money {
    /// Amount in the currency's smallest denomination.
    pub(super) amount: i64,
    /// The currency the amount is denominated in.
    pub(super) currency: Currency,
}
