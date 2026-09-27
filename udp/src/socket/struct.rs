use super::*;

/// Thread-safe wrapper for UDP socket with read-write lock.
///
/// Provides synchronized access to UDP socket operations.
#[derive(Clone, Data, Debug)]
pub struct ArcRwLockUdpSocket {
    /// Underlying UDP socket with read-write lock.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) socket: ArcRwLock<UdpSocket>,
}
