use super::*;

/// JWT configuration struct containing secret key, expiration time, and issuer.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JwtConfig {
    /// The secret key used for signing JWT tokens.
    pub(super) secret: String,
    /// Token expiration time in seconds.
    pub(super) expiration_seconds: u64,
    /// The issuer of the JWT token.
    pub(super) issuer: String,
}

/// Standard JWT claims struct with common fields.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JwtExtraJwtClaims {
    /// The subject (user identifier) of the token.
    pub(super) sub: String,
    /// The issuer of the token.
    pub(super) iss: String,
    /// Expiration time as a Unix timestamp.
    pub(super) exp: usize,
    /// Issued at time as a Unix timestamp.
    pub(super) iat: usize,
    /// Not before time as a Unix timestamp.
    pub(super) nbf: usize,
}

/// JWT token response struct containing the token and metadata.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JwtToken {
    /// The encoded JWT token string.
    pub(super) token: String,
    /// The token type (typically "Bearer").
    pub(super) token_type: String,
    /// Token expiration time in seconds.
    pub(super) expires_in: u64,
}

/// JWT service struct providing token generation and validation functionality.
#[derive(Clone, Debug)]
pub struct JwtService {
    /// The JWT configuration.
    pub(super) config: JwtConfig,
    /// The encoding key for signing tokens.
    pub(super) encoding_key: EncodingKey,
    /// The decoding key for verifying tokens.
    pub(super) decoding_key: DecodingKey,
    /// The validation settings for token verification.
    pub(super) validation: Validation,
}

/// Generic JWT claims struct that supports custom payload data.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomExtraJwtClaims<T: Default> {
    /// The custom payload data.
    #[serde(flatten)]
    pub(super) custom: T,
    /// The subject (user identifier) of the token.
    pub(super) sub: String,
    /// The issuer of the token.
    pub(super) iss: String,
    /// Expiration time as a Unix timestamp.
    pub(super) exp: usize,
    /// Issued at time as a Unix timestamp.
    pub(super) iat: usize,
}

/// Extended JWT claims struct with support for custom extra fields.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExtraJwtClaims {
    /// The subject (user identifier) of the token.
    pub(super) sub: String,
    /// The issuer of the token.
    pub(super) iss: String,
    /// Expiration time as a Unix timestamp.
    pub(super) exp: usize,
    /// Issued at time as a Unix timestamp.
    pub(super) iat: usize,
    /// Additional custom claims.
    #[serde(flatten)]
    pub(super) extra: HashMap<String, Value>,
}
