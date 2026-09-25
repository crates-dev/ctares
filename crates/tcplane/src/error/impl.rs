use super::*;

/// Implementation of `std::error::Error` for `ServerError`.
impl std::error::Error for ServerError {}

/// Implementation of `Display` for `ServerError`.
impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBind(data) => write!(f, "Tcp bind error{COLON_SPACE}{data}"),
            Self::TcpRead(data) => write!(f, "Tcp read error{COLON_SPACE}{data}"),
            Self::TcpWrite(data) => write!(f, "Tcp write error{COLON_SPACE}{data}"),
            Self::ConnectionClosed => write!(f, "Connection closed unexpectedly"),
            Self::Unknown(data) => write!(f, "Unknown error{COLON_SPACE}{data}"),
        }
    }
}

/// Implementation of `std::error::Error` for `ResponseError`.
impl std::error::Error for ResponseError {}

/// Implementation of `Display` for `ResponseError`.
impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => write!(f, "Stream not found"),
            Self::Terminated => write!(f, "Connection terminated"),
            Self::WriteError(data) => write!(f, "Write error{COLON_SPACE}{data}"),
            Self::FlushError(data) => write!(f, "Flush error{COLON_SPACE}{data}"),
        }
    }
}
