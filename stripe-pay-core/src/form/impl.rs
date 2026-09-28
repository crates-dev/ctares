use super::*;

impl FormField {
    /// Build one bracketed parameter.
    ///
    /// # Arguments
    ///
    /// - `String` - the bracketed parameter name.
    /// - `String` - the field's value.
    ///
    /// # Returns
    ///
    /// - `Self` - the field.
    pub const fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    /// Percent-encode a form key or value for Stripe's request body.
    ///
    /// Stripe expects `application/x-www-form-urlencoded`, which
    /// encodes a space as `+` and every other reserved character as
    /// `%XX` over uppercase hex digits. Encoding the bracketed key is
    /// what turns `metadata[order_id]` into `metadata%5Border_id%5D`.
    ///
    /// # Arguments
    ///
    /// - `&str` - the raw key or value to escape.
    ///
    /// # Returns
    ///
    /// - `String` - the percent-encoded text.
    pub fn percent_encode(raw: &str) -> String {
        let mut encoded: String = String::with_capacity(raw.len());
        for byte in raw.as_bytes() {
            let byte: u8 = *byte;
            if Self::is_unreserved(byte) {
                encoded.push(char::from(byte));
                continue;
            }
            if byte == SPACE {
                encoded.push(PLUS_SIGN);
                continue;
            }
            let hex: String = format!("{:02X}", byte);
            encoded.push('%');
            encoded.push_str(hex.as_str());
        }
        encoded
    }

    /// Return whether a byte may appear in a form body unescaped.
    ///
    /// This is the RFC 3986 unreserved set plus the five characters
    /// `application/x-www-form-urlencoded` also passes through:
    /// alphanumerics and `-` `_` `.` `~`. Brackets deliberately fall
    /// outside the set, which is what turns `metadata[order_id]` into
    /// `metadata%5Border_id%5D`.
    ///
    /// # Arguments
    ///
    /// - `u8` - the byte to classify.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the byte needs no percent-escaping.
    pub fn is_unreserved(byte: u8) -> bool {
        byte.is_ascii_alphanumeric()
            || byte == UNRESERVED_DASH
            || byte == UNRESERVED_UNDERSCORE
            || byte == UNRESERVED_DOT
            || byte == UNRESERVED_TILDE
    }

    /// Build the bracketed key Stripe uses for one metadata entry.
    ///
    /// # Arguments
    ///
    /// - `&str` - the bare metadata key such as `order_id`.
    ///
    /// # Returns
    ///
    /// - `String` - the full parameter name `metadata[order_id]`.
    pub fn metadata_key(key: &str) -> String {
        let mut full: String = String::with_capacity(METADATA_PREFIX.len() + key.len() + 2);
        full.push_str(METADATA_PREFIX);
        full.push(FORM_OPEN_BRACKET);
        full.push_str(key);
        full.push(FORM_CLOSE_BRACKET);
        full
    }

    /// Build the bracketed key Stripe uses for one `expand[]` entry.
    ///
    /// Stripe takes repeated list parameters as `expand[0]`, `expand[1]`,
    /// so a caller expanding two fields must get two distinct keys.
    ///
    /// # Arguments
    ///
    /// - `usize` - the field's position in the expand list.
    ///
    /// # Returns
    ///
    /// - `String` - the full parameter name `expand[0]`.
    pub fn expand_key(index: usize) -> String {
        let mut full: String = String::with_capacity(EXPAND_PREFIX.len() + 8);
        full.push_str(EXPAND_PREFIX);
        full.push(FORM_OPEN_BRACKET);
        full.push_str(index.to_string().as_str());
        full.push(FORM_CLOSE_BRACKET);
        full
    }
}

