use super::*;

/// Formats Markdown (`.md` / `.markdown` / `.mdx`) matching VS Code `esbenp.prettier-vscode` (Prettier 3.7.4: `tabWidth: 2`, `singleQuote: true`, `semi: true`, `printWidth: 80`, `proseWrap: "preserve"`, `embeddedLanguageFormatting: "auto"`, LF).
///
/// # Arguments
///
/// - `&str` - Raw Markdown source text.
///
/// # Returns
///
/// - `String` - Formatted Markdown document matching Prettier 3.7.4.
pub fn format_markdown(input: &str) -> String {
    format_markdown_inner(input, PRETTIER_PRINT_WIDTH, 0)
}

/// Formats Markdown at a specific line width and embedded recursion depth.
///
/// # Arguments
///
/// - `&str` - Raw Markdown text.
/// - `usize` - Target maximum line width.
/// - `usize` - Current embedded Markdown nesting depth.
///
/// # Returns
///
/// - `String` - Formatted Markdown text with Prettier 3.7.4 parity post-processing.
fn format_markdown_inner(input: &str, width: usize, depth: usize) -> String {
    let mut opts: Options = vscode_tidy_options();
    opts.markdown.width = width;
    let prepared: String = normalize_markdown_triple_emphasis_prepass(input);
    let formatted: String =
        tidy::markdown::format_with(&prepared, &opts.markdown, |code: Code<'_>| {
            format_markdown_embedded(code, &opts, depth)
        });
    normalize_markdown_prettier_quirks(&formatted)
}

/// Formats Prettier-supported YAML frontmatter and embedded fenced code blocks inside Markdown, leaving non-Prettier languages verbatim.
///
/// # Arguments
///
/// - `Code<'_>` - Embedded code slice, language tag, and available column width.
/// - `&Options` - Base VS Code formatter options.
/// - `usize` - Current embedded Markdown nesting depth.
///
/// # Returns
///
/// - `Option<String>` - Formatted embedded code if supported and valid, or `None` to preserve verbatim.
fn format_markdown_embedded(code: Code<'_>, base_opts: &Options, depth: usize) -> Option<String> {
    let lang_lower: String = code.lang.to_ascii_lowercase();
    let lang: &str = lang_lower.as_str();
    let mut sub_opts: Options = base_opts.clone();
    sub_opts.share(&Shared {
        width: Some(code.width),
        ..Shared::default()
    });
    sub_opts.js.semi = true;
    sub_opts.js.single_quote = true;
    sub_opts.js.jsdoc = false;
    sub_opts.yaml.single_quote = true;
    if MD_EMBED_YAML.contains(&lang) {
        return tidy::yaml::format(code.text, &sub_opts.yaml).ok();
    }
    if lang == MD_EMBED_JSON {
        sub_opts.json.dialect = tidy::json::Dialect::Json;
        return tidy::json::format(code.text, &sub_opts.json).ok();
    }
    if lang == MD_EMBED_JSONC {
        sub_opts.json.dialect = tidy::json::Dialect::Jsonc;
        return tidy::json::format(code.text, &sub_opts.json).ok();
    }
    if lang == MD_EMBED_JSON5 {
        sub_opts.json.dialect = tidy::json::Dialect::Json5;
        return tidy::json::format(code.text, &sub_opts.json).ok();
    }
    if MD_EMBED_JS.contains(&lang) {
        let hint: &str = if lang == "cjs" {
            MD_HINT_CJS
        } else if lang == "jsx" {
            MD_HINT_JSX
        } else {
            MD_HINT_JS
        };
        let mut embed: Verbatim = Verbatim;
        return tidy::format(
            code.text,
            Language::Js(tidy::js::Dialect::Js),
            &sub_opts,
            Some(hint),
            &mut embed,
        )
        .ok();
    }
    if MD_EMBED_TS.contains(&lang) {
        let hint: &str = if lang == "cts" {
            MD_HINT_CTS
        } else if lang == "tsx" {
            MD_HINT_TSX
        } else {
            MD_HINT_TS
        };
        let mut embed: Verbatim = Verbatim;
        return tidy::format(
            code.text,
            Language::Js(tidy::js::Dialect::Ts),
            &sub_opts,
            Some(hint),
            &mut embed,
        )
        .ok();
    }
    if lang == MD_EMBED_CSS {
        return tidy::format(
            code.text,
            Language::Css(Dialect::Css),
            &sub_opts,
            None,
            &mut |req: Request<'_>| format_markup_embedded(req, &sub_opts),
        )
        .ok();
    }
    if lang == MD_EMBED_SCSS {
        return tidy::format(
            code.text,
            Language::Css(Dialect::Scss),
            &sub_opts,
            None,
            &mut |req: Request<'_>| format_markup_embedded(req, &sub_opts),
        )
        .ok();
    }
    if lang == MD_EMBED_LESS {
        return tidy::format(
            code.text,
            Language::Css(Dialect::Less),
            &sub_opts,
            None,
            &mut |req: Request<'_>| format_markup_embedded(req, &sub_opts),
        )
        .ok();
    }
    if lang == MD_EMBED_HTML {
        return tidy::format(
            code.text,
            Language::Html,
            &sub_opts,
            None,
            &mut |req: Request<'_>| format_markup_embedded(req, &sub_opts),
        )
        .ok();
    }
    if lang == MD_EMBED_VUE {
        return tidy::format(
            code.text,
            Language::Vue,
            &sub_opts,
            Some(MD_HINT_VUE),
            &mut |req: Request<'_>| format_markup_embedded(req, &sub_opts),
        )
        .ok();
    }
    if MD_EMBED_MARKDOWN.contains(&lang) && depth < MD_MAX_EMBED_DEPTH {
        return Some(format_markdown_inner(code.text, code.width, depth + 1));
    }
    None
}

