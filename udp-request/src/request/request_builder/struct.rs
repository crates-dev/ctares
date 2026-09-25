use super::*;

/// A builder for creating and configuring a `UdpRequest`.
///
/// This struct provides a fluent interface for building a `UdpRequest`.
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    /// The `UdpRequest` instance being built.
    pub(crate) udp_request: UdpRequest,
    /// A mutable `UdpRequest` instance used for configuration.
    pub(crate) builder: UdpRequest,
}
