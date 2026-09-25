use super::*;

/// Macro for adding trace-level instrumentation to functions.
///
/// This macro instruments functions with trace-level logging and automatically includes `skip_all`
/// to exclude all function arguments from the span fields.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with trace-level logging and skip_all enabled
pub(crate) fn instrument_trace_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: proc_macro2::TokenStream = attr.into();
    let expanded: proc_macro2::TokenStream = quote! {
        #[::tracing::instrument(level = "trace", skip_all, #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding debug-level instrumentation to functions.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with debug-level logging
pub(crate) fn instrument_debug_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: proc_macro2::TokenStream = attr.into();
    let expanded: proc_macro2::TokenStream = quote! {
        #[::tracing::instrument(level = "debug", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding info-level instrumentation to functions.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with info-level logging
pub(crate) fn instrument_info_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: proc_macro2::TokenStream = attr.into();
    let expanded: proc_macro2::TokenStream = quote! {
        #[::tracing::instrument(level = "info", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding warn-level instrumentation to functions.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with warn-level logging
pub(crate) fn instrument_warn_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: proc_macro2::TokenStream = attr.into();
    let expanded: proc_macro2::TokenStream = quote! {
        #[::tracing::instrument(level = "warn", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding error-level instrumentation to functions.
///
/// # Arguments
///
/// - `attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with error-level logging
pub(crate) fn instrument_error_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: proc_macro2::TokenStream = attr.into();
    let expanded: proc_macro2::TokenStream = quote! {
        #[::tracing::instrument(level = "error", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}
