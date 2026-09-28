/// Why a webhook signature was rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WebhookError {
    /// The `Stripe-Signature` header carried no `v1` digest.
    MissingSignature,
    /// The signed timestamp is older or newer than the tolerance.
    TimestampOutOfTolerance,
    /// The endpoint's signing secret was empty.
    EmptySecret,
    /// The body does not hash to the supplied digest.
    SignatureMismatch,
}
