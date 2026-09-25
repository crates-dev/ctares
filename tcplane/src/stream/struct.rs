use super::*;

/// A thread-safe wrapper around `TcpStream` using `Arc<RwLock<TcpStream>>`.
///
/// This structure provides safe concurrent access to a TCP stream,
/// allowing multiple tasks to read from and write to the stream.
#[derive(Clone, Debug)]
pub struct ArcRwLockStream(pub(super) ArcRwLock<TcpStream>);
