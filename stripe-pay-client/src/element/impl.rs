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

    /// Return the client secret.
    ///
    /// # Returns
    ///
    /// - `&str` - the secret Stripe issued for the intent.
    pub fn get_client_secret(&self) -> &str {
        &self.client_secret
    }

    /// Return which element variant this configuration mounts.
    ///
    /// # Returns
    ///
    /// - `ElementKind` - the variant to render.
    pub fn get_kind(&self) -> ElementKind {
        self.kind
    }

    /// Return the requested locale.
    ///
    /// # Returns
    ///
    /// - `&str` - the locale, empty when the host page did not pick one.
    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    /// Switch to a different element variant.
    ///
    /// # Arguments
    ///
    /// - `ElementKind` - the variant to render instead.
    ///
    /// # Returns
    ///
    /// - `Self` - the configuration with the new variant.
    pub fn with_kind(mut self, kind: ElementKind) -> Self {
        self.set_kind(kind);
        self
    }

    /// Set the locale the element renders its labels in.
    ///
    /// # Arguments
    ///
    /// - `String` - the locale tag, such as `en` or `zh-CN`.
    ///
    /// # Returns
    ///
    /// - `Self` - the configuration with the new locale.
    pub fn with_locale(mut self, locale: String) -> Self {
        self.set_locale(locale);
        self
    }

    /// Switch the element variant in place.
    ///
    /// # Arguments
    ///
    /// - `ElementKind` - the variant to render.
    pub fn set_kind(&mut self, kind: ElementKind) {
        self.kind = kind;
    }

    /// Set the locale in place.
    ///
    /// # Arguments
    ///
    /// - `String` - the locale tag, such as `en` or `zh-CN`.
    pub fn set_locale(&mut self, locale: String) {
        self.locale = locale;
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
