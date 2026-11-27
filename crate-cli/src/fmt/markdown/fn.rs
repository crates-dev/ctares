use super::*;

/// Formats Markdown (`.md` / `.markdown`) matching VS Code `esbenp.prettier-vscode` (`tabWidth: 2`, `singleQuote: true`, `proseWrap: "preserve"`, `embeddedLanguageFormatting: "auto"`, LF).
///
/// # Arguments
///
/// - `&str` - Raw Markdown source text.
///
/// # Returns
///
/// - `String` - Formatted Markdown document ending with a single `\n`.
pub fn format_markdown(input: &str) -> String {
    let normalized: String = normalize_line_endings(input);
    let raw_lines: Vec<&str> = normalized.split(TEXT_LF).collect();
    let total: usize = raw_lines.len();
    let mut out_lines: Vec<String> = Vec::new();
    let mut idx: usize = 0;
    let mut prev_block: MdBlockKind = MdBlockKind::None;
    let mut pending_blank: bool = false;
    let mut list_indents: Vec<usize> = Vec::new();
    if let Some((fm_lines, next_idx)) = try_parse_frontmatter(&raw_lines) {
        out_lines.extend(fm_lines);
        idx = next_idx;
        prev_block = MdBlockKind::Frontmatter;
    }
    while idx < total {
        let raw_line: &str = raw_lines[idx];
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() {
            if !out_lines.is_empty() {
                pending_blank = true;
            }
            idx += 1;
            continue;
        }
        if let Some(fence) = parse_fence_open(raw_line) {
            let fence_indent: usize = fence.get_indent();
            let in_list: bool = matches!(prev_block, MdBlockKind::List(_)) && fence_indent > 0;
            let norm_indent: usize = if in_list {
                normalize_nested_indent(&list_indents, fence_indent)
            } else {
                list_indents.clear();
                0
            };
            let needs_sep: bool = pending_blank || !in_list;
            if !out_lines.is_empty() && (needs_sep || prev_block != MdBlockKind::None) {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            let (block_lines, next_idx): (Vec<String>, usize) =
                format_fenced_code_block(&raw_lines, idx, &fence, norm_indent);
            out_lines.extend(block_lines);
            idx = next_idx;
            if !in_list {
                prev_block = MdBlockKind::CodeFence;
            }
            continue;
        }
        if let Some((table_lines, next_idx)) = try_parse_gfm_table(&raw_lines, idx) {
            list_indents.clear();
            if !out_lines.is_empty() {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            out_lines.extend(table_lines);
            idx = next_idx;
            prev_block = MdBlockKind::Table;
            continue;
        }
        if let Some(heading) = format_atx_heading(trimmed) {
            list_indents.clear();
            if !out_lines.is_empty() {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            out_lines.push(heading);
            idx += 1;
            prev_block = MdBlockKind::Heading;
            continue;
        }
        if is_thematic_break(trimmed) {
            list_indents.clear();
            if !out_lines.is_empty() {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            out_lines.push(MD_THEMATIC_BREAK.to_string());
            idx += 1;
            prev_block = MdBlockKind::ThematicBreak;
            continue;
        }
        if trimmed.starts_with('>') {
            list_indents.clear();
            if !out_lines.is_empty() && (pending_blank || prev_block != MdBlockKind::Blockquote) {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            out_lines.push(format_blockquote_line(trimmed));
            idx += 1;
            prev_block = MdBlockKind::Blockquote;
            continue;
        }
        if let Some(item) = parse_list_item(raw_line) {
            let cur_kind: ListKind = item.get_kind();
            let switching_kind: bool = match prev_block {
                MdBlockKind::List(prev_kind) => prev_kind != cur_kind && item.get_raw_indent() == 0,
                MdBlockKind::None => false,
                _ => true,
            };
            if !out_lines.is_empty() && (pending_blank || switching_kind) {
                push_single_blank(&mut out_lines);
                if switching_kind {
                    list_indents.clear();
                }
            }
            pending_blank = false;
            let out_indent: usize =
                resolve_list_item_indent(&mut list_indents, item.get_raw_indent());
            let mut formatted: String = String::with_capacity(
                out_indent + item.get_marker().len() + 1 + item.get_body().len(),
            );
            push_spaces(&mut formatted, out_indent);
            formatted.push_str(item.get_marker());
            if !item.get_body().is_empty() {
                formatted.push(TEXT_SPACE);
                formatted.push_str(item.get_body());
            }
            out_lines.push(formatted);
            idx += 1;
            prev_block = MdBlockKind::List(cur_kind);
            continue;
        }
        if is_html_block_line(trimmed) {
            list_indents.clear();
            let needs_blank: bool =
                !out_lines.is_empty() && (pending_blank || prev_block != MdBlockKind::Html);
            if needs_blank {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            out_lines.push(
                raw_line
                    .trim_end_matches([TEXT_SPACE, TEXT_TAB])
                    .to_string(),
            );
            idx += 1;
            prev_block = MdBlockKind::Html;
            continue;
        }
        let raw_indent: usize = count_leading_spaces_md(raw_line);
        let list_continuation: bool = matches!(prev_block, MdBlockKind::List(_)) && raw_indent >= 2;
        if list_continuation {
            if pending_blank {
                push_single_blank(&mut out_lines);
            }
            pending_blank = false;
            let cont_indent: usize = normalize_nested_indent(&list_indents, raw_indent);
            let mut line: String = String::with_capacity(cont_indent + trimmed.len());
            push_spaces(&mut line, cont_indent);
            line.push_str(trimmed);
            out_lines.push(line);
            idx += 1;
            continue;
        }
        list_indents.clear();
        let needs_blank: bool =
            !out_lines.is_empty() && (pending_blank || prev_block != MdBlockKind::Paragraph);
        if needs_blank {
            push_single_blank(&mut out_lines);
        }
        pending_blank = false;
        out_lines.push(normalize_inline_emphasis(
            raw_line.trim_end_matches([TEXT_SPACE, TEXT_TAB]),
        ));
        idx += 1;
        prev_block = MdBlockKind::Paragraph;
    }
    while let Some(last) = out_lines.last() {
        if last.is_empty() {
            let _: Option<String> = out_lines.pop();
        } else {
            break;
        }
    }
    if out_lines.is_empty() {
        return String::new();
    }
    let mut out: String = out_lines.join("\n");
    out.push(TEXT_LF);
    out
}

/// Ensures the output line vector ends with at most one blank line.
///
/// # Arguments
///
/// - `&mut Vec<String>` - Output line buffer.
fn push_single_blank(out_lines: &mut Vec<String>) {
    if let Some(last) = out_lines.last()
        && !last.is_empty()
    {
        out_lines.push(String::new());
    }
}

/// Parses and formats a leading YAML frontmatter block (`---\n...\n---`) if present at line 0.
///
/// # Arguments
///
/// - `&[&str]` - All lines of the Markdown document.
///
/// # Returns
///
/// - `Option<(Vec<String>, usize)>` - Formatted frontmatter lines and the next line index after the closing `---`.
fn try_parse_frontmatter(raw_lines: &[&str]) -> Option<(Vec<String>, usize)> {
    if raw_lines.len() < 2 || raw_lines[0].trim() != MD_FRONTMATTER_FENCE {
        return None;
    }
    let mut close_idx: Option<usize> = None;
    for (i, line) in raw_lines.iter().enumerate().skip(1) {
        if line.trim() == MD_FRONTMATTER_FENCE {
            close_idx = Some(i);
            break;
        }
    }
    let end_idx: usize = close_idx?;
    let yaml_src: String = raw_lines[1..end_idx].join("\n");
    let formatted_yaml: String = format_yaml(&yaml_src);
    let mut out: Vec<String> = Vec::new();
    out.push(MD_FRONTMATTER_FENCE.to_string());
    if !formatted_yaml.is_empty() {
        for line in formatted_yaml.trim_end_matches(TEXT_LF).split(TEXT_LF) {
            out.push(line.to_string());
        }
    }
    out.push(MD_FRONTMATTER_FENCE.to_string());
    Some((out, end_idx + 1))
}

/// Attempts to parse an opening fenced code block line (` ``` ` or ` ~~~ `).
///
/// # Arguments
///
/// - `&str` - Raw Markdown line.
///
/// # Returns
///
/// - `Option<FenceOpen>` - Parsed opening fence metadata if matched.
fn parse_fence_open(raw_line: &str) -> Option<FenceOpen> {
    let indent: usize = count_leading_spaces_md(raw_line);
    let trimmed: &str = raw_line.trim_start_matches([TEXT_SPACE, TEXT_TAB]);
    let first: char = trimmed.chars().next()?;
    if first != '`' && first != '~' {
        return None;
    }
    let fence_len: usize = trimmed.chars().take_while(|c: &char| *c == first).count();
    if fence_len < 3 {
        return None;
    }
    let rest: &str = trimmed[fence_len..].trim();
    if first == '`' && rest.contains('`') {
        return None;
    }
    Some(FenceOpen::new(indent, first, fence_len, rest.to_string()))
}

/// Formats a fenced code block including Prettier embedded language formatting for `json`, `jsonc`, `yaml`, and `yml`.
///
/// # Arguments
///
/// - `&[&str]` - All lines of the document.
/// - `usize` - Line index of the opening fence.
/// - `&FenceOpen` - Parsed opening fence metadata.
/// - `usize` - Target normalized leading indentation in spaces.
///
/// # Returns
///
/// - `(Vec<String>, usize)` - Formatted code block lines and the next line index after the closing fence.
fn format_fenced_code_block(
    raw_lines: &[&str],
    start_idx: usize,
    fence: &FenceOpen,
    target_indent: usize,
) -> (Vec<String>, usize) {
    let mut body_lines: Vec<String> = Vec::new();
    let mut cursor: usize = start_idx + 1;
    let raw_indent: usize = fence.get_indent();
    let fence_char: char = fence.get_fence_char();
    let fence_len: usize = fence.get_fence_len();
    while cursor < raw_lines.len() {
        let candidate: &str = raw_lines[cursor].trim();
        let close_count: usize = candidate
            .chars()
            .take_while(|c: &char| *c == fence_char)
            .count();
        if close_count >= fence_len && candidate.len() == close_count {
            cursor += 1;
            break;
        }
        body_lines.push(strip_up_to_spaces(raw_lines[cursor], raw_indent).to_string());
        cursor += 1;
    }
    let lang: String = fence
        .get_info()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let body_joined: String = body_lines.join("\n");
    let formatted_body: Option<String> = match lang.as_str() {
        MD_LANG_JSON => format_json(&body_joined).ok(),
        MD_LANG_JSONC => Some(format_jsonc(&body_joined)),
        MD_LANG_YAML | MD_LANG_YML => Some(format_yaml(&body_joined)),
        _ => None,
    };
    let final_lines: Vec<String> = if let Some(ref embedded) = formatted_body {
        if embedded.is_empty() {
            Vec::new()
        } else {
            embedded
                .trim_end_matches(TEXT_LF)
                .split(TEXT_LF)
                .map(|s: &str| s.to_string())
                .collect()
        }
    } else {
        body_lines
    };
    let out_fence_len: usize = compute_safe_backtick_fence(&final_lines, fence_len);
    let mut fence_marker: String = String::with_capacity(out_fence_len);
    for _ in 0..out_fence_len {
        fence_marker.push('`');
    }
    let mut out: Vec<String> = Vec::with_capacity(final_lines.len() + 2);
    let mut open_line: String =
        String::with_capacity(target_indent + fence_marker.len() + fence.get_info().len());
    push_spaces(&mut open_line, target_indent);
    open_line.push_str(&fence_marker);
    open_line.push_str(fence.get_info());
    out.push(open_line);
    for line in final_lines {
        if line.is_empty() {
            out.push(String::new());
        } else {
            let mut indented: String = String::with_capacity(target_indent + line.len());
            push_spaces(&mut indented, target_indent);
            indented.push_str(&line);
            out.push(indented);
        }
    }
    let mut close_line: String = String::with_capacity(target_indent + fence_marker.len());
    push_spaces(&mut close_line, target_indent);
    close_line.push_str(&fence_marker);
    out.push(close_line);
    (out, cursor)
}

/// Computes the minimum backtick fence length that will not collide with any backtick run inside `body_lines`.
///
/// # Arguments
///
/// - `&[String]` - Formatted code block lines.
/// - `usize` - Original opening fence length.
///
/// # Returns
///
/// - `usize` - Safe backtick fence length (`>= 3`).
fn compute_safe_backtick_fence(body_lines: &[String], original_len: usize) -> usize {
    let mut max_run: usize = 0;
    for line in body_lines {
        let mut run: usize = 0;
        for ch in line.chars() {
            if ch == '`' {
                run += 1;
                if run > max_run {
                    max_run = run;
                }
            } else {
                run = 0;
            }
        }
    }
    let needed: usize = if max_run >= 3 { max_run + 1 } else { 3 };
    needed.max(original_len.clamp(3, 4))
}

/// Strips up to `max_spaces` leading ASCII spaces (or equivalent tabs) from a line.
///
/// # Arguments
///
/// - `&str` - Raw line.
/// - `usize` - Maximum number of leading spaces to remove.
///
/// # Returns
///
/// - `&str` - Sliced line with leading indentation removed.
fn strip_up_to_spaces(line: &str, max_spaces: usize) -> &str {
    let bytes: &[u8] = line.as_bytes();
    let mut i: usize = 0;
    let mut removed: usize = 0;
    while i < bytes.len() && removed < max_spaces {
        if bytes[i] == b' ' {
            i += 1;
            removed += 1;
        } else if bytes[i] == b'\t' {
            i += 1;
            removed += PRETTIER_TAB_WIDTH;
        } else {
            break;
        }
    }
    &line[i..]
}

/// Attempts to parse and format a GFM pipe table starting at `start_idx` using exact Prettier column alignment.
///
/// # Arguments
///
/// - `&[&str]` - All lines of the document.
/// - `usize` - Candidate header row index.
///
/// # Returns
///
/// - `Option<(Vec<String>, usize)>` - Formatted table lines and next line index if a valid GFM table starts here.
fn try_parse_gfm_table(raw_lines: &[&str], start_idx: usize) -> Option<(Vec<String>, usize)> {
    if start_idx + 1 >= raw_lines.len() {
        return None;
    }
    let header_raw: &str = raw_lines[start_idx].trim();
    let sep_raw: &str = raw_lines[start_idx + 1].trim();
    if !header_raw.contains('|') || !sep_raw.contains('|') {
        return None;
    }
    let alignments: Vec<TableAlign> = parse_table_separator(sep_raw)?;
    let header_cells: Vec<String> = split_table_row(header_raw);
    if header_cells.is_empty() || header_cells.len() != alignments.len() {
        return None;
    }
    let col_count: usize = alignments.len();
    let mut rows: Vec<Vec<String>> = vec![header_cells];
    let mut cursor: usize = start_idx + 2;
    while cursor < raw_lines.len() {
        let row_trimmed: &str = raw_lines[cursor].trim();
        if row_trimmed.is_empty() || !row_trimmed.contains('|') {
            break;
        }
        let mut cells: Vec<String> = split_table_row(row_trimmed);
        while cells.len() < col_count {
            cells.push(String::new());
        }
        cells.truncate(col_count);
        rows.push(cells);
        cursor += 1;
    }
    let mut col_widths: Vec<usize> = vec![MD_TABLE_MIN_COL_WIDTH; col_count];
    for row in &rows {
        for (c, cell) in row.iter().enumerate() {
            let w: usize = str_display_width(cell);
            if w > col_widths[c] {
                col_widths[c] = w;
            }
        }
    }
    let mut out: Vec<String> = Vec::with_capacity(rows.len() + 1);
    out.push(format_table_data_row(&rows[0], &col_widths, &alignments));
    out.push(format_table_sep_row(&col_widths, &alignments));
    for row in rows.iter().skip(1) {
        out.push(format_table_data_row(row, &col_widths, &alignments));
    }
    Some((out, cursor))
}

/// Parses a GFM table delimiter row into per-column alignments.
///
/// # Arguments
///
/// - `&str` - Trimmed separator row line.
///
/// # Returns
///
/// - `Option<Vec<TableAlign>>` - Column alignments if every cell is a valid GFM delimiter cell.
fn parse_table_separator(line: &str) -> Option<Vec<TableAlign>> {
    let cells: Vec<String> = split_table_row(line);
    if cells.is_empty() {
        return None;
    }
    let mut aligns: Vec<TableAlign> = Vec::with_capacity(cells.len());
    for cell in cells {
        let trimmed: &str = cell.trim();
        if trimmed.is_empty() {
            return None;
        }
        let left: bool = trimmed.starts_with(':');
        let right: bool = trimmed.ends_with(':');
        let inner: &str = trimmed.trim_matches(':');
        if inner.is_empty() || !inner.chars().all(|c: char| c == '-') {
            return None;
        }
        let align: TableAlign = match (left, right) {
            (true, true) => TableAlign::Center,
            (true, false) => TableAlign::Left,
            (false, true) => TableAlign::Right,
            (false, false) => TableAlign::None,
        };
        aligns.push(align);
    }
    Some(aligns)
}

/// Splits a GFM table row into trimmed cells, respecting escaped pipes `\|` and inline backtick code spans.
///
/// # Arguments
///
/// - `&str` - Trimmed GFM table row.
///
/// # Returns
///
/// - `Vec<String>` - Trimmed cell contents.
fn split_table_row(row: &str) -> Vec<String> {
    let mut inner: &str = row.trim();
    if let Some(stripped) = inner.strip_prefix('|') {
        inner = stripped;
    }
    if inner.ends_with('|') && !ends_with_escaped_pipe(inner) {
        inner = &inner[..(inner.len() - 1)];
    }
    let bytes: &[u8] = inner.as_bytes();
    let len: usize = bytes.len();
    let mut cells: Vec<String> = Vec::new();
    let mut start: usize = 0;
    let mut i: usize = 0;
    let mut in_code: bool = false;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b'`' {
            in_code = !in_code;
            i += 1;
            continue;
        }
        if bytes[i] == b'|' && !in_code {
            cells.push(inner[start..i].trim().to_string());
            start = i + 1;
        }
        i += 1;
    }
    cells.push(inner[start..].trim().to_string());
    cells
}

/// Returns `true` when `s` ends with an escaped pipe (`\|`) preceded by an odd number of backslashes.
///
/// # Arguments
///
/// - `&str` - Table row slice ending in `|`.
///
/// # Returns
///
/// - `bool` - Whether the trailing `|` is escaped.
fn ends_with_escaped_pipe(s: &str) -> bool {
    let bytes: &[u8] = s.as_bytes();
    if bytes.len() < 2 || bytes[bytes.len() - 1] != b'|' {
        return false;
    }
    let mut backslashes: usize = 0;
    let mut idx: usize = bytes.len() - 1;
    while idx > 0 && bytes[idx - 1] == b'\\' {
        backslashes += 1;
        idx -= 1;
    }
    backslashes % 2 == 1
}

/// Formats a single GFM table header or body row padded to Prettier column widths and alignments.
///
/// # Arguments
///
/// - `&[String]` - Row cells.
/// - `&[usize]` - Column display widths.
/// - `&[TableAlign]` - Column alignments.
///
/// # Returns
///
/// - `String` - Formatted pipe-delimited row.
fn format_table_data_row(cells: &[String], widths: &[usize], alignments: &[TableAlign]) -> String {
    let mut out: String = String::from("|");
    for (idx, cell) in cells.iter().enumerate() {
        let col_w: usize = widths[idx];
        let cell_w: usize = str_display_width(cell);
        let pad: usize = col_w.saturating_sub(cell_w);
        out.push(TEXT_SPACE);
        match alignments[idx] {
            TableAlign::None | TableAlign::Left => {
                out.push_str(cell);
                push_spaces(&mut out, pad);
            }
            TableAlign::Right => {
                push_spaces(&mut out, pad);
                out.push_str(cell);
            }
            TableAlign::Center => {
                let left_pad: usize = pad / 2;
                let right_pad: usize = pad - left_pad;
                push_spaces(&mut out, left_pad);
                out.push_str(cell);
                push_spaces(&mut out, right_pad);
            }
        }
        out.push_str(" |");
    }
    out
}

/// Formats a GFM table separator row matching Prettier dash counts and alignment colons.
///
/// # Arguments
///
/// - `&[usize]` - Column display widths (`>= 3`).
/// - `&[TableAlign]` - Column alignments.
///
/// # Returns
///
/// - `String` - Formatted delimiter row.
fn format_table_sep_row(widths: &[usize], alignments: &[TableAlign]) -> String {
    let mut out: String = String::from("|");
    for (idx, &width) in widths.iter().enumerate() {
        out.push(TEXT_SPACE);
        match alignments[idx] {
            TableAlign::None => {
                for _ in 0..width {
                    out.push('-');
                }
            }
            TableAlign::Left => {
                out.push(':');
                for _ in 0..width.saturating_sub(1) {
                    out.push('-');
                }
            }
            TableAlign::Right => {
                for _ in 0..width.saturating_sub(1) {
                    out.push('-');
                }
                out.push(':');
            }
            TableAlign::Center => {
                out.push(':');
                for _ in 0..width.saturating_sub(2) {
                    out.push('-');
                }
                out.push(':');
            }
        }
        out.push_str(" |");
    }
    out
}

/// Parses and normalizes an ATX heading (`#`..`###### Title`).
///
/// # Arguments
///
/// - `&str` - Trimmed Markdown line.
///
/// # Returns
///
/// - `Option<String>` - Normalized heading line if valid ATX heading.
fn format_atx_heading(trimmed: &str) -> Option<String> {
    let level: usize = trimmed.chars().take_while(|c: &char| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let rest: &str = &trimmed[level..];
    if !rest.is_empty() && !rest.starts_with(TEXT_SPACE) && !rest.starts_with(TEXT_TAB) {
        return None;
    }
    let title: &str = rest.trim().trim_end_matches('#').trim_end();
    let hashes: &str = &trimmed[..level];
    if title.is_empty() {
        return Some(hashes.to_string());
    }
    Some(format!("{hashes} {title}"))
}

/// Returns `true` when `trimmed` is a CommonMark thematic break (`---`, `***`, `___`, with optional spaces).
///
/// # Arguments
///
/// - `&str` - Trimmed Markdown line.
///
/// # Returns
///
/// - `bool` - Whether the line is a thematic break.
fn is_thematic_break(trimmed: &str) -> bool {
    let mut marker: Option<char> = None;
    let mut count: usize = 0;
    for ch in trimmed.chars() {
        if ch == TEXT_SPACE || ch == TEXT_TAB {
            continue;
        }
        if ch != '-' && ch != '*' && ch != '_' {
            return false;
        }
        if let Some(m) = marker {
            if m != ch {
                return false;
            }
        } else {
            marker = Some(ch);
        }
        count += 1;
    }
    count >= 3
}

/// Formats a Markdown blockquote line (bare `>` for empty lines, `> content` for non-empty lines).
///
/// # Arguments
///
/// - `&str` - Trimmed blockquote line starting with `>`.
///
/// # Returns
///
/// - `String` - Normalized blockquote line.
fn format_blockquote_line(trimmed: &str) -> String {
    let mut rest: &str = trimmed;
    let mut depth: usize = 0;
    while let Some(after) = rest.strip_prefix('>') {
        depth += 1;
        rest = after.trim_start_matches([TEXT_SPACE, TEXT_TAB]);
    }
    let mut out: String = String::with_capacity(depth * 2 + rest.len());
    for i in 0..depth {
        if i > 0 {
            out.push(TEXT_SPACE);
        }
        out.push('>');
    }
    if !rest.is_empty() {
        out.push(TEXT_SPACE);
        out.push_str(rest);
    }
    out
}

/// Attempts to parse a Markdown unordered or ordered list item line.
///
/// # Arguments
///
/// - `&str` - Raw Markdown line.
///
/// # Returns
///
/// - `Option<ListItemLine>` - Parsed list item metadata if matched.
fn parse_list_item(raw_line: &str) -> Option<ListItemLine> {
    let raw_indent: usize = count_leading_spaces_md(raw_line);
    let trimmed: &str = raw_line.trim_start_matches([TEXT_SPACE, TEXT_TAB]);
    if let Some(rest) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
    {
        return Some(ListItemLine::new(
            raw_indent,
            ListKind::Unordered,
            "-".to_string(),
            rest.trim().to_string(),
        ));
    }
    let digits_len: usize = trimmed
        .bytes()
        .take_while(|b: &u8| b.is_ascii_digit())
        .count();
    if digits_len > 0 && digits_len <= 9 && digits_len < trimmed.len() {
        let bytes: &[u8] = trimmed.as_bytes();
        let delim: u8 = bytes[digits_len];
        if (delim == b'.' || delim == b')')
            && digits_len + 1 < trimmed.len()
            && (bytes[digits_len + 1] == b' ' || bytes[digits_len + 1] == b'\t')
        {
            let number: &str = &trimmed[..digits_len];
            let body: &str = trimmed[(digits_len + 1)..].trim();
            return Some(ListItemLine::new(
                raw_indent,
                ListKind::Ordered,
                format!("{number}."),
                body.to_string(),
            ));
        }
    }
    None
}

/// Normalizes a list item's raw indentation to Prettier's 2-space nesting steps.
///
/// # Arguments
///
/// - `&mut Vec<usize>` - Stack of active raw list indentation levels.
/// - `usize` - Raw leading space count of the current list item.
///
/// # Returns
///
/// - `usize` - Normalized output indentation (`2 * level`).
fn resolve_list_item_indent(list_indents: &mut Vec<usize>, raw_indent: usize) -> usize {
    while let Some(&top) = list_indents.last() {
        if raw_indent < top {
            let _: Option<usize> = list_indents.pop();
        } else {
            break;
        }
    }
    if list_indents.last().copied() != Some(raw_indent) {
        list_indents.push(raw_indent);
    }
    list_indents.len().saturating_sub(1) * PRETTIER_TAB_WIDTH
}

/// Computes normalized indentation for content nested inside an active list item.
///
/// # Arguments
///
/// - `&[usize]` - Active raw list indentation stack.
/// - `usize` - Raw indentation of the nested element.
///
/// # Returns
///
/// - `usize` - Normalized output indentation in spaces.
fn normalize_nested_indent(list_indents: &[usize], raw_indent: usize) -> usize {
    if list_indents.is_empty() {
        return (raw_indent / PRETTIER_TAB_WIDTH) * PRETTIER_TAB_WIDTH;
    }
    let level: usize = list_indents
        .iter()
        .filter(|&&ind: &&usize| ind < raw_indent)
        .count();
    level.max(1) * PRETTIER_TAB_WIDTH
}

/// Counts leading indentation columns of a Markdown line (expanding tabs as 2 spaces).
///
/// # Arguments
///
/// - `&str` - Raw Markdown line.
///
/// # Returns
///
/// - `usize` - Indentation width in spaces.
fn count_leading_spaces_md(line: &str) -> usize {
    let mut count: usize = 0;
    for ch in line.chars() {
        if ch == TEXT_SPACE {
            count += 1;
        } else if ch == TEXT_TAB {
            count += PRETTIER_TAB_WIDTH;
        } else {
            break;
        }
    }
    count
}

/// Returns `true` when `trimmed` begins with a raw HTML block tag or comment (`<center>`, `<div>`, `<!--`, `<img`, etc.).
///
/// # Arguments
///
/// - `&str` - Trimmed Markdown line.
///
/// # Returns
///
/// - `bool` - Whether the line should be treated as an HTML block line.
fn is_html_block_line(trimmed: &str) -> bool {
    if !trimmed.starts_with('<') {
        return false;
    }
    if trimmed.starts_with("<!--") || trimmed.starts_with("</") {
        return true;
    }
    let rest: &str = &trimmed[1..];
    let tag_len: usize = rest
        .bytes()
        .take_while(|b: &u8| b.is_ascii_alphanumeric() || *b == b'-')
        .count();
    if tag_len == 0 {
        return false;
    }
    let after: &str = &rest[tag_len..];
    after.is_empty()
        || after.starts_with('>')
        || after.starts_with(TEXT_SPACE)
        || after.starts_with('/')
}

/// Normalizes standalone Markdown `*italic*` emphasis spans to Prettier's `_italic_` style outside inline code spans, leaving `**bold**` and intraword asterisks untouched.
///
/// # Arguments
///
/// - `&str` - Single Markdown paragraph line.
///
/// # Returns
///
/// - `String` - Line with Prettier-aligned `_italic_` emphasis markers.
fn normalize_inline_emphasis(line: &str) -> String {
    if !line.contains('*') {
        return line.to_string();
    }
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut out: String = String::with_capacity(len);
    let mut i: usize = 0;
    let mut in_code: bool = false;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            out.push_str(&line[i..(i + 2)]);
            i += 2;
            continue;
        }
        if bytes[i] == b'`' {
            in_code = !in_code;
            out.push('`');
            i += 1;
            continue;
        }
        if !in_code && bytes[i] == b'*' {
            let star_run: usize = bytes[i..].iter().take_while(|&&b: &&u8| b == b'*').count();
            let prev_is_alnum: bool = i > 0 && bytes[i - 1].is_ascii_alphanumeric();
            if star_run == 1
                && !prev_is_alnum
                && i + 1 < len
                && bytes[i + 1] != b' '
                && bytes[i + 1] != b'\t'
                && let Some(close_idx) = find_closing_single_star(bytes, i + 1)
            {
                out.push('_');
                out.push_str(&line[(i + 1)..close_idx]);
                out.push('_');
                i = close_idx + 1;
                continue;
            }
            out.push_str(&line[i..(i + star_run)]);
            i += star_run;
            continue;
        }
        let ch_len: usize = line[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        out.push_str(&line[i..(i + ch_len)]);
        i += ch_len;
    }
    out
}

/// Finds the matching closing single `*` for a Prettier `_italic_` span starting after the opening `*`.
///
/// # Arguments
///
/// - `&[u8]` - Paragraph line bytes.
/// - `usize` - Byte offset immediately after the opening `*`.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of the closing `*` if valid emphasis span.
fn find_closing_single_star(bytes: &[u8], start: usize) -> Option<usize> {
    let len: usize = bytes.len();
    let mut i: usize = start;
    let mut in_code: bool = false;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b'`' {
            in_code = !in_code;
            i += 1;
            continue;
        }
        if !in_code && bytes[i] == b'*' {
            let star_run: usize = bytes[i..].iter().take_while(|&&b: &&u8| b == b'*').count();
            if star_run == 1 {
                let prev_non_ws: bool = i > start && bytes[i - 1] != b' ' && bytes[i - 1] != b'\t';
                let next_is_alnum: bool = i + 1 < len && bytes[i + 1].is_ascii_alphanumeric();
                if prev_non_ws && !next_is_alnum {
                    return Some(i);
                }
            }
            i += star_run;
            continue;
        }
        i += 1;
    }
    None
}
