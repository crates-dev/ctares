use super::*;

#[test]
fn allows_every_origin_by_default() {
    let policy: CorsPolicy = CorsPolicy::default();
    let allowed: Option<String> = policy.allowed_origin("https://example.com");
    assert_eq!(allowed, Some(String::from("*")));
}

#[test]
fn answers_with_the_origin_that_was_asked_for() {
    let policy: CorsPolicy = CorsPolicy {
        allowed_origins: vec![String::from("https://example.com")],
        ..CorsPolicy::default()
    };
    let allowed: Option<String> = policy.allowed_origin("https://example.com");
    assert_eq!(allowed, Some(String::from("https://example.com")));
}

#[test]
fn refuses_an_origin_that_is_not_allowed() {
    let policy: CorsPolicy = CorsPolicy {
        allowed_origins: vec![String::from("https://example.com")],
        ..CorsPolicy::default()
    };
    let allowed: Option<String> = policy.allowed_origin("https://other.example.com");
    assert!(allowed.is_none());
}

#[test]
fn writes_the_cross_origin_headers_of_an_allowed_origin() {
    let policy: CorsPolicy = CorsPolicy::default();
    let headers: Option<Vec<(&'static str, String)>> = policy.headers("https://example.com");
    let headers: Vec<(&'static str, String)> = headers.unwrap();
    assert!(
        headers
            .iter()
            .any(
                |header: &(&'static str, String)| header.0 == "access-control-allow-origin"
                    && header.1 == "*"
            )
    );
    assert!(
        headers
            .iter()
            .any(
                |header: &(&'static str, String)| header.0 == "access-control-allow-methods"
                    && header.1 == "GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, TRACE"
            )
    );
    assert!(
        headers
            .iter()
            .any(
                |header: &(&'static str, String)| header.0 == "access-control-allow-headers"
                    && header.1 == "*"
            )
    );
    assert!(
        headers
            .iter()
            .any(|header: &(&'static str, String)| header.0 == "vary" && header.1 == "origin")
    );
}

#[test]
fn writes_no_credentials_by_default() {
    let policy: CorsPolicy = CorsPolicy::default();
    assert!(policy.max_age.is_none());
    assert!(!policy.allow_credentials);
    let headers: Option<Vec<(&'static str, String)>> = policy.headers("https://example.com");
    let headers: Vec<(&'static str, String)> = headers.unwrap();
    assert!(
        !headers
            .iter()
            .any(|header: &(&'static str, String)| header.0 == "access-control-allow-credentials")
    );
    assert!(
        !headers
            .iter()
            .any(|header: &(&'static str, String)| header.0 == "access-control-max-age")
    );
}

#[test]
fn writes_the_credentials_and_the_max_age_when_they_are_allowed() {
    let policy: CorsPolicy = CorsPolicy {
        max_age: Some(600),
        allow_credentials: true,
        ..CorsPolicy::default()
    };
    let headers: Option<Vec<(&'static str, String)>> = policy.headers("https://example.com");
    let headers: Vec<(&'static str, String)> = headers.unwrap();
    assert!(
        headers
            .iter()
            .any(
                |header: &(&'static str, String)| header.0 == "access-control-allow-credentials"
                    && header.1 == "true"
            )
    );
    assert!(
        headers.iter().any(
            |header: &(&'static str, String)| header.0 == "access-control-max-age"
                && header.1 == "600"
        )
    );
}

#[test]
fn writes_no_headers_for_an_origin_that_is_not_allowed() {
    let policy: CorsPolicy = CorsPolicy {
        allowed_origins: vec![String::from("https://example.com")],
        ..CorsPolicy::default()
    };
    let headers: Option<Vec<(&'static str, String)>> = policy.headers("https://other.example.com");
    assert!(headers.is_none());
}
