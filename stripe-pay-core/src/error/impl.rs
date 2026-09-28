use super::*;

impl StripeError {
    /// Build an API error from Stripe's error envelope.
    ///
    /// # Arguments
    ///
    /// - `String` - Stripe's machine-readable error code.
    /// - `String` - Stripe's human-readable message.
    /// - `String` - the `request-id` header value.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled API error.
    pub const fn api(code: String, message: String, request_id: String) -> Self {
        Self::Api {
            code,
            message,
            request_id,
        }
    }

    /// Return the machine-readable code when this is an API error.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Stripe's error code, or `None` for the
    ///   transport, parse and signature variants which have no code.
    pub const fn code(&self) -> Option<&str> {
        match self {
            Self::Api { code, .. } => Some(code.as_str()),
            _ => None,
        }
    }

    /// Return the `request-id` when this is an API error.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - the request id to quote to Stripe support, or
    ///   `None` when the failure happened before Stripe responded.
    pub const fn request_id(&self) -> Option<&str> {
        match self {
            Self::Api { request_id, .. } => Some(request_id.as_str()),
            _ => None,
        }
    }

    /// Return whether retrying the same request could succeed.
    ///
    /// A declined card is a settled answer, so retrying is pointless;
    /// a transport failure is not, so callers should back off and
    /// repeat.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the caller should retry with backoff.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Transport(_) => true,
            Self::Api { code, .. } => {
                code == STRIPE_ERROR_RATE_LIMIT
                    || code == STRIPE_ERROR_API_CONNECTION
                    || code == STRIPE_ERROR_LOCK_TIMEOUT
            }
            _ => false,
        }
    }
}
