use super::*;

#[test]
fn element_options_carry_the_client_secret() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    let options: String = match config.build_options() {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason:?}"),
    };
    assert!(options.contains("clientSecret"));
    assert!(options.contains("pi_1_secret_abc"));
}

#[test]
fn element_options_carry_the_effective_locale() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    let options: String = match config.build_options() {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason:?}"),
    };
    assert!(options.contains("locale"));
    assert!(options.contains(DEFAULT_LOCALE));
}

#[test]
fn element_options_honour_an_explicit_locale() {
    let config: ElementConfig =
        ElementConfig::new(String::from("pi_1_secret_abc")).with_locale(String::from("zh-CN"));
    let options: String = match config.build_options() {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason:?}"),
    };
    assert!(options.contains("zh-CN"));
}

#[test]
fn element_options_are_braced_json() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    let options: String = match config.build_options() {
        Ok(value) => value,
        Err(reason) => panic!("unexpected rejection: {reason:?}"),
    };
    assert!(options.starts_with('{'));
    assert!(options.ends_with('}'));
}

#[test]
fn element_options_are_rejected_without_a_client_secret() {
    let config: ElementConfig = ElementConfig::new(String::new());
    let outcome: Result<String, ElementError> = config.build_options();
    assert_eq!(outcome, Err(ElementError::MissingClientSecret));
}

#[test]
fn require_stripe_js_accepts_a_loaded_page() {
    assert_eq!(ElementConfig::require_stripe_js(true), Ok(()));
}

#[test]
fn require_stripe_js_rejects_a_page_without_the_script() {
    assert_eq!(
        ElementConfig::require_stripe_js(false),
        Err(ElementError::StripeJsUnavailable)
    );
}

#[test]
fn preflight_accepts_a_fully_prepared_page() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(config.preflight(true, true), Ok(()));
}

#[test]
fn preflight_reports_the_missing_secret_first() {
    let config: ElementConfig = ElementConfig::new(String::new());
    assert_eq!(
        config.preflight(false, false),
        Err(ElementError::MissingClientSecret)
    );
}

#[test]
fn preflight_reports_a_missing_mount_target_next() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(
        config.preflight(false, true),
        Err(ElementError::MissingMountTarget)
    );
}

#[test]
fn preflight_reports_a_missing_script_last() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(
        config.preflight(true, false),
        Err(ElementError::StripeJsUnavailable)
    );
}
