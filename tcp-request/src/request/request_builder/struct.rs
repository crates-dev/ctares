use super::*;

/// Builder for creating and configuring TCP requests.
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    /// The TCP request being configured.
    pub(crate) tcp_request: TcpRequest,
    /// The built TCP request instance.
    pub(crate) builder: TcpRequest,
}
