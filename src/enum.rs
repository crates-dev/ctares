use super::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum JwtValidationError {
    Expired,
    InvalidSignature,
    InvalidIssuer,
    InvalidSubject,
    NotYetValid,
    Malformed,
    Other(String),
}
