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
}
