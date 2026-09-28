use super::*;

/// Marker trait for types that can be stored in the attribute store.
///
/// The store holds `Arc<dyn Any + Send + Sync>`, so every stored value
/// also has to be `Clone` for callers to read it back out.
pub trait AnySendSyncClone: Any + Send + Sync + Clone {}

impl<T> AnySendSyncClone for T where T: Any + Send + Sync + Clone {}
