use super::*;

/// The configuration a checkout page hands to the payment element.
///
/// The struct is transport-free on purpose: the browser binds it to
/// Stripe.js at mount time, so every field can be exercised without a
/// DOM and stays unit-testable on the host target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementConfig {
    /// The client secret Stripe issued for the intent.
    pub(super) client_secret: String,
    /// Which element variant to render.
    pub(super) kind: ElementKind,
    /// The locale for the element's built-in labels.
    pub(super) locale: String,
}