/// Pre-pass rewriting triple emphasis (`***...***` and `___...___`) to `**_..._**` outside frontmatter, code blocks, thematic breaks, and inline code spans.
///
/// Preserves explicit `_**...**_` spans verbatim so `tidy::markdown` matches Prettier 3.7.4 across all four bold-italic delimiter combinations.
///
/// # Arguments
///
/// - `&str` - Raw Markdown input before `tidy::markdown`.
///
/// # Returns
///
/// - `String` - Pre-normalized Markdown ready for `tidy::markdown`.
fn normalize_markdown_triple_emphasis_prepass(input: &str) -> String {
    if !input.contains(MD_TRIPLE_STAR) && !input.contains(MD_TRIPLE_UNDER) {
        return input.to_string();
    }
    let has_trailing_lf: bool = input.ends_with(TEXT_LF);
    let raw_lines: Vec<&str> = input.split(TEXT_LF).collect();
    let line_count: usize = if has_trailing_lf && !raw_lines.is_empty() {
        raw_lines.len() - 1
    } else {
        raw_lines.len()
    };
    let mut out_lines: Vec<String> = Vec::with_capacity(line_count);
    let mut idx: usize = 0;
    if line_count >= 2 {
        let first_clean: &str = raw_lines[0].trim_end_matches(TEXT_CR);
        if (first_clean == MD_FRONTMATTER_YAML || first_clean == MD_FRONTMATTER_TOML)
            && let Some(close_rel) = raw_lines[1..line_count]
                .iter()
                .position(|l: &&str| l.trim_end_matches(TEXT_CR) == first_clean)
        {
            let close_idx: usize = close_rel + 1;
            for line in &raw_lines[..=close_idx] {
                out_lines.push((*line).to_string());
            }
            idx = close_idx + 1;
        }
    }
    let mut active_fence: Option<(char, usize, usize)> = None;
    let mut prev_bq_depth: usize = 0;
    let mut list_indent_stack: Vec<usize> = Vec::new();
    let mut prev_can_start_indented_code: bool = true;
    let mut indented_code_threshold: Option<usize> = None;
    while idx < line_count {
        let line: &str = raw_lines[idx];
        let clean: &str = line.trim_end_matches(TEXT_CR);
        let (bq_depth, _bq_end, indent_cols, stripped): (usize, usize, usize, &str) =
            strip_blockquote_and_measure_indent(clean);
        if let Some((fence_ch, fence_len, fence_base)) = active_fence {
            out_lines.push(line.to_string());
            if indent_cols < fence_base + 4 && is_closing_fence(stripped, fence_ch, fence_len) {
                active_fence = None;
                prev_can_start_indented_code = true;
            }
            idx += 1;
            continue;
        }
        if bq_depth != prev_bq_depth {
            list_indent_stack.clear();
            indented_code_threshold = None;
            prev_can_start_indented_code = true;
            prev_bq_depth = bq_depth;
        }
        if stripped.is_empty() {
            prev_can_start_indented_code = true;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        if let Some(threshold) = indented_code_threshold {
            if indent_cols >= threshold {
                prev_can_start_indented_code = true;
                out_lines.push(line.to_string());
                idx += 1;
                continue;
            }
            indented_code_threshold = None;
        }
        while list_indent_stack
            .last()
            .is_some_and(|&col: &usize| indent_cols < col)
        {
            list_indent_stack.pop();
        }
        let base_indent: usize = list_indent_stack.last().copied().unwrap_or(0);
        let rel_indent: usize = indent_cols.saturating_sub(base_indent);
        if rel_indent >= 4 && prev_can_start_indented_code {
            indented_code_threshold = Some(base_indent + 4);
            prev_can_start_indented_code = true;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        let (fence_base, fence_candidate): (usize, &str) = if rel_indent < 4
            && let Some((marker_cols, after)) = parse_list_marker(stripped)
        {
            let next_col: usize = indent_cols + marker_cols;
            list_indent_stack.push(next_col);
            (next_col, after)
        } else {
            (base_indent, stripped)
        };
        if rel_indent < 4
            && let Some((fence_ch, fence_len)) = parse_opening_fence(fence_candidate)
        {
            active_fence = Some((fence_ch, fence_len, fence_base));
            prev_can_start_indented_code = false;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        if rel_indent < 4 && is_thematic_break_line(stripped) {
            prev_can_start_indented_code = true;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        prev_can_start_indented_code = rel_indent < 4 && is_atx_heading_or_dash_break(stripped);
        let has_cr: bool = line.ends_with(TEXT_CR);
        let mut rewritten: String = rewrite_triple_emphasis_line(clean);
        if has_cr {
            rewritten.push(TEXT_CR);
        }
        out_lines.push(rewritten);
        idx += 1;
    }
    let mut out: String = out_lines.join("\n");
    if has_trailing_lf {
        out.push(TEXT_LF);
    }
    out
}

/// Returns `true` when `stripped` is a CommonMark thematic break made of `*` or `_`.
///
/// # Arguments
///
/// - `&str` - Line slice after container indentation and blockquote markers.
///
/// # Returns
///
/// - `bool` - Whether the line is a `*` or `_` thematic break.
fn is_thematic_break_line(stripped: &str) -> bool {
    let mut marker: Option<char> = None;
    let mut count: usize = 0;
    for ch in stripped.chars() {
        if ch == TEXT_SPACE || ch == TEXT_TAB {
            continue;
        }
        if ch != '*' && ch != '_' {
            return false;
        }
        if let Some(prev) = marker {
            if prev != ch {
                return false;
            }
        } else {
            marker = Some(ch);
        }
        count += 1;
    }
    count >= 3
}

/// Rewrites `***...***` and `___...___` to `**_..._**` within a single Markdown line outside escapes, code spans, autolinks, and link destinations.
///
/// # Arguments
///
/// - `&str` - Single non-code Markdown line without trailing CR.
///
/// # Returns
///
/// - `String` - Line with triple emphasis rewritten to `**_..._**`.
fn rewrite_triple_emphasis_line(line: &str) -> String {
    if !line.contains(MD_TRIPLE_STAR) && !line.contains(MD_TRIPLE_UNDER) {
        return line.to_string();
    }
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut out: String = String::with_capacity(len);
    let mut i: usize = 0;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            let next_len: usize = line[(i + 1)..]
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or(1);
            out.push_str(&line[i..(i + 1 + next_len)]);
            i += 1 + next_len;
            continue;
        }
        if bytes[i] == b'`' {
            let end_code: usize = scan_inline_code_span(bytes, i);
            out.push_str(&line[i..end_code]);
            i = end_code;
            continue;
        }
        if bytes[i] == b']'
            && i + 1 < len
            && bytes[i + 1] == b'('
            && let Some(end_link) = scan_inline_link_tail(bytes, i + 2)
        {
            out.push_str(&line[i..end_link]);
            i = end_link;
            continue;
        }
        if bytes[i] == b'<'
            && let Some(end_angle) = scan_autolink_or_tag(bytes, i)
        {
            out.push_str(&line[i..end_angle]);
            i = end_angle;
            continue;
        }
        if (bytes[i] == b'*' || bytes[i] == b'_')
            && i + 2 < len
            && bytes[i + 1] == bytes[i]
            && bytes[i + 2] == bytes[i]
        {
            let delim: u8 = bytes[i];
            let prev_not_delim: bool = i == 0 || bytes[i - 1] != delim;
            let next_not_delim: bool = i + 3 < len && bytes[i + 3] != delim;
            let next_not_ws: bool = i + 3 < len && !bytes[i + 3].is_ascii_whitespace();
            let under_boundary_ok: bool = delim != b'_'
                || i == 0
                || !line[..i]
                    .chars()
                    .next_back()
                    .is_some_and(char::is_alphanumeric);
            if prev_not_delim
                && next_not_delim
                && next_not_ws
                && under_boundary_ok
                && let Some(close_idx) = find_closing_triple_emphasis(line, i + 3, delim)
            {
                out.push_str(MD_PRETTIER_BOLD_ITALIC_OPEN);
                out.push_str(&line[(i + 3)..close_idx]);
                out.push_str(MD_PRETTIER_BOLD_ITALIC_CLOSE);
                i = close_idx + 3;
                continue;
            }
        }
        let ch_len: usize = line[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        out.push_str(&line[i..(i + ch_len)]);
        i += ch_len;
    }
    out
}

/// Scans past an inline link `(dest ...)` tail starting right after `](` so destinations are not rewritten.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Byte index immediately after `](`.
///
/// # Returns
///
/// - `Option<usize>` - Byte index right after the closing `)` if well-formed.
fn scan_inline_link_tail(bytes: &[u8], start: usize) -> Option<usize> {
    let len: usize = bytes.len();
    let mut i: usize = start;
    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    let after_dest: usize = scan_link_destination(bytes, i)?;
    let mut j: usize = after_dest;
    while j < len && (bytes[j] == b' ' || bytes[j] == b'\t') {
        j += 1;
    }
    if j < len && bytes[j] == b')' {
        return Some(j + 1);
    }
    if j > after_dest && j < len && (bytes[j] == b'"' || bytes[j] == b'\'' || bytes[j] == b'(') {
        let close_delim: u8 = if bytes[j] == b'(' { b')' } else { bytes[j] };
        j += 1;
        while j < len {
            if bytes[j] == b'\\' && j + 1 < len {
                j += 2;
                continue;
            }
            if bytes[j] == close_delim {
                j += 1;
                break;
            }
            j += 1;
        }
        while j < len && (bytes[j] == b' ' || bytes[j] == b'\t') {
            j += 1;
        }
        if j < len && bytes[j] == b')' {
            return Some(j + 1);
        }
    }
    None
}

/// Scans an angle-bracket autolink or raw HTML tag starting at `<` so inner characters are not rewritten.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Byte index of `<`.
///
/// # Returns
///
/// - `Option<usize>` - Byte index right after `>` when `<...>` is an autolink or tag.
fn scan_autolink_or_tag(bytes: &[u8], start: usize) -> Option<usize> {
    let len: usize = bytes.len();
    if start + 2 >= len {
        return None;
    }
    let next: u8 = bytes[start + 1];
    if !next.is_ascii_alphabetic() && next != b'/' && next != b'!' {
        return None;
    }
    let mut i: usize = start + 2;
    while i < len {
        if bytes[i] == b'>' {
            return Some(i + 1);
        }
        if bytes[i] == b'<' {
            return None;
        }
        i += 1;
    }
    None
}

/// Finds the matching closing `***` or `___` delimiter for a triple-emphasis span starting at `start`.
///
/// # Arguments
///
/// - `&str` - Full Markdown line.
/// - `usize` - Byte index right after the opening 3-byte delimiter.
/// - `u8` - Delimiter byte (`b'*'` or `b'_'`).
///
/// # Returns
///
/// - `Option<usize>` - Byte index of the matching closing 3-byte delimiter.
fn find_closing_triple_emphasis(line: &str, start: usize, delim: u8) -> Option<usize> {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = start;
    while i + 2 < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b'`' {
            i = scan_inline_code_span(bytes, i);
            continue;
        }
        if bytes[i] == delim && bytes[i + 1] == delim && bytes[i + 2] == delim {
            let prev_ok: bool =
                i > start && bytes[i - 1] != delim && !bytes[i - 1].is_ascii_whitespace();
            let next_not_delim: bool = i + 3 >= len || bytes[i + 3] != delim;
            let under_boundary_ok: bool = delim != b'_'
                || i + 3 >= len
                || !line[(i + 3)..]
                    .chars()
                    .next()
                    .is_some_and(char::is_alphanumeric);
            if prev_ok && next_not_delim && under_boundary_ok {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Applies minimal fence-safe and inline-code-safe Prettier 3.7.4 Markdown parity normalizations on top of `tidy::markdown`.
///
/// Inserts the single blank line Prettier emits inside empty fenced code blocks and converts double-quoted link/image/definition titles to Prettier `singleQuote: true` quoting.
///
/// # Arguments
///
/// - `&str` - Markdown output produced by `tidy::markdown`.
///
/// # Returns
///
/// - `String` - Prettier 3.7.4 aligned Markdown output.
fn normalize_markdown_prettier_quirks(input: &str) -> String {
    if !input.contains('"') && !input.contains(MD_FENCE_BACKTICK) && !input.contains(MD_FENCE_TILDE)
    {
        return input.to_string();
    }
    let has_trailing_lf: bool = input.ends_with(TEXT_LF);
    let raw_lines: Vec<&str> = input.split(TEXT_LF).collect();
    let line_count: usize = if has_trailing_lf && !raw_lines.is_empty() {
        raw_lines.len() - 1
    } else {
        raw_lines.len()
    };
    let mut out_lines: Vec<String> = Vec::with_capacity(line_count);
    let mut idx: usize = 0;
    if line_count >= 2
        && (raw_lines[0] == MD_FRONTMATTER_YAML || raw_lines[0] == MD_FRONTMATTER_TOML)
    {
        let fence: &str = raw_lines[0];
        if let Some(close_rel) = raw_lines[1..line_count]
            .iter()
            .position(|l: &&str| *l == fence)
        {
            let close_idx: usize = close_rel + 1;
            for line in &raw_lines[..=close_idx] {
                out_lines.push((*line).to_string());
            }
            idx = close_idx + 1;
        }
    }
    let mut active_fence: Option<(char, usize, usize)> = None;
    let mut fence_just_opened: bool = false;
    let mut prev_bq_depth: usize = 0;
    let mut list_indent_stack: Vec<usize> = Vec::new();
    let mut prev_can_start_indented_code: bool = true;
    let mut indented_code_threshold: Option<usize> = None;
    while idx < line_count {
        let line: &str = raw_lines[idx];
        let (bq_depth, bq_end, indent_cols, stripped): (usize, usize, usize, &str) =
            strip_blockquote_and_measure_indent(line);
        if let Some((fence_ch, fence_len, fence_base)) = active_fence {
            if indent_cols < fence_base + 4 && is_closing_fence(stripped, fence_ch, fence_len) {
                if fence_just_opened {
                    let blank_prefix: &str =
                        line[..bq_end].trim_end_matches([TEXT_SPACE, TEXT_TAB]);
                    out_lines.push(blank_prefix.to_string());
                }
                active_fence = None;
                fence_just_opened = false;
                prev_can_start_indented_code = true;
            } else {
                fence_just_opened = false;
            }
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        if bq_depth != prev_bq_depth {
            list_indent_stack.clear();
            indented_code_threshold = None;
            prev_can_start_indented_code = true;
            prev_bq_depth = bq_depth;
        }
        if stripped.is_empty() {
            prev_can_start_indented_code = true;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        if let Some(threshold) = indented_code_threshold {
            if indent_cols >= threshold {
                prev_can_start_indented_code = true;
                out_lines.push(line.to_string());
                idx += 1;
                continue;
            }
            indented_code_threshold = None;
        }
        while list_indent_stack
            .last()
            .is_some_and(|&col: &usize| indent_cols < col)
        {
            list_indent_stack.pop();
        }
        let base_indent: usize = list_indent_stack.last().copied().unwrap_or(0);
        let rel_indent: usize = indent_cols.saturating_sub(base_indent);
        if rel_indent >= 4 && prev_can_start_indented_code {
            indented_code_threshold = Some(base_indent + 4);
            prev_can_start_indented_code = true;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        let (fence_base, fence_candidate): (usize, &str) = if rel_indent < 4
            && let Some((marker_cols, after)) = parse_list_marker(stripped)
        {
            let next_col: usize = indent_cols + marker_cols;
            list_indent_stack.push(next_col);
            (next_col, after)
        } else {
            (base_indent, stripped)
        };
        if rel_indent < 4
            && let Some((fence_ch, fence_len)) = parse_opening_fence(fence_candidate)
        {
            active_fence = Some((fence_ch, fence_len, fence_base));
            fence_just_opened = true;
            prev_can_start_indented_code = false;
            out_lines.push(line.to_string());
            idx += 1;
            continue;
        }
        prev_can_start_indented_code = rel_indent < 4
            && (is_thematic_break_line(stripped) || is_atx_heading_or_dash_break(stripped));
        let step_def: String = normalize_link_def_title(line);
        let step_inline: String = normalize_inline_prettier_line(&step_def);
        out_lines.push(step_inline);
        idx += 1;
    }
    let mut out: String = out_lines.join("\n");
    if has_trailing_lf {
        out.push(TEXT_LF);
    }
    out
}

/// Strips CommonMark blockquote markers (`>` with 0..=3 leading spaces and optional 1-column space) and measures remaining column indentation.
///
/// # Arguments
///
/// - `&str` - Raw Markdown line without trailing CR.
///
/// # Returns
///
/// - `(usize, usize, usize, &str)` - Blockquote depth, byte offset right after the last `>`, remaining column indentation (tab stop = 4), and content slice.
fn strip_blockquote_and_measure_indent(line: &str) -> (usize, usize, usize, &str) {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut idx: usize = 0;
    let mut bq_depth: usize = 0;
    let mut bq_end: usize = 0;
    loop {
        let mut scan: usize = idx;
        let mut ws_cols: usize = 0;
        while scan < len {
            if bytes[scan] == b' ' {
                ws_cols += 1;
                scan += 1;
            } else if bytes[scan] == b'\t' {
                ws_cols += 4 - (ws_cols % 4);
                scan += 1;
            } else {
                break;
            }
        }
        if ws_cols < 4 && scan < len && bytes[scan] == b'>' {
            bq_depth += 1;
            scan += 1;
            bq_end = scan;
            if scan < len && bytes[scan] == b' ' {
                scan += 1;
            }
            idx = scan;
            continue;
        }
        return (bq_depth, bq_end, ws_cols, &line[scan..]);
    }
}

/// Parses a CommonMark unordered (`-`, `*`, `+`) or ordered (`1.`, `1)`) list item marker at the start of `stripped`.
///
/// # Arguments
///
/// - `&str` - Line content after container indentation and blockquote markers.
///
/// # Returns
///
/// - `Option<(usize, &str)>` - Marker content column width and remaining text on the line when `stripped` starts a list item.
fn parse_list_marker(stripped: &str) -> Option<(usize, &str)> {
    let bytes: &[u8] = stripped.as_bytes();
    let len: usize = bytes.len();
    if len == 0 {
        return None;
    }
    let marker_len: usize = if bytes[0] == b'-' || bytes[0] == b'*' || bytes[0] == b'+' {
        1
    } else if bytes[0].is_ascii_digit() {
        let digits: usize = bytes
            .iter()
            .take(9)
            .take_while(|&&b: &&u8| b.is_ascii_digit())
            .count();
        if digits == 0 || digits >= len || (bytes[digits] != b'.' && bytes[digits] != b')') {
            return None;
        }
        digits + 1
    } else {
        return None;
    };
    if marker_len == len {
        return Some((marker_len + 1, ""));
    }
    if bytes[marker_len] != b' ' && bytes[marker_len] != b'\t' {
        return None;
    }
    let mut scan: usize = marker_len;
    let mut ws_cols: usize = 0;
    while scan < len {
        if bytes[scan] == b' ' {
            ws_cols += 1;
            scan += 1;
        } else if bytes[scan] == b'\t' {
            ws_cols += 4 - ((marker_len + ws_cols) % 4);
            scan += 1;
        } else {
            break;
        }
    }
    if scan == len || ws_cols >= 5 {
        return Some((marker_len + 1, ""));
    }
    Some((marker_len + ws_cols, &stripped[scan..]))
}

/// Returns `true` when `stripped` is an ATX heading (`#`..`######`) or a `-` thematic break that allows an indented code block on the next line.
///
/// # Arguments
///
/// - `&str` - Line slice after container indentation and blockquote markers.
///
/// # Returns
///
/// - `bool` - Whether the line terminates any paragraph and permits a following indented code block.
fn is_atx_heading_or_dash_break(stripped: &str) -> bool {
    let hashes: usize = stripped.chars().take_while(|c: &char| *c == '#').count();
    if (1..=6).contains(&hashes) {
        let rest: &str = &stripped[hashes..];
        if rest.is_empty() || rest.starts_with(TEXT_SPACE) || rest.starts_with(TEXT_TAB) {
            return true;
        }
    }
    let mut dashes: usize = 0;
    for ch in stripped.chars() {
        if ch == TEXT_SPACE || ch == TEXT_TAB {
            continue;
        }
        if ch != '-' {
            return false;
        }
        dashes += 1;
    }
    dashes >= 3
}

/// Parses an opening fenced code block delimiter (`` ``` `` or `~~~`) on an unquoted line slice.
///
/// # Arguments
///
/// - `&str` - Line slice after container indentation and blockquote markers.
///
/// # Returns
///
/// - `Option<(char, usize)>` - Fence character and delimiter length if an opening fence starts here.
fn parse_opening_fence(stripped: &str) -> Option<(char, usize)> {
    let first: char = stripped.chars().next()?;
    if first != '`' && first != '~' {
        return None;
    }
    let len: usize = stripped.chars().take_while(|c: &char| *c == first).count();
    if len < 3 {
        return None;
    }
    let info: &str = &stripped[len..];
    if first == '`' && info.contains('`') {
        return None;
    }
    Some((first, len))
}

/// Returns `true` when `stripped` is a valid closing fence matching `(fence_ch, fence_len)`.
///
/// # Arguments
///
/// - `&str` - Line slice after container indentation and blockquote markers.
/// - `char` - Expected fence character (`` ` `` or `~`).
/// - `usize` - Minimum closing fence length.
///
/// # Returns
///
/// - `bool` - Whether this line closes the active fenced code block.
fn is_closing_fence(stripped: &str, fence_ch: char, fence_len: usize) -> bool {
    let trimmed: &str = stripped.trim_end_matches([TEXT_SPACE, TEXT_TAB]);
    let count: usize = trimmed
        .chars()
        .take_while(|c: &char| *c == fence_ch)
        .count();
    count >= fence_len && count == trimmed.len()
}

/// Converts a double-quoted title on a CommonMark link reference definition (`[label]: dest "title"`) to single quotes under `singleQuote: true`.
///
/// # Arguments
///
/// - `&str` - Non-code Markdown line.
///
/// # Returns
///
/// - `String` - Normalized line with single-quoted definition title when safe.
fn normalize_link_def_title(line: &str) -> String {
    let (_bq_depth, _bq_end, _indent_cols, trimmed): (usize, usize, usize, &str) =
        strip_blockquote_and_measure_indent(line);
    if !trimmed.starts_with('[') || trimmed.starts_with("[^") {
        return line.to_string();
    }
    let prefix_len: usize = line.len() - trimmed.len();
    let bytes: &[u8] = trimmed.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 1;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b']' {
            break;
        }
        i += 1;
    }
    if i + 1 >= len || bytes[i] != b']' || bytes[i + 1] != b':' {
        return line.to_string();
    }
    i += 2;
    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    let Some(after_dest) = scan_link_destination(bytes, i) else {
        return line.to_string();
    };
    let mut ws: usize = after_dest;
    while ws < len && (bytes[ws] == b' ' || bytes[ws] == b'\t') {
        ws += 1;
    }
    if ws == after_dest || ws >= len || bytes[ws] != b'"' {
        return line.to_string();
    }
    let Some(close_quote) = scan_double_quoted_title(bytes, ws) else {
        return line.to_string();
    };
    if !trimmed[(close_quote + 1)..]
        .trim_matches([TEXT_SPACE, TEXT_TAB])
        .is_empty()
    {
        return line.to_string();
    }
    let inner: &str = &trimmed[(ws + 1)..close_quote];
    let Some((open_ch, converted, close_ch)) = convert_double_title_to_prettier(inner) else {
        return line.to_string();
    };
    let mut out: String = String::with_capacity(line.len());
    out.push_str(&line[..(prefix_len + ws)]);
    out.push(open_ch);
    out.push_str(&converted);
    out.push(close_ch);
    out.push_str(&trimmed[(close_quote + 1)..]);
    out
}

/// Normalizes inline link/image `"title"` quotes outside inline code spans to match Prettier `singleQuote: true`.
///
/// # Arguments
///
/// - `&str` - Single non-code Markdown line.
///
/// # Returns
///
/// - `String` - Normalized Markdown line.
fn normalize_inline_prettier_line(line: &str) -> String {
    if !line.contains('"') {
        return line.to_string();
    }
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut out: String = String::with_capacity(len);
    let mut i: usize = 0;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            let next_len: usize = line[(i + 1)..]
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or(1);
            out.push_str(&line[i..(i + 1 + next_len)]);
            i += 1 + next_len;
            continue;
        }
        if bytes[i] == b'`' {
            let end_code: usize = scan_inline_code_span(bytes, i);
            out.push_str(&line[i..end_code]);
            i = end_code;
            continue;
        }
        if bytes[i] == b']'
            && i + 1 < len
            && bytes[i + 1] == b'('
            && let Some((replacement, next_i)) = try_rewrite_inline_link_title(line, i)
        {
            out.push_str(&replacement);
            i = next_i;
            continue;
        }
        let ch_len: usize = line[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        out.push_str(&line[i..(i + ch_len)]);
        i += ch_len;
    }
    out
}

/// Scans past a CommonMark multi-backtick inline code span starting at `start`.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Byte offset of the opening backtick.
///
/// # Returns
///
/// - `usize` - Byte offset immediately after the closing backtick run (or opening run if unclosed).
fn scan_inline_code_span(bytes: &[u8], start: usize) -> usize {
    let len: usize = bytes.len();
    let ticks: usize = bytes[start..]
        .iter()
        .take_while(|&&b: &&u8| b == b'`')
        .count();
    let mut i: usize = start + ticks;
    while i < len {
        if bytes[i] == b'`' {
            let run: usize = bytes[i..].iter().take_while(|&&b: &&u8| b == b'`').count();
            if run == ticks {
                return i + run;
            }
            i += run;
            continue;
        }
        i += 1;
    }
    start + ticks
}

/// Attempts to rewrite an inline link or image `](dest "title")` title to Prettier `singleQuote: true` quoting.
///
/// # Arguments
///
/// - `&str` - Full line string.
/// - `usize` - Byte offset of `]` immediately preceding `(`.
///
/// # Returns
///
/// - `Option<(String, usize)>` - Rewritten `](...)` slice and byte index after closing `)`, if rewritten.
fn try_rewrite_inline_link_title(line: &str, bracket_idx: usize) -> Option<(String, usize)> {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = bracket_idx + 2;
    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    let after_dest: usize = scan_link_destination(bytes, i)?;
    let mut ws: usize = after_dest;
    while ws < len && (bytes[ws] == b' ' || bytes[ws] == b'\t') {
        ws += 1;
    }
    if ws == after_dest || ws >= len || bytes[ws] != b'"' {
        return None;
    }
    let close_quote: usize = scan_double_quoted_title(bytes, ws)?;
    let mut tail: usize = close_quote + 1;
    while tail < len && (bytes[tail] == b' ' || bytes[tail] == b'\t') {
        tail += 1;
    }
    if tail >= len || bytes[tail] != b')' {
        return None;
    }
    let inner: &str = &line[(ws + 1)..close_quote];
    let (open_ch, converted, close_ch): (char, String, char) =
        convert_double_title_to_prettier(inner)?;
    let mut out: String = String::with_capacity(tail + 1 - bracket_idx);
    out.push_str(&line[bracket_idx..ws]);
    out.push(open_ch);
    out.push_str(&converted);
    out.push(close_ch);
    out.push_str(&line[(close_quote + 1)..=tail]);
    Some((out, tail + 1))
}

/// Scans a CommonMark link destination (`<...>` or balanced-paren non-whitespace token) starting at `start`.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Starting byte offset of the destination.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset immediately after the link destination.
fn scan_link_destination(bytes: &[u8], start: usize) -> Option<usize> {
    let len: usize = bytes.len();
    if start >= len {
        return None;
    }
    if bytes[start] == b'<' {
        let mut i: usize = start + 1;
        while i < len {
            if bytes[i] == b'\\' && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == b'>' {
                return Some(i + 1);
            }
            if bytes[i] == b'<' {
                return None;
            }
            i += 1;
        }
        return None;
    }
    let mut i: usize = start;
    let mut paren_depth: usize = 0;
    while i < len {
        let b: u8 = bytes[i];
        if b == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if b == b' ' || b == b'\t' {
            break;
        }
        if b == b'(' {
            paren_depth += 1;
        } else if b == b')' {
            if paren_depth == 0 {
                break;
            }
            paren_depth -= 1;
        }
        i += 1;
    }
    if i == start || paren_depth != 0 {
        return None;
    }
    Some(i)
}

/// Scans a double-quoted Markdown link title starting at `open_quote`.
///
/// # Arguments
///
/// - `&[u8]` - Line bytes.
/// - `usize` - Byte offset of opening `"`.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of matching closing `"`.
fn scan_double_quoted_title(bytes: &[u8], open_quote: usize) -> Option<usize> {
    let len: usize = bytes.len();
    let mut i: usize = open_quote + 1;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Converts the interior of a double-quoted Markdown title to Prettier `singleQuote: true` delimiters.
///
/// Uses single quotes `'...'` whenever no `'` appears (unescaping `\"` to `"`), or parentheses `(...)` when both `'` and `\"` appear without parentheses; otherwise preserves `"..."`.
///
/// # Arguments
///
/// - `&str` - Raw inner slice between `"` delimiters.
///
/// # Returns
///
/// - `Option<(char, String, char)>` - Opening delimiter, normalized title content, and closing delimiter, or `None` to keep double quotes.
fn convert_double_title_to_prettier(inner: &str) -> Option<(char, String, char)> {
    let has_single: bool = inner.contains('\'');
    let has_escaped_double: bool = inner.contains('"');
    let (open_ch, close_ch): (char, char) = if !has_single {
        ('\'', '\'')
    } else if has_escaped_double && !inner.contains('(') && !inner.contains(')') {
        ('(', ')')
    } else {
        return None;
    };
    let bytes: &[u8] = inner.as_bytes();
    let len: usize = bytes.len();
    let mut out: String = String::with_capacity(len);
    let mut i: usize = 0;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            if bytes[i + 1] == b'"' {
                out.push('"');
            } else {
                out.push('\\');
                let next_len: usize = inner[(i + 1)..]
                    .chars()
                    .next()
                    .map(char::len_utf8)
                    .unwrap_or(1);
                out.push_str(&inner[(i + 1)..(i + 1 + next_len)]);
                i += 1 + next_len;
                continue;
            }
            i += 2;
            continue;
        }
        let ch_len: usize = inner[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        out.push_str(&inner[i..(i + ch_len)]);
        i += ch_len;
    }
    Some((open_ch, out, close_ch))
}
