/// Why the client refused to mount a payment element.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElementError {
    /// The configuration carries no client secret.
    MissingClientSecret,
    /// Stripe.js was not present on the page.
    StripeJsUnavailable,
    /// The host page has no element to mount into.
    MissingMountTarget,
}
