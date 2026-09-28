/// Why Stripe moved a charge's money back to the customer.
///
/// The reason is set when the refund is created and never changes
/// afterwards, so it stays an enum rather than a free string.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    /// The customer asked for the money back.
    #[default]
    RequestedByCustomer,
    /// Stripe could not deliver the product or service.
    Duplicate,
    /// The merchant recorded the charge in error.
    Fraudulent,
    /// An order was cancelled before the product shipped.
    OrderCancellation,
}
