/// The card network a non-sensitive card belongs to.
///
/// Only the display brand is modelled. Stripe's tokenisation means a
/// server never sees a full card number, and this crate deliberately
/// models no PAN or CVC field so it cannot leak one (PCI DSS).
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CardBrand {
    /// A Visa card.
    #[default]
    Visa,
    /// A Mastercard card.
    Mastercard,
    /// An American Express card.
    Amex,
    /// A Discover card.
    Discover,
    /// A JCB card.
    Jcb,
    /// A Diners Club card.
    Diners,
    /// A UnionPay card.
    Unionpay,
    /// A brand outside the set this crate enumerates.
    Other,
}
