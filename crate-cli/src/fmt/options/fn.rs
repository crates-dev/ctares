use super::*;

/// Builds the canonical `tidy::Options` matching VS Code `esbenp.prettier-vscode` (Prettier 3.7.4) and `tamasfe.even-better-toml` (Taplo 0.7.0) settings.
///
/// # Returns
///
/// - `Options` - Fully configured multi-language formatter options.
pub(crate) fn vscode_tidy_options() -> Options {
    let mut opts: Options = Options::default();
    let shared: Shared = Shared {
        width: Some(PRETTIER_PRINT_WIDTH),
        indent: Some(PRETTIER_TAB_WIDTH),
        tabs: Some(false),
        eol: Some(Eol::Lf),
        whitespace: Some(Whitespace::Auto),
        single_quote: Some(true),
        trailing_comma: Some(true),
        bracket_spacing: Some(true),
        bracket_same_line: Some(false),
        single_attribute_per_line: Some(false),
        ignore: None,
        format_embedded: Some(true),
    };
    opts.share(&shared);
    opts.js.semi = true;
    opts.js.single_quote = true;
    opts.js.jsx_single_quote = false;
    opts.js.trailing_comma = TrailingComma::All;
    opts.js.jsdoc = false;
    opts.js.wrap = Wrap::Preserve;
    opts.js.whitespace = Whitespace::Auto;
    opts.html.vue_indent_script_and_style = false;
    opts.vue.vue_indent_script_and_style = false;
    opts.yaml.wrap = Wrap::Preserve;
    opts.markdown.wrap = Wrap::Preserve;
    opts.markdown.heading = Heading::Atx;
    opts.toml.column_width = VSCODE_TOML_COLUMN_WIDTH;
    opts.toml.tab_width = VSCODE_TOML_TAB_WIDTH;
    opts.toml.tabs = false;
    opts.toml.eol = Eol::Lf;
    opts.toml.align_entries = false;
    opts.toml.align_comments = true;
    opts.toml.align_single_comments = true;
    opts.toml.array_trailing_comma = true;
    opts.toml.array_auto_expand = true;
    opts.toml.array_auto_collapse = true;
    opts.toml.inline_table_expand = true;
    opts.toml.compact_arrays = true;
    opts.toml.compact_inline_tables = false;
    opts.toml.compact_entries = false;
    opts.toml.indent_tables = false;
    opts.toml.indent_entries = false;
    opts.toml.trailing_newline = true;
    opts.toml.reorder_keys = false;
    opts.toml.reorder_arrays = false;
    opts.toml.reorder_inline_tables = false;
    opts.toml.allowed_blank_lines = VSCODE_TOML_ALLOWED_BLANK_LINES;
    opts
}

/// Formats embedded `<style>` and frontmatter requests inside HTML/Vue/CSS with VS Code `singleQuote: true` parity.
///
/// # Arguments
///
/// - `Request<'_>` - Embedded formatting request emitted by `tidy`.
/// - `&Options` - Base VS Code formatter options.
///
/// # Returns
///
/// - `Option<String>` - Formatted embedded text or `None` to defer to `tidy` built-in handling.
pub(crate) fn format_markup_embedded(request: Request<'_>, opts: &Options) -> Option<String> {
    if request.kind == Kind::FrontMatter && request.lang == EMBED_LANG_YAML {
        let mut yaml_opts: tidy::yaml::Options = opts.yaml.clone();
        yaml_opts.width = request.width;
        return tidy::yaml::format(request.code, &yaml_opts)
            .ok()
            .map(|s: String| s.trim_end_matches(TEXT_LF).to_string());
    }
    if request.kind != Kind::Block || request.attribute {
        return None;
    }
    let dialect: Dialect = match request.lang {
        EMBED_LANG_CSS => Dialect::Css,
        EMBED_LANG_SCSS => Dialect::Scss,
        EMBED_LANG_LESS => Dialect::Less,
        _ => return None,
    };
    let indent_width: usize = request.indent.saturating_mul(opts.css.tab_width);
    let mut css_opts: tidy::css::Options = opts.css.clone();
    css_opts.dialect = dialect;
    css_opts.width = request.width.saturating_sub(indent_width);
    let mut embed: Verbatim = Verbatim;
    tidy::css::format(request.code, &css_opts, &mut embed)
        .ok()
        .map(|s: String| indent_embedded_lines(s.trim_end_matches(TEXT_LF), indent_width))
}

/// Indents lines after the first line of an embedded block by `indent_width` spaces, leaving empty lines unindented.
///
/// # Arguments
///
/// - `&str` - Formatted embedded snippet without trailing newline.
/// - `usize` - Number of leading spaces matching the host element's content column.
///
/// # Returns
///
/// - `String` - Snippet whose lines after the first align with `tidy::html`'s literal-line printer.
fn indent_embedded_lines(formatted: &str, indent_width: usize) -> String {
    if indent_width == 0 || !formatted.contains(TEXT_LF) {
        return formatted.to_string();
    }
    let pad: String = " ".repeat(indent_width);
    let mut out: String = String::with_capacity(formatted.len() + indent_width * 4);
    let mut lines: Split<'_, char> = formatted.split(TEXT_LF);
    if let Some(first) = lines.next() {
        out.push_str(first);
    }
    for line in lines {
        out.push(TEXT_LF);
        if !line.is_empty() {
            out.push_str(&pad);
            out.push_str(line);
        }
    }
    out
}
