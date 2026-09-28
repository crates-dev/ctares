use super::*;

impl Currency {
    /// Return the ISO 4217 alphabetic code Stripe expects.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the three-letter uppercase currency code.
    pub const fn code(&self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Gbp => "GBP",
            Currency::Jpy => "JPY",
            Currency::Chf => "CHF",
        }
    }

    /// Return the number of decimal places this currency stores.
    ///
    /// Stripe's zero-decimal currencies such as JPY report amounts in
    /// the major denomination rather than in minor units, so the
    /// exponent is the single source of truth for the conversion.
    ///
    /// # Returns
    ///
    /// - `u32` - the exponent Stripe uses when serializing amounts.
    pub const fn exponent(&self) -> u32 {
        match self {
            Currency::Usd | Currency::Eur | Currency::Gbp | Currency::Chf => {
                DEFAULT_CURRENCY_EXPONENT
            }
            Currency::Jpy => ZERO_DECIMAL_EXPONENT,
        }
    }

    /// Return the divisor converting a major amount to minor units.
    ///
    /// # Returns
    ///
    /// - `u32` - `100` for two-decimal currencies and `1` for JPY.
    pub const fn minor_units_divisor(&self) -> u32 {
        match self {
            Currency::Usd | Currency::Eur | Currency::Gbp | Currency::Chf => MINOR_UNITS_DIVISOR,
            Currency::Jpy => 1,
        }
    }
}

impl Display for Currency {
    /// Render the ISO 4217 code.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the uppercase currency code.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.code())
    }
}

impl FromStr for Currency {
    type Err = StripeParseError;

    /// Parse an ISO 4217 code, case-insensitively.
    ///
    /// # Arguments
    ///
    /// - `&str` - the currency code to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - the parsed currency, or the
    ///   reason the code was not recognised.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_ascii_uppercase().as_str() {
            "USD" => Ok(Currency::Usd),
            "EUR" => Ok(Currency::Eur),
            "GBP" => Ok(Currency::Gbp),
            "JPY" => Ok(Currency::Jpy),
            "CHF" => Ok(Currency::Chf),
            other => Err(StripeParseError::UnknownCurrency(String::from(other))),
        }
    }
}

impl Money {
    /// Build a zero amount in the given currency.
    ///
    /// # Arguments
    ///
    /// - `Currency` - the currency of the zero amount.
    ///
    /// # Returns
    ///
    /// - `Self` - a zero-valued amount.
    pub const fn zero(currency: Currency) -> Self {
        Self {
            amount: 0,
            currency,
        }
    }

    /// Build an amount directly from a minor-unit value.
    ///
    /// This is the constructor that matches the wire format: Stripe
    /// sends and accepts `amount` in the smallest denomination.
    ///
    /// # Arguments
    ///
    /// - `i64` - the amount in the currency's smallest denomination.
    /// - `Currency` - the currency the amount is denominated in.
    ///
    /// # Returns
    ///
    /// - `Self` - the amount, unchanged.
    pub const fn from_minor(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Build an amount from a major-unit value.
    ///
    /// `1_099` USD becomes `109_900` minor units; `1_099` JPY stays
    /// `1_099` because JPY has no minor unit.
    ///
    /// # Arguments
    ///
    /// - `i64` - the amount in major units.
    /// - `Currency` - the currency the amount is denominated in.
    ///
    /// # Returns
    ///
    /// - `Self` - the amount converted into minor units.
    pub const fn from_major(amount: i64, currency: Currency) -> Self {
        let scale: i64 = currency.minor_units_divisor() as i64;
        Self {
            amount: amount * scale,
            currency,
        }
    }

    /// Return the ISO 4217 code Stripe expects for this amount.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the three-letter uppercase currency code.
    pub fn currency_code(&self) -> &'static str {
        self.get_currency().code()
    }

    /// Return the amount converted back to major units.
    ///
    /// # Returns
    ///
    /// - `Decimal` - the minor-unit count rescaled by the currency's
    ///   exponent, so `200` USD reads back as `2.00`.
    pub fn to_major(&self) -> Decimal {
        Decimal::new(self.get_amount(), self.get_currency().exponent())
    }

    /// Return the same minor-unit count labelled with another currency.
    ///
    /// Stripe never converts between currencies; this only rewrites the
    /// label and exists for presentation code, not for sending a charge.
    ///
    /// # Arguments
    ///
    /// - `Currency` - the currency to relabel the amount with.
    ///
    /// # Returns
    ///
    /// - `Self` - the same minor-unit count under a new currency.
    pub fn with_currency(&self, currency: Currency) -> Self {
        Self {
            amount: self.get_amount(),
            currency,
        }
    }

    /// Return whether the amount is negative, which Stripe rejects.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the amount is below zero.
    pub fn is_negative(&self) -> bool {
        self.get_amount() < 0
    }

    /// Return the amount with any leading minus sign removed.
    ///
    /// # Returns
    ///
    /// - `Self` - the absolute value of the amount.
    pub fn abs(&self) -> Self {
        Self {
            amount: self.get_amount().abs(),
            currency: self.get_currency(),
        }
    }

    /// Return this amount plus another of the same currency.
    ///
    /// # Arguments
    ///
    /// - `Self` - the amount to add.
    ///
    /// # Returns
    ///
    /// - `Result<Self, StripeParseError>` - the sum, or an error when
    ///   the currencies differ or the addition overflows `i64`.
    pub fn checked_add(&self, other: Self) -> Result<Self, StripeParseError> {
        if self.get_currency() != other.get_currency() {
            return Err(StripeParseError::CurrencyMismatch {
                left: self.get_currency(),
                right: other.get_currency(),
            });
        }
        let sum: i64 = self
            .get_amount()
            .checked_add(other.get_amount())
            .ok_or(StripeParseError::AmountOverflow)?;
        Ok(Self {
            amount: sum,
            currency: self.get_currency(),
        })
    }
}

impl Display for Money {
    /// Render the amount as a decimal in major units.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the decimal amount and its currency code.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.to_major(), self.currency_code())
    }
}
