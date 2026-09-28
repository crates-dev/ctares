/// The resource family a `StripeId` addresses.
///
/// Stripe encodes the resource family into the identifier prefix, so
/// `pi_3Abc...` is a PaymentIntent and `cus_9Xyz...` is a Customer.
/// Keeping the family alongside the raw value lets the compiler reject
/// passing a Charge identifier where a PaymentIntent is required.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub enum StripeIdKind {
    /// A Checkout Session.
    CheckoutSession,
    /// A Charge created by a PaymentIntent.
    Charge,
    /// A Customer.
    Customer,
    /// A PaymentIntent.
    PaymentIntent,
    /// A PaymentMethod.
    PaymentMethod,
    /// A Refund.
    Refund,
    /// A webhook Event.
    Event,
}
