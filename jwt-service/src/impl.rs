use super::*;

impl JwtConfig {
    /// Creates a new JwtConfig instance.
    ///
    /// # Arguments
    ///
    /// - `S` - The secret key for signing tokens.
    /// - `u64` - Token validity duration in seconds.
    /// - `S` - The issuer identifier for the token.
    ///
    /// # Returns
    ///
    /// - `JwtConfig` - The configured JwtConfig instance.
    pub fn new<S>(secret: S, expiration_seconds: u64, issuer: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            secret: secret.as_ref().to_string(),
            expiration_seconds,
            issuer: issuer.as_ref().to_string(),
        }
    }
}

impl JwtExtraJwtClaims {
    /// Creates a new JwtExtraJwtClaims instance.
    ///
    /// # Arguments
    ///
    /// - `S` - The subject (user identifier).
    /// - `S` - The issuer.
    /// - `usize` - Expiration time as a Unix timestamp.
    /// - `usize` - Issued at time as a Unix timestamp.
    /// - `usize` - Not before time as a Unix timestamp.
    ///
    /// # Returns
    ///
    /// - `JwtExtraJwtClaims` - The created claims instance.
    pub fn new<S>(sub: S, iss: S, exp: usize, iat: usize, nbf: usize) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            sub: sub.as_ref().to_string(),
            iss: iss.as_ref().to_string(),
            exp,
            iat,
            nbf,
        }
    }
}

impl JwtService {
    /// Creates a new JwtService instance.
    ///
    /// # Arguments
    ///
    /// - `JwtConfig` - The JWT configuration.
    /// - `EncodingKey` - The encoding key for signing tokens.
    /// - `DecodingKey` - The decoding key for verifying tokens.
    /// - `Validation` - The validation settings for token verification.
    ///
    /// # Returns
    ///
    /// - `JwtService` - The created service instance.
    pub fn new(
        config: JwtConfig,
        encoding_key: EncodingKey,
        decoding_key: DecodingKey,
        validation: Validation,
    ) -> Self {
        Self {
            config,
            encoding_key,
            decoding_key,
            validation,
        }
    }
}

impl ExtraJwtClaims {
    /// Creates a new ExtraJwtClaims instance.
    ///
    /// # Arguments
    ///
    /// - `S` - The subject (user identifier).
    /// - `S` - The issuer.
    /// - `usize` - Expiration time as a Unix timestamp.
    ///
    /// # Returns
    ///
    /// - `ExtraJwtClaims` - The created claims instance.
    pub fn new<S>(sub: S, iss: S, exp: usize) -> Self
    where
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        Self {
            sub: sub.as_ref().to_string(),
            iss: iss.as_ref().to_string(),
            exp,
            iat: now,
            extra: HashMap::new(),
        }
    }
}

/// Display implementation for JwtValidationError.
impl Display for JwtValidationError {
    /// Formats the validation error for display.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - Result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expired => write!(f, "Token has expired"),
            Self::InvalidSignature => write!(f, "Invalid token signature"),
            Self::InvalidIssuer => write!(f, "Invalid token issuer"),
            Self::InvalidSubject => write!(f, "Invalid token subject"),
            Self::NotYetValid => write!(f, "Token is not yet valid"),
            Self::Malformed => write!(f, "Malformed token"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

/// Error trait implementation for JwtValidationError.
impl Error for JwtValidationError {}

/// Conversion from jsonwebtoken errors to JwtValidationError.
///
/// # Arguments
///
/// - `jsonwebtoken::errors::Error` - The jsonwebtoken error to convert.
///
/// # Returns
///
/// - `JwtValidationError` - The corresponding validation error.
impl From<jsonwebtoken::errors::Error> for JwtValidationError {
    /// Converts a jsonwebtoken error into a validation error.
    ///
    /// # Arguments
    ///
    /// - `jsonwebtoken::errors::Error` - The jsonwebtoken error to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - The corresponding validation error.
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::Expired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => Self::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => Self::InvalidIssuer,
            jsonwebtoken::errors::ErrorKind::InvalidSubject => Self::InvalidSubject,
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => Self::NotYetValid,
            _ => Self::Other(error.to_string()),
        }
    }
}

/// Implementation block for JwtConfig.
impl JwtConfig {
    /// Creates a new JwtConfig instance with the specified settings.
    ///
    /// # Arguments
    ///
    /// - `String` - The secret key for signing tokens.
    /// - `u64` - Token validity duration in seconds.
    /// - `String` - The issuer identifier for the token.
    ///
    /// # Returns
    ///
    /// - `JwtConfig` - The configured JwtConfig instance.
    pub fn with_settings(secret: String, expiration_seconds: u64, issuer: String) -> Self {
        let mut instance: JwtConfig = Self::default();
        instance
            .set_secret(secret)
            .set_expiration_seconds(expiration_seconds)
            .set_issuer(issuer);
        instance
    }
}

