use super::*;

/// Represents an HTTP-like response structure.
///
/// This structure wraps response data and provides methods for
/// building and sending responses.
///
/// Accessors stay hand-written: this is a tuple struct, so `#[derive(Data)]`
/// would emit index-based names (`get_0`/`set_0`), and `set_data` requires a
/// generic `Into<ResponseData>` parameter the macro cannot express.
#[derive(Clone, Debug, Default)]
pub struct Response(pub(super) ResponseData);