impl FormParams {
    /// Build an empty parameter set.
    ///
    /// # Returns
    ///
    /// - `Self` - a builder with no fields.
    pub const fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Return the fields in insertion order.
    ///
    /// `#[derive(Getter)]` cannot express this accessor: a `clone`
    /// getter hands back an owned `Vec` whose borrow dies at the end
    /// of the statement, and a `deref` getter breaks the matching
    /// `GetterMut` signature. A slice is the only return type that lets
    /// `encode` sort the fields in place.
    ///
    /// # Returns
    ///
    /// - `&[FormField]` - the fields, in insertion order.
    pub fn get_fields(&self) -> &[FormField] {
        &self.fields
    }

    /// Return a mutable view of the fields.
    ///
    /// Kept crate-private on purpose: the public `set` builder is the
    /// only path that should add a field, because it is what preserves
    /// the one-entry-per-key invariant.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<FormField>` - the fields, for in-place maintenance.
    fn get_mut_fields(&mut self) -> &mut Vec<FormField> {
        &mut self.fields
    }

    /// Return the number of fields in the builder.
    ///
    /// # Returns
    ///
    /// - `usize` - the field count.
    pub fn len(&self) -> usize {
        self.get_fields().len()
    }

    /// Return whether the builder holds no fields.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when there is nothing to encode.
    pub fn is_empty(&self) -> bool {
        self.get_fields().is_empty()
    }

    /// Add a field, replacing any existing field with the same key.
    ///
    /// # Arguments
    ///
    /// - `String` - the bracketed parameter name.
    /// - `String` - the field's value.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - the builder with the field added or replaced.
    pub fn set(&mut self, key: String, value: String) -> &mut Self {
        let fields: &mut Vec<FormField> = self.get_mut_fields();
        let existing: Option<usize> = fields
            .iter()
            .position(|field: &FormField| *field.get_key() == key);
        match existing {
            Some(index) => {
                fields[index] = FormField::new(key, value);
            }
            None => {
                fields.push(FormField::new(key, value));
            }
        }
        self
    }

    /// Add a field and return the builder by value.
    ///
    /// # Arguments
    ///
    /// - `String` - the bracketed parameter name.
    /// - `String` - the field's value.
    ///
    /// # Returns
    ///
    /// - `Self` - the builder with the field added or replaced.
    pub fn with(mut self, key: String, value: String) -> Self {
        self.set(key, value);
        self
    }

    /// Add a nested metadata entry using Stripe's bracket notation.
    ///
    /// Stripe limits metadata keys to 40 characters and values to 500;
    /// both are checked here so a bad key fails at build time rather
    /// than as an opaque 400 from the API.
    ///
    /// # Arguments
    ///
    /// - `String` - the metadata key.
    /// - `String` - the metadata value.
    ///
    /// # Returns
    ///
    /// - `Result<Self, StripeError>` - the builder, or the reason the
    ///   key or value exceeded Stripe's documented limits.
    pub fn with_metadata(mut self, key: String, value: String) -> Result<Self, StripeError> {
        if key.is_empty() || key.chars().count() > METADATA_KEY_MAX_CHARS {
            return Err(StripeError::Malformed(String::from(
                METADATA_KEY_LIMIT_MESSAGE,
            )));
        }
        if value.chars().count() > METADATA_VALUE_MAX_CHARS {
            return Err(StripeError::Malformed(String::from(
                METADATA_VALUE_LIMIT_MESSAGE,
            )));
        }
        self.set(FormField::metadata_key(&key), value);
        Ok(self)
    }

    /// Return the body as a percent-encoded `key=value` string.
    ///
    /// Keys are sorted so the same logical request always produces the
    /// same body, which makes request signing and test fixtures
    /// reproducible.
    ///
    /// # Returns
    ///
    /// - `String` - the encoded body.
    pub fn encode(&self) -> String {
        let mut sorted: Vec<&FormField> = self.get_fields().iter().collect();
        sorted.sort_by(|left: &&FormField, right: &&FormField| left.get_key().cmp(right.get_key()));
        let mut body: String = String::new();
        for field in sorted {
            if !body.is_empty() {
                body.push(FORM_PAIR_SEPARATOR);
            }
            body.push_str(FormField::percent_encode(field.get_key()).as_str());
            body.push(FORM_KEY_VALUE_SEPARATOR);
            body.push_str(FormField::percent_encode(field.get_value()).as_str());
        }
        body
    }

