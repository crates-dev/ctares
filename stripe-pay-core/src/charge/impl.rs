use super::*;

impl ChargeStatus {
    /// Return the wire string Stripe uses for this status.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the `status` value in a Stripe response.
    pub const fn as_str(&self) -> &'static str {
        match self {
            ChargeStatus::Pending => CHARGE_STATUS_PENDING,
            ChargeStatus::Succeeded => CHARGE_STATUS_SUCCEEDED,
            ChargeStatus::Authorized => CHARGE_STATUS_AUTHORIZED,
            ChargeStatus::Failed => CHARGE_STATUS_FAILED,
        }
    }

    /// Return whether the charge moved money.
    ///
    /// An authorised charge has reserved funds but not taken them, so
    /// it is deliberately not counted as captured.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` only for a succeeded charge.
    pub const fn is_captured(&self) -> bool {
        matches!(self, ChargeStatus::Succeeded)
    }
}

impl Display for ChargeStatus {
    /// Render the wire string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the status as Stripe spells it.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl PaymentMethodKind {
    /// Return the wire string Stripe uses for this method kind.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the `type` value in a Stripe response.
    pub const fn as_str(&self) -> &'static str {
        match self {
            PaymentMethodKind::Card => METHOD_KIND_CARD,
            PaymentMethodKind::SepaDebit => METHOD_KIND_SEPA_DEBIT,
            PaymentMethodKind::BankTransfer => METHOD_KIND_BANK_TRANSFER,
            PaymentMethodKind::Wallet => METHOD_KIND_WALLET,
        }
    }
}

impl Charge {
    /// Build a Charge from its wire fields.
    ///
    /// # Arguments
    ///
    /// - `String` - Stripe's identifier for the charge.
    /// - `Money` - the amount captured.
    /// - `ChargeStatus` - whether the charge moved money.
    /// - `PaymentMethodKind` - how the customer paid.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled charge.
    pub fn new(
        id: String,
        amount: Money,
        status: ChargeStatus,
        payment_method_kind: PaymentMethodKind,
    ) -> Self {
        Self {
            id,
            amount: amount.get_amount(),
            currency: String::from(amount.currency_code()),
            status,
            payment_method_kind,
            disputed: false,
            payment_intent: None,
        }
    }

    /// Return Stripe's identifier for this charge.
    ///
    /// # Returns
    ///
    /// - `&str` - the Charge identifier.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Return the amount this charge captured.
    ///
    /// # Returns
    ///
    /// - `Result<Money, StripeParseError>` - the amount, or an error
    ///   when the response carried a currency this crate does not model.
    pub fn get_amount(&self) -> Result<Money, StripeParseError> {
        let currency: Currency = self.currency.parse()?;
        Ok(Money::from_minor(self.amount, currency))
    }

    /// Return whether the charge moved money.
    ///
    /// # Returns
    ///
    /// - `ChargeStatus` - the charge's settlement state.
    pub fn get_status(&self) -> ChargeStatus {
        self.status
    }

    /// Return how the customer paid.
    ///
    /// # Returns
    ///
    /// - `PaymentMethodKind` - the payment method family.
    pub fn get_payment_method_kind(&self) -> PaymentMethodKind {
        self.payment_method_kind
    }

    /// Return whether the charge is under dispute.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` once a customer has challenged the charge.
    pub fn is_disputed(&self) -> bool {
        self.get_disputed()
    }

    /// Return the raw dispute flag Stripe sent.
    ///
    /// # Returns
    ///
    /// - `bool` - the value of the `disputed` response field.
    pub fn get_disputed(&self) -> bool {
        self.disputed
    }

    /// Return the PaymentIntent that created this charge.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - the PaymentIntent identifier, or `None` when
    ///   the charge did not come from one.
    pub fn get_payment_intent(&self) -> Option<&str> {
        self.payment_intent.as_deref()
    }

    /// Return whether this charge actually moved the customer's money.
    ///
    /// An authorised charge has reserved funds but not taken them, so
    /// it is deliberately not counted as captured.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` only for a succeeded charge.
    pub fn is_captured(&self) -> bool {
        self.get_status().is_captured()
    }
}
