use super::*;

impl StripeIdKind {
    /// Return the wire prefix Stripe uses for this resource family.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the two-character prefix, such as `pi` for a
    ///   PaymentIntent.
    pub const fn prefix(&self) -> &'static str {
        match self {
            StripeIdKind::CheckoutSession => "cs",
            StripeIdKind::Charge => "ch",
            StripeIdKind::Customer => "cus",
            StripeIdKind::PaymentIntent => "pi",
            StripeIdKind::PaymentMethod => "pm",
            StripeIdKind::Refund => "re",
            StripeIdKind::Event => "evt",
        }
    }

    /// Return the Stripe `object` name for this resource family.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the value Stripe puts in the `object` field
    ///   of a response body for this family.
    pub const fn object_name(&self) -> &'static str {
        match self {
            StripeIdKind::CheckoutSession => OBJECT_CHECKOUT_SESSION,
            StripeIdKind::Charge => OBJECT_CHARGE,
            StripeIdKind::Customer => OBJECT_CUSTOMER,
            StripeIdKind::PaymentIntent => OBJECT_PAYMENT_INTENT,
            StripeIdKind::PaymentMethod => OBJECT_PAYMENT_METHOD,
            StripeIdKind::Refund => OBJECT_REFUND,
            StripeIdKind::Event => OBJECT_EVENT,
        }
    }

    /// Return whether a wire value carries this family's prefix.
    ///
    /// A Stripe identifier always begins with its type prefix followed
    /// by an underscore, so this rejects a PaymentIntent identifier
    /// whose body happens to start with another family's letters.
    ///
    /// # Arguments
    ///
    /// - `&str` - the wire value to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value starts with this family's
    ///   prefix followed by an underscore.
    pub fn matches_prefix(&self, value: &str) -> bool {
        let prefix: &str = self.prefix();
        value
            .strip_prefix(prefix)
            .is_some_and(|rest: &str| rest.starts_with('_'))
    }
}

impl StripeId {
    /// Build an identifier from its resource family and wire value.
    ///
    /// This constructor trusts the caller. Use `parse` when the value
    /// comes from an untyped response body and the family must be
    /// derived from the prefix.
    ///
    /// # Arguments
    ///
    /// - `StripeIdKind` - the resource family the identifier addresses.
    /// - `String` - the full wire value, normally starting with the
    ///   family's prefix followed by an underscore.
    ///
    /// # Returns
    ///
    /// - `Self` - the typed identifier.
    pub const fn new(kind: StripeIdKind, raw: String) -> Self {
        Self { kind, raw }
    }

    /// Derive the resource family from a wire value's prefix.
    ///
    /// Stripe's family prefixes are mutually exclusive, so the first
    /// family whose prefix matches wins. A value carrying no known
    /// prefix is rejected rather than silently defaulting to some
    /// resource, because guessing here would let a charge identifier
    /// be passed to the refund endpoint.
    ///
    /// # Arguments
    ///
    /// - `&str` - the wire value, for example `pi_3Abc...`.
    ///
    /// # Returns
    ///
    /// - `Result<Self, StripeParseError>` - the typed identifier, or an
    ///   error when the value carries no known family prefix.
    pub fn parse(raw: &str) -> Result<Self, StripeParseError> {
        let families: [StripeIdKind; 7] = [
            StripeIdKind::CheckoutSession,
            StripeIdKind::Charge,
            StripeIdKind::Customer,
            StripeIdKind::PaymentIntent,
            StripeIdKind::PaymentMethod,
            StripeIdKind::Refund,
            StripeIdKind::Event,
        ];
        for family in families {
            if family.matches_prefix(raw) {
                return Ok(Self {
                    kind: family,
                    raw: String::from(raw),
                });
            }
        }
        Err(StripeParseError::UnrecognisedId(String::from(raw)))
    }

    /// Return the resource family this identifier addresses.
    ///
    /// # Returns
    ///
    /// - `StripeIdKind` - the identifier's resource family.
    pub fn get_kind(&self) -> StripeIdKind {
        self.kind
    }

    /// Return the full wire value including its type prefix.
    ///
    /// # Returns
    ///
    /// - `&str` - the identifier exactly as Stripe sent it.
    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    /// Return whether the wire value carries this identifier's prefix.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the stored value still matches its family.
    pub fn has_valid_prefix(&self) -> bool {
        self.get_kind().matches_prefix(self.get_raw())
    }
}

impl Display for StripeId {
    /// Render the full wire value.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the identifier's raw wire value.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get_raw())
    }
}
