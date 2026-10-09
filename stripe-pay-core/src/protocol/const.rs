/// Stripe API version this crate is written against.
///
/// The value is sent as the `Stripe-Version` request header and is
/// echoed back in every webhook payload.
pub const STRIPE_API_VERSION: &str = "2026-08-26.dahlia";

/// Base URL of the Stripe REST API.
pub const STRIPE_API_BASE_URL: &str = "https://api.stripe.com";

/// Stripe's `type` discriminator for a successful charge.
pub const CHARGE_TYPE_CHARGE: &str = "charge";

/// Literal prefix shared by Stripe's legacy object identifiers.
pub const ID_PREFIX: &str = "id_";

/// Stripe's `object` discriminator shared by every Stripe object.
pub const OBJECT_DISCRIMINATOR: &str = "object";

/// Default tolerance, in seconds, when validating a webhook timestamp.
pub const DEFAULT_WEBHOOK_TOLERANCE_SECONDS: i64 = 300;

/// Stripe error code for a declined card, which is a settled answer.
pub const STRIPE_ERROR_CARD_DECLINED: &str = "card_declined";

/// Form parameter carrying a non-sensitive card's last four digits.
pub const FORM_CARD_LAST4: &str = "card[last4]";

/// Form parameter carrying a non-sensitive card's expiry month.
pub const FORM_CARD_EXP_MONTH: &str = "card[exp_month]";

/// Form parameter carrying a non-sensitive card's expiry year.
pub const FORM_CARD_EXP_YEAR: &str = "card[exp_year]";

/// Divisor converting a major-unit amount into minor units.
///
/// Two-decimal currencies such as USD use 100 minor units per major
/// unit; zero-decimal currencies such as JPY use 1.
pub(crate) const MINOR_UNITS_DIVISOR: u32 = 100;

/// Number of decimal places a two-decimal currency stores.
pub(crate) const DEFAULT_CURRENCY_EXPONENT: u32 = 2;

/// Number of decimal places a zero-decimal currency stores.
pub(crate) const ZERO_DECIMAL_EXPONENT: u32 = 0;

/// Stripe error code for a request rejected by the rate limiter.
pub(crate) const STRIPE_ERROR_RATE_LIMIT: &str = "rate_limit";

/// Stripe error code for a request that never reached the API.
pub(crate) const STRIPE_ERROR_API_CONNECTION: &str = "api_connection_error";

/// Stripe error code for a lock the account is still holding.
pub(crate) const STRIPE_ERROR_LOCK_TIMEOUT: &str = "lock_timeout";

/// Separator between two `key=value` pairs in an encoded form body.
pub(crate) const FORM_PAIR_SEPARATOR: char = '&';

/// Separator between a form key and its value.
pub(crate) const FORM_KEY_VALUE_SEPARATOR: char = '=';

/// Prefix of the bracketed key carrying Stripe metadata entries.
pub(crate) const METADATA_PREFIX: &str = "metadata";

/// Prefix of the bracketed key carrying Stripe `expand` entries.
pub(crate) const EXPAND_PREFIX: &str = "expand";

/// Open bracket used in Stripe's bracketed parameter names.
pub(crate) const FORM_OPEN_BRACKET: char = '[';

/// Close bracket used in Stripe's bracketed parameter names.
pub(crate) const FORM_CLOSE_BRACKET: char = ']';

/// Form parameter carrying a PaymentIntent or Refund amount.
pub(crate) const FORM_AMOUNT: &str = "amount";

/// Form parameter carrying an ISO 4217 currency code.
pub(crate) const FORM_CURRENCY: &str = "currency";

/// Form parameter carrying a Stripe customer identifier.
pub(crate) const FORM_CUSTOMER: &str = "customer";

/// Form parameter carrying a Stripe charge identifier.
pub(crate) const FORM_CHARGE: &str = "charge";

/// Form parameter carrying a refund's reason.
pub(crate) const FORM_REASON: &str = "reason";

/// Form parameter naming the PaymentIntent being confirmed.
pub(crate) const FORM_PAYMENT_INTENT: &str = "payment_intent";

/// Form parameter naming the payment method used to confirm.
pub(crate) const FORM_PAYMENT_METHOD: &str = "payment_method";

/// Form parameter requesting immediate confirmation.
pub(crate) const FORM_CONFIRM: &str = "confirm";

/// Form parameter carrying the browser return URL after 3-D Secure.
pub(crate) const FORM_RETURN_URL: &str = "return_url";

/// Form parameter carrying a customer's email address.
pub(crate) const FORM_EMAIL: &str = "email";

/// Form parameter carrying a customer's internal description.
pub(crate) const FORM_DESCRIPTION: &str = "description";

/// Wire value Stripe reads as boolean true in a form body.
pub(crate) const TRUE_LITERAL: &str = "true";

/// Longest metadata key Stripe accepts.
pub(crate) const METADATA_KEY_MAX_CHARS: usize = 40;

/// Longest metadata value Stripe accepts.
pub(crate) const METADATA_VALUE_MAX_CHARS: usize = 500;

/// Diagnostic for a metadata key outside Stripe's documented limit.
pub(crate) const METADATA_KEY_LIMIT_MESSAGE: &str = "metadata key must be 1 to 40 characters";

/// Diagnostic for a metadata value outside Stripe's documented limit.
pub(crate) const METADATA_VALUE_LIMIT_MESSAGE: &str =
    "metadata value must be at most 500 characters";

/// Byte of the hyphen form encoding passes through unescaped.
pub(crate) const UNRESERVED_DASH: u8 = b'-';

/// Byte of the underscore form encoding passes through unescaped.
pub(crate) const UNRESERVED_UNDERSCORE: u8 = b'_';

/// Byte of the dot form encoding passes through unescaped.
pub(crate) const UNRESERVED_DOT: u8 = b'.';

/// Byte of the tilde form encoding passes through unescaped.
pub(crate) const UNRESERVED_TILDE: u8 = b'~';

/// The byte a space encodes to in a form body.
pub(crate) const SPACE: u8 = b' ';

/// The character a space becomes in a form body.
pub(crate) const PLUS_SIGN: char = '+';
