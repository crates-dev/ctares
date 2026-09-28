use super::*;

/// Helper function to wrap data in an `Arc<RwLock<T>>`.
///
/// # Arguments
///
/// - `T` - The data to wrap.
///
/// # Returns
///
/// - `ArcRwLock<T>` - The wrapped data.
#[inline(always)]
pub fn arc_rwlock<T>(data: T) -> ArcRwLock<T> {
    Arc::new(RwLock::new(data))
}
