use super::*;

/// JWT configuration struct containing secret key, expiration time, and issuer.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JwtConfig {
    /// The secret key used for signing JWT tokens.
    pub secret: String,
    /// Token expiration time in seconds.
    #[get(type(copy))]
    pub expiration_seconds: u64,
    /// The issuer of the JWT token.
    pub issuer: String,
}

/// Standard JWT claims struct with common fields.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JwtExtraJwtClaims {
    /// The subject (user identifier) of the token.
    pub sub: String,
    /// The issuer of the token.
    pub iss: String,
    /// Expiration time as a Unix timestamp.
    #[get(type(copy))]
    pub exp: usize,
    /// Issued at time as a Unix timestamp.
    #[get(type(copy))]
    pub iat: usize,
    /// Not before time as a Unix timestamp.
    #[get(type(copy))]
    pub nbf: usize,
}

/// JWT token response struct containing the token and metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JwtToken {
    /// The encoded JWT token string.
    pub token: String,
    /// The token type (typically the `Bearer` scheme).
    pub token_type: String,
    /// Token expiration time in seconds.
    #[get(type(copy))]
    pub expires_in: u64,
}

/// JWT service struct providing token generation and validation functionality.
#[derive(Clone, Data, Debug)]
pub struct JwtService {
    /// The JWT configuration.
    pub config: JwtConfig,
    /// The encoding key for signing tokens.
    pub encoding_key: EncodingKey,
    /// The decoding key for verifying tokens.
    pub decoding_key: DecodingKey,
    /// The validation settings for token verification.
    pub validation: Validation,
}

/// Generic JWT claims struct that supports custom payload data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CustomExtraJwtClaims<T: Default> {
    /// The custom payload data.
    #[serde(flatten)]
    pub custom: T,
    /// The subject (user identifier) of the token.
    pub sub: String,
    /// The issuer of the token.
    pub iss: String,
    /// Expiration time as a Unix timestamp.
    #[get(type(copy))]
    pub exp: usize,
    /// Issued at time as a Unix timestamp.
    #[get(type(copy))]
    pub iat: usize,
}

/// Extended JWT claims struct with support for custom extra fields.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ExtraJwtClaims {
    /// The subject (user identifier) of the token.
    pub sub: String,
    /// The issuer of the token.
    pub iss: String,
    /// Expiration time as a Unix timestamp.
    #[get(type(copy))]
    pub exp: usize,
    /// Issued at time as a Unix timestamp.
    #[get(type(copy))]
    pub iat: usize,
    /// Additional custom claims.
    #[serde(flatten)]
    pub(super) extra: HashMap<String, Value>,
}
