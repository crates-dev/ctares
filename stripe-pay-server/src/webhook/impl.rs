use super::*;

impl WebhookError {
    /// Return the reason as a stable diagnostic string.
    ///
    /// # Returns
    ///
    /// - `&'static str` - the message a log line should carry.
    pub const fn message(&self) -> &'static str {
        match self {
            WebhookError::MissingSignature => MESSAGE_NO_SIGNATURE,
            WebhookError::TimestampOutOfTolerance => MESSAGE_TIMESTAMP_OUT_OF_TOLERANCE,
            WebhookError::EmptySecret => MESSAGE_EMPTY_SECRET,
            WebhookError::SignatureMismatch => MESSAGE_SIGNATURE_MISMATCH,
        }
    }
}

impl Display for WebhookError {
    /// Render the diagnostic.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - the formatter to write the value into.
    ///
    /// # Returns
    ///
    /// A `Formatter` carrying the diagnostic message.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message())
    }
}

impl WebhookEvent {
    /// Build a verification target from the three webhook inputs.
    ///
    /// The signature is the bare hex digest, not the whole
    /// `Stripe-Signature` header; `parse_signature_header` extracts it.
    ///
    /// # Arguments
    ///
    /// - `String` - the exact request body Stripe signed.
    /// - `i64` - the timestamp Stripe signed, in seconds.
    /// - `String` - the hex digest Stripe sent.
    ///
    /// # Returns
    ///
    /// - `Self` - the assembled verification target.
    pub fn new(payload: String, timestamp: i64, signature: String) -> Self {
        let mut built: Self = Self {
            payload: String::new(),
            timestamp: 0,
            signature: String::new(),
        };
        built.set_payload(payload);
        built.set_timestamp(timestamp);
        built.set_signature(signature);
        built
    }

    /// Replace the signed payload during construction.
    ///
    /// # Arguments
    ///
    /// - `String` - the request body Stripe signed.
    fn set_payload(&mut self, payload: String) {
        self.payload = payload;
    }

