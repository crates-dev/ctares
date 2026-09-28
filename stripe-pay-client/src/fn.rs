use super::*;

/// Render the options object Stripe.js expects for an element.
///
/// Stripe.js is called from JavaScript, so the crate passes a JSON
/// options object rather than typed arguments. Building it here keeps
/// the payload identical to what a hand-written mount would send and
/// lets the tests assert on it without a browser.
///
/// # Arguments
///
/// - `&ElementConfig` - the configuration to render.
///
/// # Returns
///
/// - `Result<String, ElementError>` - the JSON options, or
///   `MissingClientSecret` when the configuration is unusable.
pub fn build_element_options(config: &ElementConfig) -> Result<String, ElementError> {
    if !config.is_mountable() {
        return Err(ElementError::MissingClientSecret);
    }
    let options: String = format!(
        "{{{}:{}{}{}}}",
        CLIENT_SECRET_FIELD,
        String::from("\""),
        config.get_client_secret(),
        String::from("\""),
    );
    Ok(build_options_with_locale(&options, config))
}

/// Append the locale entry to a rendered options object.
///
/// # Arguments
///
/// - `&str` - the options object built so far.
/// - `&ElementConfig` - the configuration supplying the locale.
///
/// # Returns
///
/// - `String` - the options object with its locale entry.
fn build_options_with_locale(options: &str, config: &ElementConfig) -> String {
    let trimmed: &str = options.strip_suffix('}').unwrap_or(options);
    let rendered: String = format!(
        "{},{}:{}\"{}\"{}}}",
        trimmed,
        LOCALE_FIELD,
        String::from("\""),
        config.effective_locale(),
        String::from("\""),
    );
    rendered
}

/// Return whether the page has loaded the Stripe.js global.
///
/// The check is a pure string test against the global registry, which
/// keeps it callable from both wasm and the host target.
///
/// # Arguments
///
/// - `bool` - whether `window.Stripe` resolved when probed.
///
/// # Returns
///
/// - `Result<(), ElementError>` - `Ok(())` when Stripe.js is present.
pub fn require_stripe_js(available: bool) -> Result<(), ElementError> {
    if available {
        return Ok(());
    }
    Err(ElementError::StripeJsUnavailable)
}

/// Check every precondition a mount needs, in the order a caller hits them.
///
/// # Arguments
///
/// - `&ElementConfig` - the configuration to validate.
/// - `bool` - whether the page has a mount target.
/// - `bool` - whether Stripe.js is loaded.
///
/// # Returns
///
/// - `Result<(), ElementError>` - `Ok(())` when the element can mount.
pub fn preflight(
    config: &ElementConfig,
    has_mount_target: bool,
    stripe_js_loaded: bool,
) -> Result<(), ElementError> {
    if !config.is_mountable() {
        return Err(ElementError::MissingClientSecret);
    }
    if !has_mount_target {
        return Err(ElementError::MissingMountTarget);
    }
    require_stripe_js(stripe_js_loaded)
}
