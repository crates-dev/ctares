/// Gets the number of available threads for parallel processing.
///
/// This function returns the number of threads that the system can execute simultaneously,
/// which is useful for determining optimal parallelism levels.
///
/// # Returns
///
/// - `usize` - The number of available threads, or 1 if the value cannot be determined.
pub fn get_thread_count() -> usize {
    match std::thread::available_parallelism() {
        Ok(count) => count.get(),
        Err(_) => 1,
    }
}
