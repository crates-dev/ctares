/// Combines multiple paths into a single valid path, handling overlapping slashes.
///
/// - Removes trailing slashes from the base path.
/// - Removes leading slashes from subsequent paths to avoid duplication.
/// - Supports multiple path segments for flexible usage.
///
/// # Parameters
/// - `base`: The base path as a string slice. It serves as the starting point for the combined path.
/// - `sub_path`: One or more subsequent paths as string slices. These are appended to the base path in order.
///
/// # Returns
/// - `String`: The resulting combined path as a `String`, with platform-specific separators and cleaned of redundant slashes.
#[macro_export]
macro_rules! join_paths {
    ($base:expr, $($sub_path:expr),+) => {{
        let mut path = std::path::PathBuf::from($base.trim_end_matches(['/', '\\'].as_ref()));
        if cfg!(target_os = "windows") {
            if path.is_dir() && path.to_string_lossy().ends_with(":") {
                path.push("/");
            }
        }
        $(
            let clean_sub_path = $sub_path.trim_start_matches(['/', '\\'].as_ref());
            path.push(clean_sub_path);
        )+
        path.to_string_lossy().replace("\\", "/")
    }};
}
