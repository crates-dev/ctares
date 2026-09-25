use super::*;

/// A type alias for a read guard of `ArcRwLock<TcpStream>`.
pub type ArcRwLockStreamReadGuard<'a> = ArcRwLockReadGuard<'a, TcpStream>;

/// A type alias for a write guard of `ArcRwLock<TcpStream>`.
pub type ArcRwLockStreamWriteGuard<'a> = ArcRwLockWriteGuard<'a, TcpStream>;

/// A type alias for an optional `ArcRwLockStream`.
pub type OptionArcRwLockStream = Option<ArcRwLockStream>;
