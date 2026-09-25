use super::*;

/// Represents an HTTP-like response structure.
///
/// This structure wraps response data and provides methods for
/// building and sending responses.
#[derive(Clone, Debug, Default)]
pub struct Response(pub(super) ResponseData);
