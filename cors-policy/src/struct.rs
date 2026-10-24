use super::*;

/// The rules that decide which origins may ask a server for an answer.
///
/// The policy holds no server state, so the same rules can be used by any
/// server that answers a browser from another origin.
#[derive(Clone, Debug, Getter)]
pub struct CorsPolicy {
    /// The origins that are allowed, or one wildcard for every origin.
    pub allowed_origins: Vec<String>,
    /// The methods that are allowed.
    pub allowed_methods: Vec<String>,
    /// The request headers that are allowed.
    pub allowed_headers: Vec<String>,
    /// The seconds a browser may cache the answer of a preflight request.
    #[get(type(copy))]
    pub max_age: Option<u64>,
    /// Whether the answer may carry credentials.
    #[get(type(copy))]
    pub allow_credentials: bool,
}
