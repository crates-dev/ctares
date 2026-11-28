use super::*;

/// Formats JSON (`.json`, `tsconfig.json`, `.vscode/*.json`) matching VS Code Prettier 3.7.4 `json` parser (`tabWidth: 2`, `printWidth: 80`, `objectWrap: "preserve"`, allows comments, strips trailing commas).
///
/// # Arguments
///
/// - `&str` - Raw JSON source text.
///
/// # Returns
///
/// - `Result<String, io::Error>` - Formatted JSON text, or an invalid-data I/O error on syntax failure.
pub fn format_json(input: &str) -> Result<String, io::Error> {
    format_json_dialect(input, tidy::json::Dialect::Json)
}

/// Formats JSON manifests (`package.json`, `package-lock.json`, `composer.json`) matching VS Code Prettier 3.7.4 `json-stringify` parser.
///
/// # Arguments
///
/// - `&str` - Raw JSON manifest text.
///
/// # Returns
///
/// - `Result<String, io::Error>` - Expanded JSON text, or an invalid-data I/O error on syntax failure.
pub fn format_json_stringify(input: &str) -> Result<String, io::Error> {
    format_json_dialect(input, tidy::json::Dialect::Stringify)
}

/// Formats JSONC (`.jsonc`, `.cjson`) matching VS Code Prettier 3.7.4 `jsonc` parser (preserving comments and `trailingComma: "all"`).
///
/// # Arguments
///
/// - `&str` - Raw JSONC source text.
///
/// # Returns
///
/// - `String` - Formatted JSONC text, or unchanged input if syntax errors occur.
pub fn format_jsonc(input: &str) -> String {
    format_json_dialect(input, tidy::json::Dialect::Jsonc)
        .unwrap_or_else(|_: io::Error| input.to_string())
}

/// Formats JSON source using the requested `tidy::json::Dialect` and VS Code Prettier 3.7.4 options.
///
/// # Arguments
///
/// - `&str` - Raw JSON text.
/// - `tidy::json::Dialect` - Target Prettier JSON dialect (`Json`, `Stringify`, or `Jsonc`).
///
/// # Returns
///
/// - `Result<String, io::Error>` - Formatted JSON string or invalid-data I/O error.
fn format_json_dialect(input: &str, dialect: tidy::json::Dialect) -> Result<String, io::Error> {
    let mut opts: Options = vscode_tidy_options();
    opts.json.dialect = dialect;
    tidy::json::format(input, &opts.json)
        .map_err(|_: tidy::Error| io::Error::new(ErrorKind::InvalidData, ERROR_JSON_PARSE_FAILED))
}
