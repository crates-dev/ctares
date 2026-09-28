/// A webhook payload whose signature Stripe vouches for.
///
/// The struct keeps the three values verification actually needs, so a
/// handler never has to re-parse the header once this is built.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookEvent {
    /// The exact bytes Stripe signed.
    pub(super) payload: String,
    /// The timestamp Stripe signed, in seconds since the epoch.
    pub(super) timestamp: i64,
    /// The hex digest Stripe sent, without the `v1=` prefix.
    pub(super) signature: String,
}