    /// Build the request body for creating a PaymentIntent.
    ///
    /// Stripe creates a PaymentIntent with a flat form body; the amount
    /// travels as a minor-unit integer and the currency as its ISO 4217
    /// code, exactly as `Money` stores them.
    ///
    /// # Arguments
    ///
    /// - `Money` - the amount to capture.
    /// - `Option<String>` - the customer to charge, or `None` for an
    ///   intent that is not yet attached to a customer.
    ///
    /// # Returns
    ///
    /// - `String` - the encoded request body.
    pub fn encode_create_payment_intent(amount: Money, customer: Option<String>) -> String {
        let mut params: Self = Self::new();
        params.set(String::from(FORM_AMOUNT), amount.get_amount().to_string());
        params.set(
            String::from(FORM_CURRENCY),
            String::from(amount.currency_code()),
        );
        if let Some(customer_id) = customer {
            params.set(String::from(FORM_CUSTOMER), customer_id);
        }
        params.encode()
    }

    /// Build the request body for refunding part or all of a charge.
    ///
    /// # Arguments
    ///
    /// - `&str` - the charge being refunded.
    /// - `Option<Money>` - the amount to return, or `None` to refund
    ///   the charge's full remaining balance.
    /// - `RefundReason` - why the money is being returned.
    ///
    /// # Returns
    ///
    /// - `String` - the encoded request body.
    pub fn encode_create_refund(
        charge: &str,
        amount: Option<Money>,
        reason: RefundReason,
    ) -> String {
        let mut params: Self = Self::new();
        params.set(String::from(FORM_CHARGE), String::from(charge));
        if let Some(refund_amount) = amount {
            params.set(
                String::from(FORM_AMOUNT),
                refund_amount.get_amount().to_string(),
            );
        }
        params.set(String::from(FORM_REASON), String::from(reason.as_str()));
        params.encode()
    }

    /// Build the request body for confirming a PaymentIntent.
    ///
    /// Confirmation names the payment method and may set the return URL
    /// the browser lands on once 3-D Secure finishes.
    ///
    /// # Arguments
    ///
    /// - `&str` - the PaymentIntent to confirm.
    /// - `&str` - the payment method to charge.
    /// - `Option<&str>` - the browser return URL, or `None` when the
    ///   payment method owes no redirect.
    ///
    /// # Returns
    ///
    /// - `String` - the encoded request body.
    pub fn encode_confirm_payment_intent(
        payment_intent: &str,
        payment_method: &str,
        return_url: Option<&str>,
    ) -> String {
        let mut params: Self = Self::new();
        params.set(
            String::from(FORM_PAYMENT_INTENT),
            String::from(payment_intent),
        );
        params.set(
            String::from(FORM_PAYMENT_METHOD),
            String::from(payment_method),
        );
        params.set(String::from(FORM_CONFIRM), String::from(TRUE_LITERAL));
        if let Some(url) = return_url {
            params.set(String::from(FORM_RETURN_URL), String::from(url));
        }
        params.encode()
    }

    /// Build the request body for creating a Customer.
    ///
    /// Stripe accepts a customer's email and description as flat form
    /// fields; both are optional.
    ///
    /// # Arguments
    ///
    /// - `Option<&str>` - the customer's email, or `None` to omit it.
    /// - `Option<&str>` - an internal description, or `None` to omit it.
    ///
    /// # Returns
    ///
    /// - `String` - the encoded request body.
    pub fn encode_create_customer(email: Option<&str>, description: Option<&str>) -> String {
        let mut params: Self = Self::new();
        if let Some(address) = email {
            params.set(String::from(FORM_EMAIL), String::from(address));
        }
        if let Some(text) = description {
            params.set(String::from(FORM_DESCRIPTION), String::from(text));
        }
        params.encode()
    }
}
