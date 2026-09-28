use super::*;

impl PaymentIntentStatus {
    /// Return the wire string Stripe uses for this state.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the `status` value in a Stripe response.
    pub const fn as_str(&self) -> &'static str {
        match self {
            PaymentIntentStatus::RequiresPaymentMethod => STATUS_REQUIRES_PAYMENT_METHOD,
            PaymentIntentStatus::RequiresAction => STATUS_REQUIRES_ACTION,
            PaymentIntentStatus::RequiresConfirmation => STATUS_REQUIRES_CONFIRMATION,
            PaymentIntentStatus::Processing => STATUS_PROCESSING,
            PaymentIntentStatus::RequiresCapture => STATUS_REQUIRES_CAPTURE,
            PaymentIntentStatus::Succeeded => STATUS_SUCCEEDED,
            PaymentIntentStatus::Canceled => STATUS_CANCELED,
        }
    }

    /// Return whether no further state change is possible.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` for `Succeeded` and `Canceled`, the two states
    ///   Stripe never moves out of.
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled
        )
    }

    /// Return whether Stripe can move this state to `next`.
    ///
    /// The table mirrors the PaymentIntent lifecycle Stripe documents:
    /// a failed confirmation returns the intent to
    /// `RequiresPaymentMethod` so the customer can retry, a manual
    /// capture flow parks on `RequiresCapture`, and neither
    /// `Succeeded` nor `Canceled` ever moves again.
    ///
    /// # Arguments
    ///
    /// - `&PaymentIntentStatus` - the state Stripe would move into.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the transition is reachable.
    pub const fn can_transition_to(&self, next: &PaymentIntentStatus) -> bool {
        match self {
            PaymentIntentStatus::RequiresPaymentMethod => matches!(
                next,
                PaymentIntentStatus::RequiresConfirmation
                    | PaymentIntentStatus::RequiresAction
                    | PaymentIntentStatus::Processing
                    | PaymentIntentStatus::RequiresCapture
                    | PaymentIntentStatus::Succeeded
                    | PaymentIntentStatus::Canceled
            ),
            PaymentIntentStatus::RequiresConfirmation => matches!(
                next,
                PaymentIntentStatus::RequiresAction
                    | PaymentIntentStatus::Processing
                    | PaymentIntentStatus::RequiresCapture
                    | PaymentIntentStatus::RequiresPaymentMethod
                    | PaymentIntentStatus::Succeeded
                    | PaymentIntentStatus::Canceled
            ),
            PaymentIntentStatus::RequiresAction => matches!(
                next,
                PaymentIntentStatus::Processing
                    | PaymentIntentStatus::RequiresCapture
                    | PaymentIntentStatus::Succeeded
                    | PaymentIntentStatus::Canceled
            ),
            PaymentIntentStatus::Processing => matches!(
                next,
                PaymentIntentStatus::Succeeded
                    | PaymentIntentStatus::RequiresCapture
                    | PaymentIntentStatus::RequiresPaymentMethod
                    | PaymentIntentStatus::Canceled
            ),
            PaymentIntentStatus::RequiresCapture => matches!(
                next,
                PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled
            ),
            PaymentIntentStatus::Succeeded => false,
            PaymentIntentStatus::Canceled => false,
        }
    }
}

impl Display for PaymentIntentStatus {
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

impl PaymentIntentNextAction {
    /// Return whether the browser must handle a challenge.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the client owes the user a 3-D Secure or
    ///   bank-redirect flow before the payment can proceed.
    pub const fn requires_client_action(&self) -> bool {
        matches!(self, PaymentIntentNextAction::RequireAction)
    }
}

impl PaymentIntent {
    /// Build a PaymentIntent from its wire fields.
    ///
    /// # Arguments
    ///
    /// - `String` - Stripe's identifier for the intent.
    /// - `Money` - the amount to capture.
    /// - `PaymentIntentStatus` - the intent's lifecycle state.
    /// - `PaymentIntentNextAction` - what the customer must still do.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled intent.
    pub fn new(
        id: String,
        amount: Money,
        status: PaymentIntentStatus,
        next_action: PaymentIntentNextAction,
    ) -> Self {
        Self {
            id,
            amount: amount.get_amount(),
            currency: String::from(amount.currency_code()),
            status,
            next_action,
            latest_charge: None,
            customer: None,
        }
    }

    /// Return the amount this intent will capture.
    ///
    /// The minor-unit count is paired with the currency recorded in
    /// the response, so the two never drift apart. The `amount` and
    /// `currency` fields are not exposed separately for this reason.
    ///
    /// # Returns
    ///
    /// - `Result<Money, StripeParseError>` - the amount, or an error
    ///   when the response carried a currency this crate does not model.
    pub fn get_amount(&self) -> Result<Money, StripeParseError> {
        let currency: Currency = self.currency.parse()?;
        Ok(Money::from_minor(self.amount, currency))
    }

    /// Return whether the payment has been captured.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` once the intent reaches a terminal state that
    ///   means money moved.
    pub fn is_succeeded(&self) -> bool {
        matches!(self.get_status(), PaymentIntentStatus::Succeeded)
    }

    /// Return whether the browser must handle a bank challenge.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the client still owes the user a 3-D
    ///   Secure or redirect step.
    pub fn requires_action(&self) -> bool {
        self.get_next_action().requires_client_action()
    }
}
