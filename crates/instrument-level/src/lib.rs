//! A Rust procedural macro collection providing convenient tracing instrumentation macros
//! for different log levels (trace, debug, info, warn, error). This crate simplifies
//! the process of adding tracing spans to functions with pre-configured log levels.

mod r#fn;

use r#fn::*;

use {proc_macro::TokenStream, quote::quote, syn::*};

/// Enables trace-level instrumentation for the decorated function.
///
/// This attribute macro wraps the function with `#[::tracing::instrument(level = "trace", skip_all)]`,
/// enabling automatic tracing instrumentation at the trace level with all arguments excluded from span fields.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The TokenStream representing the function to be instrumented.
///
/// # Returns
///
/// Returns the expanded TokenStream with tracing instrumentation applied.
///
/// # Examples
///
/// ```
/// use tracing;
/// use instrument_level::*;
///
/// #[instrument_trace]
/// async fn test(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument_trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_trace_macro(attr, item)
}

/// Enables debug-level instrumentation for the decorated function.
///
/// This attribute macro wraps the function with `#[::tracing::instrument(level = "debug")]`,
/// enabling automatic tracing instrumentation at the debug level.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The TokenStream representing the function to be instrumented.
///
/// # Returns
///
/// Returns the expanded TokenStream with tracing instrumentation applied.
///
/// # Examples
///
/// ```
/// use tracing;
/// use instrument_level::*;
///
/// #[instrument_debug]
/// async fn test(x: i32, y: i32) -> i32 {
///     x + y
/// }
///
/// #[instrument_debug(skip_all)]
/// async fn test_skip_all(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument_debug(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_debug_macro(attr, item)
}

/// Enables info-level instrumentation for the decorated function.
///
/// This attribute macro wraps the function with `#[::tracing::instrument(level = "info")]`,
/// enabling automatic tracing instrumentation at the info level.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The TokenStream representing the function to be instrumented.
///
/// # Returns
///
/// Returns the expanded TokenStream with tracing instrumentation applied.
///
/// # Examples
///
/// ```
/// use tracing;
/// use instrument_level::*;
///
/// #[instrument_info]
/// async fn test(x: i32, y: i32) -> i32 {
///     x + y
/// }
///
/// #[instrument_info(skip_all)]
/// async fn test_skip_all(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument_info(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_info_macro(attr, item)
}

/// Enables warn-level instrumentation for the decorated function.
///
/// This attribute macro wraps the function with `#[::tracing::instrument(level = "warn")]`,
/// enabling automatic tracing instrumentation at the warn level.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The TokenStream representing the function to be instrumented.
///
/// # Returns
///
/// Returns the expanded TokenStream with tracing instrumentation applied.
///
/// # Examples
///
/// ```
/// use tracing;
/// use instrument_level::*;
///
/// #[instrument_warn]
/// async fn test(x: i32, y: i32) -> i32 {
///     x + y
/// }
///
/// #[instrument_warn(skip_all)]
/// async fn test_skip_all(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument_warn(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_warn_macro(attr, item)
}

/// Enables error-level instrumentation for the decorated function.
///
/// This attribute macro wraps the function with `#[::tracing::instrument(level = "error")]`,
/// enabling automatic tracing instrumentation at the error level.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The TokenStream representing the function to be instrumented.
///
/// # Returns
///
/// Returns the expanded TokenStream with tracing instrumentation applied.
///
/// # Examples
///
/// ```
/// use tracing;
/// use instrument_level::*;
///
/// #[instrument_error]
/// async fn test(x: i32, y: i32) -> i32 {
///     x + y
/// }
///
/// #[instrument_error(skip_all)]
/// async fn test_skip_all(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_error_macro(attr, item)
}
