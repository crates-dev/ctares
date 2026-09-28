use super::*;

/// A typed Stripe object identifier such as `pi_3Abc...` or `cus_9Xyz...`.
///
/// The `kind` records which resource the identifier points at, and
/// `raw` keeps the original wire value untouched so it round-trips
/// back to Stripe byte-for-byte.
#[derive(Clone, Debug, Eq, Getter, Hash, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct StripeId {
    /// The resource family this identifier addresses.
    #[get(type(copy))]
    pub(super) kind: StripeIdKind,
    /// The full wire value including its type prefix.
    pub(super) raw: String,
}
