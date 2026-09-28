/// Wire string for a Visa card.
pub const CARD_BRAND_VISA: &str = "visa";

/// Wire string for a Mastercard card.
pub const CARD_BRAND_MASTERCARD: &str = "mastercard";

/// Wire string for an American Express card.
pub const CARD_BRAND_AMEX: &str = "amex";

/// Wire string for a Discover card.
pub const CARD_BRAND_DISCOVER: &str = "discover";

/// Wire string for a JCB card.
pub const CARD_BRAND_JCB: &str = "jcb";

/// Wire string for a Diners Club card.
pub const CARD_BRAND_DINERS: &str = "diners";

/// Wire string for a UnionPay card.
pub const CARD_BRAND_UNIONPAY: &str = "unionpay";

/// Wire string for a brand outside the set this crate enumerates.
pub const CARD_BRAND_OTHER: &str = "other";

/// Number of trailing digits Stripe retains for a card.
pub const CARD_LAST4_CHARS: usize = 4;
