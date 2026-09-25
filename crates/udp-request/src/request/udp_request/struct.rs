use super::*;

/// Represents a UDP request.
///
/// This struct holds the configuration and response for a UDP request.
#[derive(Clone, Debug)]
pub struct UdpRequest {
    /// The configuration for the UDP request, wrapped in an `Arc<RwLock<>>`.
    pub(crate) config: ArcRwLock<Config>,
    /// The response of the UDP request, wrapped in an `Arc<RwLock<>>`.
    pub(crate) response: ArcRwLock<UdpResponseBinary>,
}
