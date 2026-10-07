/// Header carrying the signed timestamp and digest.
pub const HEADER_SIGNATURE: &str = "stripe-signature";

/// Number of hex characters in one SHA-256 digest.
pub const DIGEST_HEX_CHARS: usize = 64;

/// Drift Stripe recommends tolerating, in seconds.
pub const DEFAULT_TOLERANCE_SECONDS: i64 = 300;

/// Diagnostic when the signature header carries no digest at all.
pub const MESSAGE_NO_SIGNATURE: &str = "webhook signature header contains no digest";

/// Diagnostic when the signed body and the digest disagree.
pub const MESSAGE_SIGNATURE_MISMATCH: &str = "webhook signature does not match the payload";

/// Diagnostic when the configured signing secret is empty.
pub const MESSAGE_EMPTY_SECRET: &str = "webhook signing secret is empty";

/// Signature scheme Stripe prefixes to every webhook digest.
pub(crate) const SIGNATURE_SCHEME: &str = "v1";

/// Environment variable holding the endpoint's signing secret.
pub(crate) const SECRET_KEY: &str = "STRIPE_WEBHOOK_SECRET";

/// Header key whose value is the signed timestamp.
pub(crate) const TIMESTAMP_FIELD: &str = "t";

/// Separator between the timestamp and each digest in the header.
pub(crate) const TIMESTAMP_SEPARATOR: char = ',';

/// Separator between a header key and its value.
pub(crate) const KEY_VALUE_SEPARATOR: char = '=';

/// Diagnostic when the signed timestamp falls outside the tolerance.
pub(crate) const MESSAGE_TIMESTAMP_OUT_OF_TOLERANCE: &str =
    "webhook timestamp is outside the accepted tolerance";
