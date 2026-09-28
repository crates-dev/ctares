use super::*;

/// The non-sensitive card details Stripe exposes on a PaymentMethod.
///
/// PCI DSS keeps full card numbers and verification codes inside
/// Stripe's own systems, so this type carries only the four fields a
/// receipt may display: the network brand, the last four digits, and
/// the expiry. There is deliberately no PAN and no CVC field to fill.
#[derive(Clone, Debug, Default, Eq, Getter, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CardDetails {
    /// The card network the card belongs to.
    #[get(type(copy))]
    pub(super) brand: CardBrand,
    /// The last four digits of the card number.
    #[get]
    pub(super) last4: String,
    /// The expiry month, 1 through 12.
    #[get(type(copy))]
    pub(super) exp_month: i64,
    /// The four-digit expiry year.
    #[get(type(copy))]
    pub(super) exp_year: i64,
}
