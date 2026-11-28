use super::*;

/// Formats YAML / YML content matching VS Code `esbenp.prettier-vscode` (Prettier 3.7.4: `tabWidth: 2`, `singleQuote: true`, `printWidth: 80`, `bracketSpacing: true`, `proseWrap: "preserve"`, LF).
///
/// # Arguments
///
/// - `&str` - Raw YAML or YML document text.
///
/// # Returns
///
/// - `String` - Formatted YAML text, or unchanged input if syntax errors occur.
pub fn format_yaml(input: &str) -> String {
    let opts: Options = vscode_tidy_options();
    tidy::yaml::format(input, &opts.yaml).unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats YAML content with an explicit file path so `.prettierrc` files are formatted as JSON when valid JSON or YAML otherwise.
///
/// # Arguments
///
/// - `&str` - Raw YAML or RC file content.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted YAML or JSON text, or unchanged input if syntax errors occur.
pub(crate) fn format_yaml_with_path(input: &str, path: &Path) -> String {
    let mut opts: Options = vscode_tidy_options();
    opts.yaml.filepath = Some(path.to_string_lossy().into_owned());
    tidy::yaml::format(input, &opts.yaml).unwrap_or_else(|_: tidy::Error| input.to_string())
}
