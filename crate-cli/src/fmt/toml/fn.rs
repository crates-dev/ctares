use super::*;

/// Formats TOML files matching VS Code `tamasfe.even-better-toml` (Taplo 0.7.0 defaults with 4-space indentation).
///
/// # Arguments
///
/// - `&str` - Raw TOML source text.
///
/// # Returns
///
/// - `String` - Formatted TOML text, or unchanged input if syntax errors occur.
pub fn format_toml(input: &str) -> String {
    let opts: Options = vscode_tidy_options();
    tidy::toml::format(input, &opts.toml).unwrap_or_else(|_: tidy::Error| input.to_string())
}
