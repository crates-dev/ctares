use super::*;

/// Split a `Stripe-Signature` header into its timestamp and digest.
///
/// Stripe sends comma-separated entries such as
/// `t=1614556800,v1=deadbeef,v0=ignored`. The `v0` entry is the legacy
/// scheme and `v1` is the current one, so only `v1` is accepted; a
/// header carrying no `v1` entry is rejected rather than downgraded.
///
/// # Arguments
///
/// - `&str` - the raw `Stripe-Signature` header value.
///
/// # Returns
///
/// - `Result<WebhookEvent, WebhookError>` - the verification target, or
///   `MissingSignature` when no timestamp or `v1` digest was present.
pub fn parse_signature_header(header: &str, payload: &str) -> Result<WebhookEvent, WebhookError> {
    let mut timestamp: Option<i64> = None;
    let mut signature: Option<String> = None;
    for entry in header.split(TIMESTAMP_SEPARATOR) {
        let Some((key, value)) = entry.split_once(KEY_VALUE_SEPARATOR) else {
            continue;
        };
        match key {
            TIMESTAMP_FIELD => {
                timestamp = value.parse::<i64>().ok();
            }
            SIGNATURE_SCHEME => {
                signature = Some(String::from(value));
            }
            _ => {}
        }
    }
    let resolved_timestamp: i64 = match timestamp {
        Some(value) => value,
        None => return Err(WebhookError::MissingSignature),
    };
    let resolved_signature: String = match signature {
        Some(value) => value,
        None => return Err(WebhookError::MissingSignature),
    };
    Ok(WebhookEvent::new(
        String::from(payload),
        resolved_timestamp,
        resolved_signature,
    ))
}

/// Read the webhook signing secret from the process environment.
///
/// # Arguments
///
/// # Returns
///
/// - `Result<String, WebhookError>` - the configured secret, or
///   `EmptySecret` when the variable is unset or blank.
pub fn secret_from_env() -> Result<String, WebhookError> {
    match std::env::var(SECRET_KEY) {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(WebhookError::EmptySecret),
    }
}

/// Verify a raw webhook request end to end.
///
/// This is the entry point a hyperlane handler calls: it parses the
/// header, then checks freshness and the digest.
///
/// # Arguments
///
/// - `&str` - the raw `Stripe-Signature` header value.
/// - `&str` - the exact request body Stripe signed.
/// - `&str` - the endpoint's webhook signing secret.
/// - `i64` - the current time in seconds since the epoch.
/// - `i64` - the accepted drift, in seconds.
///
/// # Returns
///
/// - `Result<WebhookEvent, WebhookError>` - the verified event.
pub fn verify_webhook(
    header: &str,
    payload: &str,
    secret: &str,
    now: i64,
    tolerance: i64,
) -> Result<WebhookEvent, WebhookError> {
    let event: WebhookEvent = parse_signature_header(header, payload)?;
    event.verify(secret, now, tolerance)?;
    Ok(event)
}

/// Compare two byte slices without leaking their contents through timing.
///
/// A webhook digest is attacker-supplied, so an early-exit comparison
/// would leak how many leading bytes matched.
///
/// # Arguments
///
/// - `&[u8]` - the digest this crate computed.
/// - `&[u8]` - the digest the caller supplied.
///
/// # Returns
///
/// - `bool` - `true` when the two slices are equal.
pub fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference: u8 = 0;
    let mut index: usize = 0;
    while index < left.len() {
        difference |= left[index] ^ right[index];
        index += 1;
    }
    difference == 0
}
