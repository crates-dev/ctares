use super::*;

const SECRET: &str = "whsec_test_secret";
const PAYLOAD: &str = "{\"id\":\"evt_1\",\"type\":\"payment_intent.succeeded\"}";

#[test]
fn signature_header_yields_the_signed_timestamp_and_digest() {
    let event: WebhookEvent =
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::from("abc123"));
    let header: String = "t=1614556800,v1=abc123".to_string();
    let parsed: WebhookEvent = match parse_signature_header(&header, PAYLOAD) {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason}"),
    };
    assert_eq!(parsed.get_timestamp(), event.get_timestamp());
    assert_eq!(parsed.get_signature(), event.get_signature());
    assert_eq!(parsed.get_payload(), event.get_payload());
}

#[test]
fn signature_header_ignores_the_legacy_v0_scheme() {
    let header: String = String::from("t=1614556800,v0=stale,v1=current");
    let parsed: WebhookEvent = match parse_signature_header(&header, PAYLOAD) {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason}"),
    };
    assert_eq!(parsed.get_signature(), "current");
}

#[test]
fn signature_header_without_a_v1_digest_is_rejected() {
    let header: String = String::from("t=1614556800,v0=stale");
    let outcome: Result<WebhookEvent, WebhookError> = parse_signature_header(&header, PAYLOAD);
    assert_eq!(outcome, Err(WebhookError::MissingSignature));
}

#[test]
fn signature_header_without_a_timestamp_is_rejected() {
    let header: String = String::from("v1=abc123");
    let outcome: Result<WebhookEvent, WebhookError> = parse_signature_header(&header, PAYLOAD);
    assert_eq!(outcome, Err(WebhookError::MissingSignature));
}

#[test]
fn signature_header_rejects_a_non_numeric_timestamp() {
    let header: String = String::from("t=not-a-number,v1=abc123");
    let outcome: Result<WebhookEvent, WebhookError> = parse_signature_header(&header, PAYLOAD);
    assert_eq!(outcome, Err(WebhookError::MissingSignature));
}

#[test]
fn compute_signature_is_stable_for_the_same_input() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    let first: String = event.compute_signature(SECRET).unwrap_or_default();
    let second: String = event.compute_signature(SECRET).unwrap_or_default();
    assert_eq!(first, second);
    assert_eq!(first.len(), DIGEST_HEX_CHARS);
}

#[test]
fn compute_signature_rejects_an_empty_secret() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    let outcome: Result<String, WebhookError> = event.compute_signature("");
    assert_eq!(outcome, Err(WebhookError::EmptySecret));
}

#[test]
fn compute_signature_changes_when_the_secret_changes() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    let mine: String = event.compute_signature(SECRET).unwrap_or_default();
    let theirs: String = event.compute_signature("whsec_other").unwrap_or_default();
    assert_ne!(mine, theirs);
}

#[test]
fn verify_signature_accepts_the_digest_stripe_sent() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    assert_eq!(event.verify_signature(SECRET), Ok(()));
}

#[test]
fn verify_signature_rejects_a_tampered_payload() {
    let good: String = WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::from(""))
        .compute_signature(SECRET)
        .unwrap_or_default();
    let tampered: WebhookEvent =
        WebhookEvent::new(String::from("{\"id\":\"evt_2\"}"), 1614556800, good);
    assert_eq!(
        tampered.verify_signature(SECRET),
        Err(WebhookError::SignatureMismatch)
    );
}

#[test]
fn verify_signature_rejects_a_digest_of_the_wrong_length() {
    let short: WebhookEvent =
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::from("abc"));
    assert_eq!(
        short.verify_signature(SECRET),
        Err(WebhookError::SignatureMismatch)
    );
}

#[test]
fn a_timestamp_inside_the_tolerance_is_fresh() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    assert!(event.is_timestamp_fresh(1614556800, DEFAULT_TOLERANCE_SECONDS));
    assert!(event.is_timestamp_fresh(1614556800 + 299, DEFAULT_TOLERANCE_SECONDS));
}

#[test]
fn a_timestamp_outside_the_tolerance_is_stale() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    assert!(!event.is_timestamp_fresh(1614556800 + 301, DEFAULT_TOLERANCE_SECONDS));
}

#[test]
fn a_future_timestamp_beyond_the_tolerance_is_stale() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    assert!(!event.is_timestamp_fresh(1614556800 - 301, DEFAULT_TOLERANCE_SECONDS));
}

#[test]
fn verify_rejects_a_stale_timestamp_before_the_digest() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    let outcome: Result<(), WebhookError> = event.verify(
        SECRET,
        1614556800 + DEFAULT_TOLERANCE_SECONDS + 1,
        DEFAULT_TOLERANCE_SECONDS,
    );
    assert_eq!(outcome, Err(WebhookError::TimestampOutOfTolerance));
}

#[test]
fn verify_accepts_a_fresh_genuine_event() {
    let event: WebhookEvent = WebhookEvent::new(
        String::from(PAYLOAD),
        1614556800,
        WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
            .compute_signature(SECRET)
            .unwrap_or_default(),
    );
    assert_eq!(
        event.verify(SECRET, 1614556800 + 10, DEFAULT_TOLERANCE_SECONDS),
        Ok(())
    );
}

#[test]
fn webhook_error_messages_are_stable() {
    assert_eq!(
        WebhookError::MissingSignature.message(),
        MESSAGE_NO_SIGNATURE
    );
    assert_eq!(
        WebhookError::SignatureMismatch.message(),
        MESSAGE_SIGNATURE_MISMATCH
    );
    assert_eq!(WebhookError::EmptySecret.to_string(), MESSAGE_EMPTY_SECRET);
}

#[test]
fn verify_webhook_accepts_a_genuine_request() {
    let digest: String = WebhookEvent::new(String::from(PAYLOAD), 1614556800, String::new())
        .compute_signature(SECRET)
        .unwrap_or_default();
    let header: String = format!("t=1614556800,v1={digest}");
    let outcome: Result<WebhookEvent, WebhookError> = verify_webhook(
        &header,
        PAYLOAD,
        SECRET,
        1614556800,
        DEFAULT_TOLERANCE_SECONDS,
    );
    assert!(outcome.is_ok());
}

#[test]
fn verify_webhook_rejects_a_forged_request() {
    let header: String = String::from("t=1614556800,v1=deadbeef");
    let outcome: Result<WebhookEvent, WebhookError> = verify_webhook(
        &header,
        PAYLOAD,
        SECRET,
        1614556800,
        DEFAULT_TOLERANCE_SECONDS,
    );
    assert_eq!(outcome, Err(WebhookError::SignatureMismatch));
}
