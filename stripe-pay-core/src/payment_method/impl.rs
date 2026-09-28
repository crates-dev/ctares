use super::*;

impl CardBrand {
    /// Return the wire string Stripe uses for this brand.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the brand value in a Stripe response.
    pub const fn as_str(&self) -> &'static str {
        match self {
            CardBrand::Visa => CARD_BRAND_VISA,
            CardBrand::Mastercard => CARD_BRAND_MASTERCARD,
            CardBrand::Amex => CARD_BRAND_AMEX,
            CardBrand::Discover => CARD_BRAND_DISCOVER,
            CardBrand::Jcb => CARD_BRAND_JCB,
            CardBrand::Diners => CARD_BRAND_DINERS,
            CardBrand::Unionpay => CARD_BRAND_UNIONPAY,
            CardBrand::Other => CARD_BRAND_OTHER,
        }
    }
}

impl Display for CardBrand {
    /// Render the wire string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` writing the brand as Stripe spells it.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl FromStr for CardBrand {
    /// The type returned when a brand cannot be parsed.
    type Err = StripeParseError;

    /// Parse a Stripe brand string.
    ///
    /// # Arguments
    ///
    /// - `&str` - the brand string from a Stripe response.
    ///
    /// # Returns
    ///
    /// - `Result<Self, StripeParseError>` - the matching brand, or
    ///   `Other` for a brand this crate does not enumerate.
    fn from_str(text: &str) -> Result<Self, StripeParseError> {
        match text {
            CARD_BRAND_VISA => Ok(CardBrand::Visa),
            CARD_BRAND_MASTERCARD => Ok(CardBrand::Mastercard),
            CARD_BRAND_AMEX => Ok(CardBrand::Amex),
            CARD_BRAND_DISCOVER => Ok(CardBrand::Discover),
            CARD_BRAND_JCB => Ok(CardBrand::Jcb),
            CARD_BRAND_DINERS => Ok(CardBrand::Diners),
            CARD_BRAND_UNIONPAY => Ok(CardBrand::Unionpay),
            _ => Ok(CardBrand::Other),
        }
    }
}

impl CardDetails {
    /// Build card details from their displayable fields.
    ///
    /// # Arguments
    ///
    /// - `CardBrand` - the card network.
    /// - `String` - the last four digits.
    /// - `i64` - the expiry month.
    /// - `i64` - the expiry year.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled card details.
    pub const fn new(brand: CardBrand, last4: String, exp_month: i64, exp_year: i64) -> Self {
        Self {
            brand,
            last4,
            exp_month,
            exp_year,
        }
    }

    /// Return whether the last field holds exactly four digits.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value is four characters long.
    pub fn has_four_digit_tail(&self) -> bool {
        self.get_last4().chars().count() == CARD_LAST4_CHARS
    }
}