/// Conversion from JwtConfig to JwtService.
///
/// # Arguments
///
/// - `JwtConfig` - The JWT configuration to convert.
///
/// # Returns
///
/// - `JwtService` - The initialized JWT service.
impl From<JwtConfig> for JwtService {
    /// Builds a JWT service from its configuration.
    ///
    /// # Arguments
    ///
    /// - `JwtConfig` - The JWT configuration to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized JWT service.
    fn from(config: JwtConfig) -> Self {
        let encoding_key: EncodingKey = EncodingKey::from_secret(config.get_secret().as_bytes());
        let decoding_key: DecodingKey = DecodingKey::from_secret(config.get_secret().as_bytes());
        let mut validation: Validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[config.get_issuer()]);
        Self::new(config, encoding_key, decoding_key, validation)
    }
}

/// Implementation block for JwtService core functionality.
impl JwtService {
    /// Generates a JWT token for the given subject.
    ///
    /// # Arguments
    ///
    /// - `S` - The subject (user identifier) to include in the token.
    ///
    /// # Returns
    ///
    /// - `Result<JwtToken, String>` - The generated token on success, or an error message on failure.
    pub fn generate_token<S>(&self, subject: S) -> Result<JwtToken, String>
    where
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let exp: usize = now + *self.get_config().get_expiration_seconds() as usize;
        let claims: JwtExtraJwtClaims = JwtExtraJwtClaims::new(
            subject.as_ref().to_string(),
            self.get_config().get_issuer().clone(),
            exp,
            now,
            now,
        );
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            self.get_encoding_key(),
        )
        .map_err(|error: jsonwebtoken::errors::Error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token
            .set_token(token)
            .set_token_type(BEARER.to_string())
            .set_expires_in(*self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    /// Validates a JWT token and returns its claims.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to validate.
    ///
    /// # Returns
    ///
    /// - `Result<JwtExtraJwtClaims, JwtValidationError>` - The claims on success, or a validation error on failure.
    pub fn validate_token<T>(&self, token: T) -> Result<JwtExtraJwtClaims, JwtValidationError>
    where
        T: AsRef<str>,
    {
        let token_data: TokenData<JwtExtraJwtClaims> = decode::<JwtExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    /// Extracts the subject from a JWT token.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to extract the subject from.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - The subject on success, or an error message on failure.
    pub fn get_subject_from_token<T>(&self, token: T) -> Result<String, String>
    where
        T: AsRef<str>,
    {
        let claims: JwtExtraJwtClaims = self
            .validate_token(token)
            .map_err(|e: JwtValidationError| e.to_string())?;
        Ok(claims.get_sub().clone())
    }

    /// Checks if a JWT token is expired.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to check.
    ///
    /// # Returns
    ///
    /// - `Result<bool, String>` - True if expired, false otherwise, or an error message on failure.
    pub fn is_token_expired<T>(&self, token: T) -> Result<bool, String>
    where
        T: AsRef<str>,
    {
        match decode::<JwtExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let now: usize = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as usize;
                Ok(*token_data.claims.get_exp() < now)
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

/// Implementation block for JwtService with custom claims support.
impl JwtService {
    /// Generates a JWT token with custom claims.
    ///
    /// # Arguments
    ///
    /// - `S` - The subject (user identifier) to include in the token.
    /// - `U` - The custom claims to include in the token payload.
    ///
    /// # Returns
    ///
    /// - `Result<JwtToken, String>` - The generated token on success, or an error message on failure.
    pub fn generate_token_with_claims<U, S>(
        &self,
        subject: S,
        claims: U,
    ) -> Result<JwtToken, String>
    where
        U: Clone + Default + Serialize + for<'de> Deserialize<'de>,
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let mut res_claims: CustomExtraJwtClaims<U> = CustomExtraJwtClaims::default();
        res_claims
            .set_custom(claims)
            .set_sub(subject.as_ref().to_string())
            .set_iss(self.get_config().get_issuer().clone())
            .set_exp(now + *self.get_config().get_expiration_seconds() as usize)
            .set_iat(now);
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &res_claims,
            self.get_encoding_key(),
        )
        .map_err(|error: jsonwebtoken::errors::Error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token.set_token(token);
        jwt_token.set_token_type(BEARER.to_string());
        jwt_token.set_expires_in(*self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    /// Validates a JWT token and returns its custom claims.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to validate.
    ///
    /// # Returns
    ///
    /// - `Result<CustomExtraJwtClaims<U>, JwtValidationError>` - The custom claims on success, or a validation error on failure.
    pub fn validate_token_with_claims<U, T>(
        &self,
        token: T,
    ) -> Result<CustomExtraJwtClaims<U>, JwtValidationError>
    where
        U: Clone + Default + Serialize + for<'de> Deserialize<'de>,
        T: AsRef<str>,
    {
        let token_data: TokenData<CustomExtraJwtClaims<U>> = decode::<CustomExtraJwtClaims<U>>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    /// Generates a JWT token with extra custom fields.
    ///
    /// # Arguments
    ///
    /// - `S` - The subject (user identifier) to include in the token.
    /// - `HashMap<String, Value>` - Additional key-value pairs to include in the token payload.
    ///
    /// # Returns
    ///
    /// - `Result<JwtToken, String>` - The generated token on success, or an error message on failure.
    pub fn generate_token_with_extra_claims<S>(
        &self,
        subject: S,
        extra: HashMap<String, Value>,
    ) -> Result<JwtToken, String>
    where
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let mut claims: ExtraJwtClaims = ExtraJwtClaims::new(
            subject.as_ref().to_string(),
            self.get_config().get_issuer().clone(),
            now + *self.get_config().get_expiration_seconds() as usize,
        );
        claims.set_extra(extra);
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            self.get_encoding_key(),
        )
        .map_err(|error: jsonwebtoken::errors::Error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token.set_token(token);
        jwt_token.set_token_type(BEARER.to_string());
        jwt_token.set_expires_in(*self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    /// Validates a JWT token and returns its claims with extra fields.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to validate.
    ///
    /// # Returns
    ///
    /// - `Result<ExtraJwtClaims, JwtValidationError>` - The claims with extra fields on success, or a validation error on failure.
    pub fn validate_token_with_extra_claims<T>(
        &self,
        token: T,
    ) -> Result<ExtraJwtClaims, JwtValidationError>
    where
        T: AsRef<str>,
    {
        let token_data: TokenData<ExtraJwtClaims> = decode::<ExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    /// Extracts a specific field value from a JWT token's extra claims.
    ///
    /// # Arguments
    ///
    /// - `T` - The JWT token to extract the field from.
    /// - `K` - The key of the field to retrieve.
    ///
    /// # Returns
    ///
    /// - `Result<Option<Value>, JwtValidationError>` - The field value if found, or None, or a validation error.
    pub fn get_from_token<T, K>(
        &self,
        token: T,
        field_key: K,
    ) -> Result<Option<Value>, JwtValidationError>
    where
        T: AsRef<str>,
        K: AsRef<str>,
    {
        let claims: ExtraJwtClaims = self.validate_token_with_extra_claims(token)?;
        Ok(claims.get(field_key.as_ref()).cloned())
    }
}

/// Implementation block for ExtraJwtClaims helper methods.
impl ExtraJwtClaims {
    /// Inserts a key-value pair into the extra claims.
    ///
    /// # Arguments
    ///
    /// - `String` - The key to insert.
    /// - `Value` - The value to associate with the key.
    ///
    /// # Returns
    ///
    /// - `Self` - The modified ExtraJwtClaims instance for chaining.
    pub fn insert(mut self, key: String, value: Value) -> Self {
        self.get_mut_extra().insert(key, value);
        self
    }

    /// Extends the extra claims with additional key-value pairs.
    ///
    /// # Arguments
    ///
    /// - `HashMap<String, Value>` - A HashMap of key-value pairs to add.
    ///
    /// # Returns
    ///
    /// - `Self` - The modified ExtraJwtClaims instance for chaining.
    pub fn extend_extra(mut self, extra: HashMap<String, Value>) -> Self {
        self.get_mut_extra().extend(extra);
        self
    }

    /// Retrieves a value from the extra claims by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&Value>` - The value if found, or None.
    pub fn get<K>(&self, key: K) -> Option<&Value>
    where
        K: AsRef<str>,
    {
        self.get_extra().get(key.as_ref())
    }

    /// Checks if a key exists in the extra claims.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key exists, false otherwise.
    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_extra().contains_key(key.as_ref())
    }

    /// Removes a key from the extra claims and returns its value.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to remove.
    ///
    /// # Returns
    ///
    /// - `Option<Value>` - The removed value if found, or None.
    pub fn remove<K>(&mut self, key: K) -> Option<Value>
    where
        K: AsRef<str>,
    {
        self.get_mut_extra().remove(key.as_ref())
    }
}
