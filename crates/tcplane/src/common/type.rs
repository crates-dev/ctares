use super::*;

/// A type alias for `Arc<RwLock<T>>`.
///
/// Provides thread-safe shared ownership with read-write access.
pub type ArcRwLock<T> = Arc<RwLock<T>>;

/// A type alias for read guard of `ArcRwLock<T>`.
pub type ArcRwLockReadGuard<'a, T> = RwLockReadGuard<'a, T>;

/// A type alias for write guard of `ArcRwLock<T>`.
pub type ArcRwLockWriteGuard<'a, T> = RwLockWriteGuard<'a, T>;

/// A type alias for a hash map with `Arc<dyn Any + Send + Sync>` values.
pub type HashMapArcAnySendSync = HashMap<String, Arc<dyn Any + Send + Sync>>;

/// A type alias for an optional socket address.
pub type OptionSocketAddr = Option<SocketAddr>;

/// A type alias for an optional socket host (IP address).
pub type OptionSocketHost = Option<std::net::IpAddr>;

/// A type alias for an optional socket port.
pub type OptionSocketPort = Option<u16>;

/// A type alias for response data (byte vector).
pub type ResponseData = Vec<u8>;

/// A type alias for response result.
pub type ResponseResult = Result<(), ResponseError>;

/// A type alias for error handling function.
pub type ErrorHandleFn = dyn Fn(String) + Send + Sync;

/// A type alias for arc-wrapped error handling function.
pub type ArcErrorHandle = Arc<ErrorHandleFn>;

/// A type alias for server hook handler function.
pub type ServerHookHandler =
    Arc<dyn Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>;

/// A type alias for a list of server hook handlers.
pub type ServerHookList = Vec<ServerHookHandler>;
