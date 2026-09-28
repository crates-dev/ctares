use super::*;

/// Type alias for thread-safe attribute storage.
///
/// This type is used to store arbitrary data that can be shared across
/// different parts of the application in a thread-safe manner.
pub type ThreadSafeAttributeStore = HashMap<String, Arc<dyn Any + Send + Sync>>;
