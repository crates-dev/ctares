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

    /// Replace the signed payload.
    ///
    /// # Arguments
    ///
    /// - `String` - the request body Stripe signed.
    pub fn set_payload(&mut self, payload: String) {
        self.payload = payload;
    }

    /// Replace the signed timestamp.
    ///
    /// # Arguments
    ///
    /// - `i64` - seconds since the Unix epoch.
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    /// Replace the expected digest.
    ///
    /// # Arguments
    ///
    /// - `String` - the hex digest without the `v1=` prefix.
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    /// Return the signed payload.
    ///
    /// # Returns
    ///
    /// - `&str` - the request body Stripe signed.
    pub fn get_payload(&self) -> &str {
        &self.payload
    }

    /// Return the signed timestamp.
    ///
    /// # Returns
    ///
    /// - `i64` - seconds since the Unix epoch.
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Return the expected hex digest.
    ///
    /// # Returns
    ///
    /// - `&str` - the digest without the `v1=` prefix.
    pub fn get_signature(&self) -> &str {
        &self.signature
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
        if constant_time_eq(expected.as_bytes(), self.get_signature().as_bytes()) {
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
}
