use super::*;

#[test]
fn picks_the_encoding_the_client_asked_for() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("gzip");
    assert_eq!(encoding, Some(String::from("gzip")));
}

#[test]
fn prefers_the_offered_order_over_the_order_of_the_client() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("br, gzip");
    assert_eq!(encoding, Some(String::from("gzip")));
}

#[test]
fn refuses_an_encoding_that_is_not_offered() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("deflate");
    assert!(encoding.is_none());
}

#[test]
fn refuses_an_encoding_the_client_refused() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("gzip;q=0");
    assert!(encoding.is_none());
}

#[test]
fn accepts_every_encoding_when_the_client_asked_for_every_one() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("*");
    assert_eq!(encoding, Some(String::from("gzip")));
}

#[test]
fn reads_the_encoding_in_the_case_the_client_wrote_it_in() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    let encoding: Option<String> = policy.pick("GZIP");
    assert_eq!(encoding, Some(String::from("gzip")));
}

#[test]
fn leaves_a_body_that_is_already_compressed_alone() {
    assert!(!CompressionPolicy::is_compressible("image/png"));
    assert!(!CompressionPolicy::is_compressible("application/zip"));
    assert!(!CompressionPolicy::is_compressible("text/event-stream"));
}

#[test]
fn compresses_a_body_that_is_worth_compressing() {
    assert!(CompressionPolicy::is_compressible("text/html"));
    assert!(CompressionPolicy::is_compressible("application/json"));
    assert!(CompressionPolicy::is_compressible(""));
}

#[test]
fn offers_the_gzip_and_the_brotli_encoding_by_default() {
    let policy: CompressionPolicy = CompressionPolicy::default();
    assert_eq!(policy.get_min_size(), 1024);
    assert_eq!(policy.get_encodings().len(), 2);
    assert_eq!(policy.get_encodings()[0], String::from("gzip"));
    assert_eq!(policy.get_encodings()[1], String::from("br"));
}
