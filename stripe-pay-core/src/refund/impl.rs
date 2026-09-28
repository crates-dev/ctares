use super::*;

impl RefundReason {
    /// Return the wire string Stripe uses for this reason.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the `reason` value in a Stripe request.
    pub const fn as_str(&self) -> &'static str {
        match self {
            RefundReason::RequestedByCustomer => REASON_REQUESTED_BY_CUSTOMER,
            RefundReason::Duplicate => REASON_DUPLICATE,
            RefundReason::Fraudulent => REASON_FRAUDULENT,
            RefundReason::OrderCancellation => REASON_ORDER_CANCELLATION,
        }
    }

    /// Return whether the reason marks an involuntary return.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` for reasons the merchant did not choose.
    pub const fn is_involuntary(&self) -> bool {
        matches!(self, RefundReason::Duplicate | RefundReason::Fraudulent)
    }
}

impl Display for RefundReason {
    /// Render the wire string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the reason as Stripe spells it.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Refund {
    /// Build a Refund from its wire fields.
    ///
    /// # Arguments
    ///
    /// - `String` - Stripe's identifier for the refund.
    /// - `String` - the charge being refunded.
    /// - `Money` - the amount returned.
    /// - `RefundReason` - why the money was returned.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled refund.
    pub fn new(id: String, charge: String, amount: Money, reason: RefundReason) -> Self {
        Self {
            id,
            charge,
            amount: amount.get_amount(),
            currency: String::from(amount.currency_code()),
            reason,
            succeeded: true,
        }
    }

    /// Return the amount this refund returns.
    ///
    /// # Returns
    ///
    /// - `Result<Money, StripeParseError>` - the amount, or an error
    ///   when the response carried a currency this crate does not model.
    pub fn get_amount(&self) -> Result<Money, StripeParseError> {
        let currency: Currency = self.currency.parse()?;
        Ok(Money::from_minor(self.amount, currency))
    }

    /// Return whether Stripe has fully processed this refund.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` once the money is on its way back.
    pub fn is_succeeded(&self) -> bool {
        self.get_succeeded()
    }

    /// Return whether the merchant chose this reason themselves.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` for reasons the merchant did not pick.
    pub fn is_involuntary(&self) -> bool {
        self.get_reason().is_involuntary()
    }
}
