use super::*;

/// A type alias for a handler function.
///
/// This type represents a boxed async function that processes a context.
pub type HandlerFunc =
    Box<dyn Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>;

/// A type alias for a list of handler functions.
pub type HandlerList = Vec<HandlerFunc>;

/// A type alias for an arc-wrapped list of handlers.
pub type HandlerListArc = Arc<HandlerList>;

/// A type alias for a RwLock-wrapped arc of handler list.
pub type HandlerListArcLock = ArcRwLock<HandlerList>;
