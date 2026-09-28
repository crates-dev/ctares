use super::*;

#[test]
fn a_default_config_uses_the_payment_element() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(config.get_kind(), ElementKind::Payment);
}

#[test]
fn a_config_returns_the_client_secret_it_was_given() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(config.get_client_secret(), "pi_1_secret_abc");
}

#[test]
fn with_kind_switches_the_element_variant() {
    let config: ElementConfig =
        ElementConfig::new(String::from("pi_1_secret_abc")).with_kind(ElementKind::Card);
    assert_eq!(config.get_kind(), ElementKind::Card);
}

#[test]
fn with_kind_setup_keeps_its_own_variant() {
    let config: ElementConfig =
        ElementConfig::new(String::from("seti_1_secret_abc")).with_kind(ElementKind::Setup);
    assert_eq!(config.get_kind(), ElementKind::Setup);
}

#[test]
fn an_unset_locale_falls_back_to_the_crate_default() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert_eq!(config.get_locale(), "");
    assert_eq!(config.effective_locale(), DEFAULT_LOCALE);
}

#[test]
fn an_explicit_locale_survives_the_fallback() {
    let config: ElementConfig =
        ElementConfig::new(String::from("pi_1_secret_abc")).with_locale(String::from("zh-CN"));
    assert_eq!(config.get_locale(), "zh-CN");
    assert_eq!(config.effective_locale(), "zh-CN");
}

#[test]
fn a_config_with_a_client_secret_is_mountable() {
    let config: ElementConfig = ElementConfig::new(String::from("pi_1_secret_abc"));
    assert!(config.is_mountable());
}

#[test]
fn a_config_without_a_client_secret_is_not_mountable() {
    let config: ElementConfig = ElementConfig::new(String::new());
    assert!(!config.is_mountable());
}

#[test]
fn a_whitespace_only_client_secret_is_not_mountable() {
    let config: ElementConfig = ElementConfig::new(String::from("   "));
    assert!(!config.is_mountable());
}
