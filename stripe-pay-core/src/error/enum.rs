use super::*;

/// A failure raised while constructing or interpreting core types.
///
/// These are the errors a caller can provoke before any network call
/// happens: an unparsable currency, a mismatched pair of amounts, or
/// an identifier whose prefix matches no known Stripe resource.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum StripeParseError {
    /// The currency code is not one this crate models.
    #[error("unknown currency code: {0}")]
    UnknownCurrency(String),
    /// The identifier prefix matches no known Stripe resource family.
    #[error("unrecognised Stripe identifier: {0}")]
    UnrecognisedId(String),
    /// Two amounts in different currencies were combined.
    #[error("cannot combine {left} and {right}: Stripe never converts between currencies")]
    CurrencyMismatch {
        /// The currency of the left-hand amount.
        left: Currency,
        /// The currency of the right-hand amount.
        right: Currency,
    },
    /// Adding two amounts exceeded the `i64` range Stripe accepts.
    #[error("amount overflow: the sum exceeds the 64-bit range Stripe accepts")]
    AmountOverflow,
}

/// A failure Stripe reported, or a transport failure on the way there.
///
/// Stripe's REST errors carry a machine-readable `code` alongside the
/// human `message`. The `request_id` is what support asks for, so it
/// is kept as a distinct field rather than being folded into the
/// message.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum StripeError {
    /// Stripe answered with a structured error object.
    #[error("stripe error {code}: {message}")]
    Api {
        /// Stripe's machine-readable error code, such as
        /// `card_declined`.
        code: String,
        /// Stripe's human-readable explanation.
        message: String,
        /// The `request-id` header, which support needs for tracing.
        request_id: String,
    },
    /// The request never reached Stripe, or the answer never came back.
    #[error("stripe transport error: {0}")]
    Transport(String),
    /// A webhook signature did not verify.
    #[error("webhook signature verification failed: {0}")]
    SignatureVerification(String),
    /// A core type could not be built from the given input.
    #[error(transparent)]
    Parse(#[from] StripeParseError),
    /// The response body was not the JSON shape this crate expects.
    #[error("malformed stripe response: {0}")]
    Malformed(String),
}
