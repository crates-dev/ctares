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

    /// Return the bracketed parameter name.
    ///
    /// # Returns
    ///
    /// - `&str` - the field's key.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Return the field's value.
    ///
    /// # Returns
    ///
    /// - `&str` - the field's stringified value.
    pub fn get_value(&self) -> &str {
        &self.value
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
    /// # Returns
    ///
    /// - `&[FormField]` - the parameters collected so far.
    pub fn get_fields(&self) -> &[FormField] {
        &self.fields
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

    /// Return a mutable reference to the fields in insertion order.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<FormField>` - the builder's field storage, so a
    ///   caller can insert or replace entries in place.
    pub fn get_fields_mut(&mut self) -> &mut Vec<FormField> {
        &mut self.fields
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
        let existing: Option<usize> = self
            .get_fields()
            .iter()
            .position(|field: &FormField| field.get_key() == key);
        match existing {
            Some(index) => {
                let slot: &mut FormField = &mut self.get_fields_mut()[index];
                *slot = FormField::new(key, value);
            }
            None => {
                self.get_fields_mut().push(FormField::new(key, value));
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
        self.set(metadata_key(&key), value);
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
            body.push_str(percent_encode(field.get_key()).as_str());
            body.push(FORM_KEY_VALUE_SEPARATOR);
            body.push_str(percent_encode(field.get_value()).as_str());
        }
        body
    }
}