    /// Replace the signed timestamp.
    ///
    /// # Arguments
    ///
    /// - `i64` - seconds since the Unix epoch.
    fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    /// Replace the expected digest.
    ///
    /// # Arguments
    ///
    /// - `String` - the hex digest without the `v1=` prefix.
    fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    /// Sign the payload the way Stripe does.
    ///
    /// Stripe signs the timestamp and the payload joined by a dot,
    /// then HMAC-SHA256 hex-encodes the digest, so this builds that
    /// exact signed text before hashing.
    ///
    /// # Arguments
    ///
    /// - `&str` - the endpoint's webhook signing secret.
    ///
    /// # Returns
    ///
    /// - `Result<String, WebhookError>` - the hex digest Stripe would
    ///   have sent, or an error when the secret is empty.
    pub fn compute_signature(&self, secret: &str) -> Result<String, WebhookError> {
        if secret.is_empty() {
            return Err(WebhookError::EmptySecret);
        }
        let signed: String = format!("{}.{}", self.get_timestamp(), self.get_payload());
        let mut mac: Hmac<Sha256> = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
            Ok(created) => created,
            Err(_invalid_length) => return Err(WebhookError::EmptySecret),
        };
        mac.update(signed.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Return whether the timestamp sits within `tolerance` of `now`.
    ///
    /// Stripe recommends rejecting anything outside a five-minute
    /// window so a captured request cannot be replayed later.
    ///
    /// # Arguments
    ///
    /// - `i64` - the current time in seconds since the epoch.
    /// - `i64` - the accepted drift, in seconds.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the timestamp is inside the window.
    pub fn is_timestamp_fresh(&self, now: i64, tolerance: i64) -> bool {
        let drift: i64 = now - self.get_timestamp();
        if drift < 0 {
            -drift <= tolerance
        } else {
            drift <= tolerance
        }
    }

    /// Verify the digest against the secret.
    ///
    /// # Arguments
    ///
    /// - `&str` - the endpoint's webhook signing secret.
    ///
    /// # Returns
    ///
    /// - `Result<(), WebhookError>` - `Ok(())` when the digest matches.
    pub fn verify_signature(&self, secret: &str) -> Result<(), WebhookError> {
        let expected: String = self.compute_signature(secret)?;
        if Self::constant_time_eq(expected.as_bytes(), self.get_signature().as_bytes()) {
            return Ok(());
        }
        Err(WebhookError::SignatureMismatch)
    }

    /// Verify freshness and then the digest.
    ///
    /// # Arguments
    ///
    /// - `&str` - the endpoint's webhook signing secret.
    /// - `i64` - the current time in seconds since the epoch.
    /// - `i64` - the accepted drift, in seconds.
    ///
    /// # Returns
    ///
    /// - `Result<(), WebhookError>` - `Ok(())` when the event is genuine.
    pub fn verify(&self, secret: &str, now: i64, tolerance: i64) -> Result<(), WebhookError> {
        if !self.is_timestamp_fresh(now, tolerance) {
            return Err(WebhookError::TimestampOutOfTolerance);
        }
        self.verify_signature(secret)
    }

    /// Split a `Stripe-Signature` header into its timestamp and digest.
    ///
    /// Stripe sends comma-separated entries such as
    /// `t=1614556800,v1=deadbeef,v0=ignored`. The `v0` entry is the
    /// legacy scheme and `v1` is the current one, so only `v1` is
    /// accepted; a header carrying no `v1` entry is rejected rather
    /// than downgraded.
    ///
    /// # Arguments
    ///
    /// - `&str` - the raw `Stripe-Signature` header value.
    /// - `&str` - the exact request body Stripe signed.
    ///
    /// # Returns
    ///
    /// - `Result<WebhookEvent, WebhookError>` - the verification
    ///   target, or `MissingSignature` when no timestamp or `v1` digest
    ///   was present.
    pub fn parse_signature_header(
        header: &str,
        payload: &str,
    ) -> Result<WebhookEvent, WebhookError> {
        let mut timestamp: Option<i64> = None;
        let mut signature: Option<String> = None;
        for entry in header.split(TIMESTAMP_SEPARATOR) {
            let Some((key, value)) = entry.split_once(KEY_VALUE_SEPARATOR) else {
                continue;
            };
            match key {
                TIMESTAMP_FIELD => {
                    timestamp = value.parse::<i64>().ok();
                }
                SIGNATURE_SCHEME => {
                    signature = Some(String::from(value));
                }
                _ => {}
            }
        }
        let resolved_timestamp: i64 = match timestamp {
            Some(value) => value,
            None => return Err(WebhookError::MissingSignature),
        };
        let resolved_signature: String = match signature {
            Some(value) => value,
            None => return Err(WebhookError::MissingSignature),
        };
        Ok(WebhookEvent::new(
            String::from(payload),
            resolved_timestamp,
            resolved_signature,
        ))
    }

    /// Read the webhook signing secret from the process environment.
    ///
    /// # Returns
    ///
    /// - `Result<String, WebhookError>` - the configured secret, or
    ///   `EmptySecret` when the variable is unset or blank.
    pub fn secret_from_env() -> Result<String, WebhookError> {
        match std::env::var(SECRET_KEY) {
            Ok(value) if !value.is_empty() => Ok(value),
            _ => Err(WebhookError::EmptySecret),
        }
    }

    /// Verify a raw webhook request end to end.
    ///
    /// This is the entry point a hyperlane handler calls: it parses the
    /// header, then checks freshness and the digest.
    ///
    /// # Arguments
    ///
    /// - `&str` - the raw `Stripe-Signature` header value.
    /// - `&str` - the exact request body Stripe signed.
    /// - `&str` - the endpoint's webhook signing secret.
    /// - `i64` - the current time in seconds since the epoch.
    /// - `i64` - the accepted drift, in seconds.
    ///
    /// # Returns
    ///
    /// - `Result<WebhookEvent, WebhookError>` - the verified event.
    pub fn verify_webhook(
        header: &str,
        payload: &str,
        secret: &str,
        now: i64,
        tolerance: i64,
    ) -> Result<WebhookEvent, WebhookError> {
        let event: WebhookEvent = Self::parse_signature_header(header, payload)?;
        event.verify(secret, now, tolerance)?;
        Ok(event)
    }

    /// Compare two byte slices without leaking their contents through
    /// timing.
    ///
    /// A webhook digest is attacker-supplied, so an early-exit
    /// comparison would leak how many leading bytes matched.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - the digest this crate computed.
    /// - `&[u8]` - the digest the caller supplied.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the two slices are equal.
    fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
        if left.len() != right.len() {
            return false;
        }
        let mut difference: u8 = 0;
        let mut index: usize = 0;
        while index < left.len() {
            difference |= left[index] ^ right[index];
            index += 1;
        }
        difference == 0
    }
}
