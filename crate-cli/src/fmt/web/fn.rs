use super::*;

/// Formats JavaScript (`.js`, `.mjs`, `.cjs`, `.wxs`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `semi: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw JavaScript source text.
///
/// # Returns
///
/// - `String` - Formatted JavaScript text, or unchanged input if syntax errors occur.
pub fn format_javascript(input: &str) -> String {
    format_javascript_with_path(input, Path::new(WEB_HINT_JS))
}

/// Formats TypeScript (`.ts`, `.mts`, `.cts`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `semi: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw TypeScript source text.
///
/// # Returns
///
/// - `String` - Formatted TypeScript text, or unchanged input if syntax errors occur.
pub fn format_typescript(input: &str) -> String {
    format_typescript_with_path(input, Path::new(WEB_HINT_TS))
}

/// Formats TypeScript JSX (`.tsx`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `jsxSingleQuote: false`, `semi: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw TSX source text.
///
/// # Returns
///
/// - `String` - Formatted TSX text, or unchanged input if syntax errors occur.
pub fn format_tsx(input: &str) -> String {
    format_tsx_with_path(input, Path::new(WEB_HINT_TSX))
}

/// Formats CSS (`.css`, `.wxss`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw CSS stylesheet text.
///
/// # Returns
///
/// - `String` - Formatted CSS text, or unchanged input if syntax errors occur.
pub fn format_css(input: &str) -> String {
    format_stylesheet(input, Dialect::Css)
}

/// Formats SCSS (`.scss`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw SCSS stylesheet text.
///
/// # Returns
///
/// - `String` - Formatted SCSS text, or unchanged input if syntax errors occur.
pub fn format_scss(input: &str) -> String {
    format_stylesheet(input, Dialect::Scss)
}

/// Formats Less (`.less`) matching VS Code Prettier 3.7.4 (`singleQuote: true`, `tabWidth: 2`).
///
/// # Arguments
///
/// - `&str` - Raw Less stylesheet text.
///
/// # Returns
///
/// - `String` - Formatted Less text, or unchanged input if syntax errors occur.
pub fn format_less(input: &str) -> String {
    format_stylesheet(input, Dialect::Less)
}

/// Formats HTML (`.html`, `.htm`) including embedded `<script>` and `<style>` blocks matching VS Code Prettier 3.7.4.
///
/// # Arguments
///
/// - `&str` - Raw HTML document text.
///
/// # Returns
///
/// - `String` - Formatted HTML text, or unchanged input if syntax errors occur.
pub fn format_html(input: &str) -> String {
    format_html_with_path(input, Path::new(WEB_HINT_HTML))
}

/// Formats Vue Single-File Components (`.vue`) including embedded `<script>` and `<style>` blocks matching VS Code Prettier 3.7.4.
///
/// # Arguments
///
/// - `&str` - Raw Vue SFC source text.
///
/// # Returns
///
/// - `String` - Formatted Vue SFC text, or unchanged input if syntax errors occur.
pub fn format_vue(input: &str) -> String {
    format_vue_with_path(input, Path::new(WEB_HINT_VUE))
}

/// Formats JavaScript with an explicit file path so `.cjs` CommonJS semantics are preserved.
///
/// # Arguments
///
/// - `&str` - Raw JavaScript source text.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted JavaScript text, or unchanged input on syntax error.
pub(crate) fn format_javascript_with_path(input: &str, path: &Path) -> String {
    let opts: Options = vscode_tidy_options();
    let path_str: String = path.to_string_lossy().into_owned();
    let mut embed: Verbatim = Verbatim;
    tidy::format(
        input,
        Language::Js(tidy::js::Dialect::Js),
        &opts,
        Some(path_str.as_str()),
        &mut embed,
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats TypeScript with an explicit file path so `.cts` and `.d.ts` semantics are preserved.
///
/// # Arguments
///
/// - `&str` - Raw TypeScript source text.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted TypeScript text, or unchanged input on syntax error.
pub(crate) fn format_typescript_with_path(input: &str, path: &Path) -> String {
    let opts: Options = vscode_tidy_options();
    let path_str: String = path.to_string_lossy().into_owned();
    let mut embed: Verbatim = Verbatim;
    tidy::format(
        input,
        Language::Js(tidy::js::Dialect::Ts),
        &opts,
        Some(path_str.as_str()),
        &mut embed,
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats TSX with an explicit file path so JSX-in-TypeScript parsing is enabled.
///
/// # Arguments
///
/// - `&str` - Raw TSX source text.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted TSX text, or unchanged input on syntax error.
pub(crate) fn format_tsx_with_path(input: &str, path: &Path) -> String {
    let opts: Options = vscode_tidy_options();
    let path_str: String = path.to_string_lossy().into_owned();
    let mut embed: Verbatim = Verbatim;
    tidy::format(
        input,
        Language::Js(tidy::js::Dialect::Ts),
        &opts,
        Some(path_str.as_str()),
        &mut embed,
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats HTML with an explicit file path so HTML5 `<!doctype html>` casing matches Prettier 3.7.4.
///
/// # Arguments
///
/// - `&str` - Raw HTML document text.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted HTML text, or unchanged input on syntax error.
pub(crate) fn format_html_with_path(input: &str, path: &Path) -> String {
    let opts: Options = vscode_tidy_options();
    let path_str: String = path.to_string_lossy().into_owned();
    tidy::format(
        input,
        Language::Html,
        &opts,
        Some(path_str.as_str()),
        &mut |req: Request<'_>| format_markup_embedded(req, &opts),
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats Vue SFC with an explicit file path and embedded CSS/JS Prettier parity.
///
/// # Arguments
///
/// - `&str` - Raw Vue SFC source text.
/// - `&Path` - Source file path hint.
///
/// # Returns
///
/// - `String` - Formatted Vue SFC text, or unchanged input on syntax error.
pub(crate) fn format_vue_with_path(input: &str, path: &Path) -> String {
    let opts: Options = vscode_tidy_options();
    let path_str: String = path.to_string_lossy().into_owned();
    tidy::format(
        input,
        Language::Vue,
        &opts,
        Some(path_str.as_str()),
        &mut |req: Request<'_>| format_markup_embedded(req, &opts),
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}

/// Formats a stylesheet in the requested `Dialect` with VS Code Prettier 3.7.4 options.
///
/// # Arguments
///
/// - `&str` - Raw stylesheet source text.
/// - `Dialect` - Target stylesheet dialect (`Css`, `Scss`, or `Less`).
///
/// # Returns
///
/// - `String` - Formatted stylesheet text, or unchanged input on syntax error.
fn format_stylesheet(input: &str, dialect: Dialect) -> String {
    let opts: Options = vscode_tidy_options();
    tidy::format(
        input,
        Language::Css(dialect),
        &opts,
        None,
        &mut |req: Request<'_>| format_markup_embedded(req, &opts),
    )
    .unwrap_or_else(|_: tidy::Error| input.to_string())
}
