/// Whether the charge actually took the customer's money.
///
/// Stripe distinguishes a charge that captured funds from one that was
/// authorised but not captured, and from one that failed outright.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    /// Stripe has the customer's details but has not taken money yet.
    #[default]
    Pending,
    /// The charge succeeded and the funds are captured.
    Succeeded,
    /// The charge was authorised but not captured.
    Authorized,
    /// The charge failed and no money moved.
    Failed,
}

/// How the customer paid.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodKind {
    /// A payment card.
    #[default]
    Card,
    /// A bank debit mandate.
    SepaDebit,
    /// A bank transfer, such as ACH.
    BankTransfer,
    /// A wallet such as Apple Pay or Google Pay.
    Wallet,
}
