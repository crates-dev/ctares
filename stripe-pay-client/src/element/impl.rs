use super::*;

impl ElementConfig {
    /// Build a configuration for a client secret.
    ///
    /// # Arguments
    ///
    /// - `String` - the client secret Stripe issued for the intent.
    ///
    /// # Returns
    ///
    /// - `Self` - the configuration, defaulting to the payment variant.
    pub fn new(client_secret: String) -> Self {
        Self {
            client_secret,
            kind: ElementKind::Payment,
            locale: String::new(),
        }
    }

    /// Return a copy of this configuration rendering the given variant.
    ///
    /// # Arguments
    ///
    /// - `ElementKind` - the variant the page should mount.
    ///
    /// # Returns
    ///
    /// - `Self` - the configuration with the variant switched.
    pub fn with_kind(mut self, kind: ElementKind) -> Self {
        self.set_kind(kind);
        self
    }

    /// Return a copy of this configuration with an explicit locale.
    ///
    /// # Arguments
    ///
    /// - `String` - the locale tag, such as `en` or `zh-CN`.
    ///
    /// # Returns
    ///
    /// - `Self` - the configuration with the locale set.
    pub fn with_locale(mut self, locale: String) -> Self {
        self.set_locale(locale);
        self
    }

    /// Return the locale actually sent to Stripe.js.
    ///
    /// An empty locale means the host page did not choose one, so the
    /// element falls back to the crate default rather than sending an
    /// empty string Stripe would reject.
    ///
    /// # Returns
    ///
    /// - `&str` - the effective locale.
    pub fn effective_locale(&self) -> &str {
        let requested: &str = self.get_locale();
        if requested.is_empty() {
            DEFAULT_LOCALE
        } else {
            requested
        }
    }

    /// Return whether the configuration can mount an element.
    ///
    /// Stripe rejects an empty client secret, so a blank one has to
    /// fail before the element is ever created.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a client secret is present.
    pub fn is_mountable(&self) -> bool {
        !self.get_client_secret().trim().is_empty()
    }

    /// Render the options object Stripe.js expects for an element.
    ///
    /// Stripe.js is called from JavaScript, so the crate passes a JSON
    /// options object rather than typed arguments. Building it here
    /// keeps the payload identical to what a hand-written mount would
    /// send and lets the tests assert on it without a browser.
    ///
    /// # Returns
    ///
    /// - `Result<String, ElementError>` - the JSON options, or
    ///   `MissingClientSecret` when the configuration is unusable.
    pub fn build_options(&self) -> Result<String, ElementError> {
        if !self.is_mountable() {
            return Err(ElementError::MissingClientSecret);
        }
        let trimmed: String = format!(
            "{{{}:{}{}{}}}",
            CLIENT_SECRET_FIELD,
            String::from("\""),
            self.get_client_secret(),
            String::from("\""),
        );
        Ok(format!(
            "{},{}:{}\"{}\"{}}}",
            trimmed.strip_suffix('}').unwrap_or(trimmed.as_str()),
            LOCALE_FIELD,
            String::from("\""),
            self.effective_locale(),
            String::from("\""),
        ))
    }

    /// Return whether the page has loaded the Stripe.js global.
    ///
    /// The check is a pure boolean test against the global registry,
    /// which keeps it callable from both wasm and the host target.
    ///
    /// # Arguments
    ///
    /// - `bool` - whether `window.Stripe` resolved when probed.
    ///
    /// # Returns
    ///
    /// - `Result<(), ElementError>` - `Ok(())` when Stripe.js is
    ///   present, `StripeJsUnavailable` otherwise.
    pub fn require_stripe_js(available: bool) -> Result<(), ElementError> {
        if available {
            return Ok(());
        }
        Err(ElementError::StripeJsUnavailable)
    }

    /// Check every precondition a mount needs, in the order a caller
    /// hits them.
    ///
    /// # Arguments
    ///
    /// - `bool` - whether the page has a mount target.
    /// - `bool` - whether Stripe.js is loaded.
    ///
    /// # Returns
    ///
    /// - `Result<(), ElementError>` - `Ok(())` when the element can
    ///   mount, or the first unmet precondition.
    pub fn preflight(
        &self,
        has_mount_target: bool,
        stripe_js_loaded: bool,
    ) -> Result<(), ElementError> {
        if !self.is_mountable() {
            return Err(ElementError::MissingClientSecret);
        }
        if !has_mount_target {
            return Err(ElementError::MissingMountTarget);
        }
        Self::require_stripe_js(stripe_js_loaded)
    }
}
