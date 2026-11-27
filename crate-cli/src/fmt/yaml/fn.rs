use super::*;

/// Formats YAML / YML content matching VS Code `esbenp.prettier-vscode` (`tabWidth: 2`, `singleQuote: true`, `bracketSpacing: true`, LF).
///
/// # Arguments
///
/// - `&str` - Raw YAML or YML document text.
///
/// # Returns
///
/// - `String` - Formatted YAML document ending with a single `\n`.
pub fn format_yaml(input: &str) -> String {
    let normalized: String = normalize_line_endings(input);
    let raw_lines: Vec<&str> = normalized.split(TEXT_LF).collect();
    let total: usize = raw_lines.len();
    let mut out_lines: Vec<String> = Vec::new();
    let mut scopes: Vec<YamlScope> = Vec::new();
    let mut flow_depth: usize = 0;
    let mut flow_base_out: usize = 0;
    let mut prev_blank: bool = false;
    let mut idx: usize = 0;
    while idx < total {
        let raw_line: &str = raw_lines[idx];
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() {
            if !out_lines.is_empty() && !prev_blank {
                out_lines.push(String::new());
                prev_blank = true;
            }
            idx += 1;
            continue;
        }
        prev_blank = false;
        if trimmed == YAML_DOC_START || trimmed == YAML_DOC_END {
            scopes.clear();
            flow_depth = 0;
            out_lines.push(trimmed.to_string());
            idx += 1;
            continue;
        }
        let raw_indent: usize = count_yaml_indent(raw_line);
        if flow_depth > 0 {
            let leading_close: bool = trimmed.starts_with(']') || trimmed.starts_with('}');
            let line_depth: usize = if leading_close {
                flow_depth.saturating_sub(1)
            } else {
                flow_depth
            };
            let formatted_content: String = format_yaml_flow_line(trimmed);
            let out_indent: usize = flow_base_out + line_depth * PRETTIER_TAB_WIDTH;
            let mut line: String = String::with_capacity(out_indent + formatted_content.len());
            push_spaces(&mut line, out_indent);
            line.push_str(&formatted_content);
            out_lines.push(line);
            let delta: isize = scan_yaml_flow_delta(trimmed);
            let next_flow: isize = (flow_depth as isize) + delta;
            flow_depth = if next_flow < 0 { 0 } else { next_flow as usize };
            idx += 1;
            continue;
        }
        if trimmed.starts_with('#') {
            let comment_indent: usize =
                resolve_comment_indent(&scopes, raw_indent, &raw_lines, idx);
            let mut line: String = String::with_capacity(comment_indent + trimmed.len());
            push_spaces(&mut line, comment_indent);
            line.push_str(trimmed);
            out_lines.push(line);
            idx += 1;
            continue;
        }
        let is_seq: bool = is_yaml_sequence_line(trimmed);
        let out_indent: usize = resolve_yaml_indent(&mut scopes, raw_indent, is_seq);
        let formatted_content: String = format_yaml_line_content(trimmed);
        let mut line: String = String::with_capacity(out_indent + formatted_content.len());
        push_spaces(&mut line, out_indent);
        line.push_str(&formatted_content);
        out_lines.push(line);
        let opens_block_scalar: bool = line_starts_block_scalar(&formatted_content);
        let expects_child: bool =
            !opens_block_scalar && line_expects_block_child(&formatted_content);
        if is_seq {
            scopes.push(YamlScope::new(raw_indent, out_indent, false, true));
            let after_dash: &str = strip_yaml_seq_marker(&formatted_content);
            if !after_dash.is_empty() && find_yaml_colon(after_dash).is_some() {
                let inline_raw: usize = raw_indent + PRETTIER_TAB_WIDTH;
                let inline_out: usize = out_indent + PRETTIER_TAB_WIDTH;
                scopes.push(YamlScope::new(inline_raw, inline_out, expects_child, false));
            }
        } else {
            scopes.push(YamlScope::new(raw_indent, out_indent, expects_child, false));
        }
        let delta: isize = scan_yaml_flow_delta(&formatted_content);
        if delta > 0 {
            flow_depth = delta as usize;
            flow_base_out = out_indent;
        }
        if opens_block_scalar {
            let scalar_owner_raw: usize =
                if is_seq && find_yaml_colon(strip_yaml_seq_marker(&formatted_content)).is_some() {
                    raw_indent + PRETTIER_TAB_WIDTH
                } else {
                    raw_indent
                };
            let scalar_target_indent: usize =
                if is_seq && find_yaml_colon(strip_yaml_seq_marker(&formatted_content)).is_some() {
                    out_indent + PRETTIER_TAB_WIDTH * 2
                } else {
                    out_indent + PRETTIER_TAB_WIDTH
                };
            let (consumed, had_trailing_blank): (usize, bool) = collect_and_emit_block_scalar(
                &raw_lines,
                idx + 1,
                scalar_owner_raw,
                scalar_target_indent,
                &mut out_lines,
            );
            idx += 1 + consumed;
            if had_trailing_blank {
                out_lines.push(String::new());
                prev_blank = true;
            }
            continue;
        }
        idx += 1;
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

/// Counts leading indentation columns of a YAML line (expanding tabs as 2 spaces).
///
/// # Arguments
///
/// - `&str` - Raw YAML line.
///
/// # Returns
///
/// - `usize` - Leading indentation width in spaces.
fn count_yaml_indent(line: &str) -> usize {
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

/// Returns `true` when a trimmed YAML line starts a block sequence item (`-` or `- ...`).
///
/// # Arguments
///
/// - `&str` - Trimmed YAML line.
///
/// # Returns
///
/// - `bool` - Whether the line is a sequence item.
fn is_yaml_sequence_line(trimmed: &str) -> bool {
    trimmed == "-" || trimmed.starts_with("- ") || trimmed.starts_with("-\t")
}

/// Strips the leading `-` sequence marker and following space from a trimmed sequence line.
///
/// # Arguments
///
/// - `&str` - Trimmed YAML sequence line.
///
/// # Returns
///
/// - `&str` - Content following the `-` marker.
fn strip_yaml_seq_marker(trimmed: &str) -> &str {
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return rest.trim_start();
    }
    if let Some(rest) = trimmed.strip_prefix("-\t") {
        return rest.trim_start();
    }
    ""
}

/// Resolves the normalized Prettier 2-space indentation for the current YAML line and updates the scope stack.
///
/// # Arguments
///
/// - `&mut Vec<YamlScope>` - Active indentation scope stack.
/// - `usize` - Raw source indentation of the current line.
/// - `bool` - Whether the current line begins with a sequence `-` marker.
///
/// # Returns
///
/// - `usize` - Target output indentation column.
fn resolve_yaml_indent(scopes: &mut Vec<YamlScope>, raw_indent: usize, is_seq: bool) -> usize {
    while let Some(top) = scopes.last().copied() {
        if raw_indent > top.get_raw_indent() {
            break;
        }
        if is_seq
            && raw_indent == top.get_raw_indent()
            && top.get_expects_block_child()
            && !top.get_is_sequence()
        {
            return top.get_out_indent() + PRETTIER_TAB_WIDTH;
        }
        if is_seq && raw_indent == top.get_raw_indent() && top.get_is_sequence() {
            let out: usize = top.get_out_indent();
            let _: Option<YamlScope> = scopes.pop();
            return out;
        }
        if !is_seq && raw_indent == top.get_raw_indent() && !top.get_is_sequence() {
            let out: usize = top.get_out_indent();
            let _: Option<YamlScope> = scopes.pop();
            return out;
        }
        let _: Option<YamlScope> = scopes.pop();
    }
    if let Some(parent) = scopes.last().copied() {
        return parent.get_out_indent() + PRETTIER_TAB_WIDTH;
    }
    0
}

/// Resolves indentation for a full-line YAML comment relative to active scopes and the following line.
///
/// # Arguments
///
/// - `&[YamlScope]` - Active scope stack.
/// - `usize` - Raw indentation of the comment line.
/// - `&[&str]` - All source lines.
/// - `usize` - Index of the comment line.
///
/// # Returns
///
/// - `usize` - Normalized output indentation column for the comment.
fn resolve_comment_indent(
    scopes: &[YamlScope],
    raw_indent: usize,
    raw_lines: &[&str],
    idx: usize,
) -> usize {
    if raw_indent == 0 {
        return 0;
    }
    let mut probe: usize = idx + 1;
    while probe < raw_lines.len() {
        let next_trimmed: &str = raw_lines[probe].trim();
        if !next_trimmed.is_empty() && !next_trimmed.starts_with('#') {
            let next_raw: usize = count_yaml_indent(raw_lines[probe]);
            if next_raw == raw_indent {
                let mut cloned: Vec<YamlScope> = scopes.to_vec();
                return resolve_yaml_indent(
                    &mut cloned,
                    next_raw,
                    is_yaml_sequence_line(next_trimmed),
                );
            }
            break;
        }
        probe += 1;
    }
    for scope in scopes.iter().rev() {
        if raw_indent == scope.get_raw_indent() {
            return scope.get_out_indent();
        }
        if raw_indent > scope.get_raw_indent() {
            return scope.get_out_indent() + PRETTIER_TAB_WIDTH;
        }
    }
    0
}

/// Captures and emits all lines of a YAML block scalar (`|`, `|-`, `>`, etc.) preserving relative indentation verbatim.
///
/// # Arguments
///
/// - `&[&str]` - All source lines.
/// - `usize` - Starting line index immediately after the block scalar header.
/// - `usize` - Raw indentation of the block scalar's owner key or item.
/// - `usize` - Target normalized base indentation for the scalar's body lines.
/// - `&mut Vec<String>` - Output line buffer.
///
/// # Returns
///
/// - `(usize, bool)` - `(lines_consumed, trailing_blank_line_deferred)`.
fn collect_and_emit_block_scalar(
    raw_lines: &[&str],
    start_idx: usize,
    owner_raw_indent: usize,
    target_base_indent: usize,
    out_lines: &mut Vec<String>,
) -> (usize, bool) {
    let mut body: Vec<(usize, &str)> = Vec::new();
    let mut cursor: usize = start_idx;
    while cursor < raw_lines.len() {
        let line: &str = raw_lines[cursor];
        let trimmed_end: &str = line.trim_end_matches([TEXT_SPACE, TEXT_TAB]);
        if trimmed_end.is_empty() {
            body.push((0, ""));
            cursor += 1;
            continue;
        }
        let line_indent: usize = count_yaml_indent(line);
        if line_indent <= owner_raw_indent {
            break;
        }
        let content_start: usize = byte_offset_after_indent(line);
        body.push((line_indent, &trimmed_end[content_start..]));
        cursor += 1;
    }
    let mut trailing_blanks: usize = 0;
    while let Some((_, text)) = body.last() {
        if text.is_empty() {
            let _: Option<(usize, &str)> = body.pop();
            trailing_blanks += 1;
        } else {
            break;
        }
    }
    let min_indent: usize = body
        .iter()
        .filter(|(_, text): &&(usize, &str)| !text.is_empty())
        .map(|(ind, _): &(usize, &str)| *ind)
        .min()
        .unwrap_or(owner_raw_indent + PRETTIER_TAB_WIDTH);
    for (line_indent, text) in body {
        if text.is_empty() {
            out_lines.push(String::new());
        } else {
            let rel_indent: usize = line_indent.saturating_sub(min_indent);
            let total_indent: usize = target_base_indent + rel_indent;
            let mut formatted: String = String::with_capacity(total_indent + text.len());
            push_spaces(&mut formatted, total_indent);
            formatted.push_str(text);
            out_lines.push(formatted);
        }
    }
    let consumed: usize = cursor - start_idx;
    let has_more_content: bool = cursor < raw_lines.len();
    (consumed, trailing_blanks > 0 && has_more_content)
}

/// Computes the byte index of the first non-whitespace character in `line`.
///
/// # Arguments
///
/// - `&str` - Raw line.
///
/// # Returns
///
/// - `usize` - Byte offset of first non-space/tab character.
fn byte_offset_after_indent(line: &str) -> usize {
    let bytes: &[u8] = line.as_bytes();
    let mut i: usize = 0;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    i
}

/// Formats a single trimmed YAML line (normalizing sequence marker, mapping colon spacing, flow spacing, and `singleQuote: true`).
///
/// # Arguments
///
/// - `&str` - Trimmed YAML line.
///
/// # Returns
///
/// - `String` - Prettier-aligned YAML line content.
fn format_yaml_line_content(trimmed: &str) -> String {
    if trimmed == "-" {
        return "-".to_string();
    }
    if is_yaml_sequence_line(trimmed) {
        let rest: &str = strip_yaml_seq_marker(trimmed);
        if rest.is_empty() {
            return "-".to_string();
        }
        return format!("- {}", format_yaml_entry(rest));
    }
    format_yaml_entry(trimmed)
}

/// Formats a YAML mapping entry or scalar value while keeping inline comments intact.
///
/// # Arguments
///
/// - `&str` - Trimmed mapping or value line (without leading `- `).
///
/// # Returns
///
/// - `String` - Normalized YAML entry.
fn format_yaml_entry(entry: &str) -> String {
    let (code_part, comment_part): (&str, Option<&str>) = split_yaml_comment(entry);
    let formatted_code: String = if let Some(colon_idx) = find_yaml_colon(code_part) {
        let raw_key: &str = code_part[..colon_idx].trim_end();
        let raw_val: &str = code_part[(colon_idx + 1)..].trim();
        let key: String = convert_yaml_quotes_and_flows(raw_key);
        if raw_val.is_empty() {
            format!("{key}:")
        } else {
            let val: String = convert_yaml_quotes_and_flows(raw_val);
            format!("{key}: {val}")
        }
    } else {
        convert_yaml_quotes_and_flows(code_part.trim())
    };
    if let Some(comment) = comment_part {
        if formatted_code.is_empty() {
            return comment.to_string();
        }
        return format!("{formatted_code} {comment}");
    }
    formatted_code
}

/// Formats a single line inside a multi-line YAML flow sequence or mapping.
///
/// # Arguments
///
/// - `&str` - Trimmed flow line.
///
/// # Returns
///
/// - `String` - Normalized flow line.
fn format_yaml_flow_line(trimmed: &str) -> String {
    let (code_part, comment_part): (&str, Option<&str>) = split_yaml_comment(trimmed);
    let formatted_code: String = convert_yaml_quotes_and_flows(code_part.trim());
    if let Some(comment) = comment_part {
        if formatted_code.is_empty() {
            return comment.to_string();
        }
        return format!("{formatted_code} {comment}");
    }
    formatted_code
}

/// Splits a YAML line into `(code_without_trailing_ws, Option<inline_comment>)`, ignoring `#` inside quotes or `${{ ... }}`.
///
/// # Arguments
///
/// - `&str` - YAML line segment.
///
/// # Returns
///
/// - `(&str, Option<&str>)` - Code slice and optional `#...` comment slice.
fn split_yaml_comment(input: &str) -> (&str, Option<&str>) {
    let bytes: &[u8] = input.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut in_single: bool = false;
    let mut in_double: bool = false;
    while i < len {
        if in_single {
            if bytes[i] == b'\'' {
                if i + 1 < len && bytes[i + 1] == b'\'' {
                    i += 2;
                    continue;
                }
                in_single = false;
            }
            i += 1;
            continue;
        }
        if in_double {
            if bytes[i] == b'\\' && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == b'"' {
                in_double = false;
            }
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(YAML_EXPR_OPEN.as_bytes())
            && let Some(close_rel) =
                find_bytes_subslice(&bytes[(i + 3)..], YAML_EXPR_CLOSE.as_bytes())
        {
            i = i + 3 + close_rel + 2;
            continue;
        }
        match bytes[i] {
            b'\'' => in_single = true,
            b'"' => in_double = true,
            b'#' if i == 0 || bytes[i - 1] == b' ' || bytes[i - 1] == b'\t' => {
                return (input[..i].trim_end(), Some(input[i..].trim_end()));
            }
            _ => {}
        }
        i += 1;
    }
    (input.trim_end(), None)
}

/// Finds the mapping key/value separator `:` at top level (outside quotes, flow containers, and `${{ ... }}`).
///
/// # Arguments
///
/// - `&str` - YAML code slice without inline comments.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of the mapping separator `:`.
fn find_yaml_colon(input: &str) -> Option<usize> {
    let bytes: &[u8] = input.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut in_single: bool = false;
    let mut in_double: bool = false;
    let mut flow_level: usize = 0;
    while i < len {
        if in_single {
            if bytes[i] == b'\'' {
                if i + 1 < len && bytes[i + 1] == b'\'' {
                    i += 2;
                    continue;
                }
                in_single = false;
            }
            i += 1;
            continue;
        }
        if in_double {
            if bytes[i] == b'\\' && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == b'"' {
                in_double = false;
            }
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(YAML_EXPR_OPEN.as_bytes())
            && let Some(close_rel) =
                find_bytes_subslice(&bytes[(i + 3)..], YAML_EXPR_CLOSE.as_bytes())
        {
            i = i + 3 + close_rel + 2;
            continue;
        }
        match bytes[i] {
            b'\'' => in_single = true,
            b'"' => in_double = true,
            b'[' | b'{' => flow_level += 1,
            b']' | b'}' => flow_level = flow_level.saturating_sub(1),
            b':' if flow_level == 0
                && (i + 1 == len || bytes[i + 1] == b' ' || bytes[i + 1] == b'\t') =>
            {
                return Some(i);
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Applies Prettier `singleQuote: true` and flow-collection spacing (`[a, b]`, `{ a: b }`) strictly to YAML quoted scalars and flow containers, leaving plain scalars (such as `run: echo "ok"`) and `${{ ... }}` expressions untouched.
///
/// # Arguments
///
/// - `&str` - Trimmed YAML key, scalar value, or flow element (no comments).
///
/// # Returns
///
/// - `String` - Normalized YAML fragment.
fn convert_yaml_quotes_and_flows(input: &str) -> String {
    let trimmed: &str = input.trim();
    if trimmed.is_empty() || trimmed.starts_with(YAML_EXPR_OPEN) {
        return trimmed.to_string();
    }
    let bytes: &[u8] = trimmed.as_bytes();
    if bytes[0] == b'"' {
        let end_idx: usize = scan_double_quoted(bytes, 0);
        if end_idx == bytes.len() {
            return normalize_yaml_double_quote(trimmed);
        }
        return trimmed.to_string();
    }
    if bytes[0] == b'['
        && let Some(close_idx) = find_matching_flow_delim(trimmed, 0, b'[', b']')
        && close_idx + 1 == bytes.len()
    {
        return format_yaml_flow_sequence(&trimmed[1..close_idx]);
    }
    if bytes[0] == b'{'
        && let Some(close_idx) = find_matching_flow_delim(trimmed, 0, b'{', b'}')
        && close_idx + 1 == bytes.len()
    {
        return format_yaml_flow_mapping(&trimmed[1..close_idx]);
    }
    trimmed.to_string()
}

/// Converts a YAML double-quoted scalar `"..."` into `'...'` when Prettier `singleQuote: true` permits it.
///
/// # Arguments
///
/// - `&str` - Double-quoted token including surrounding `"` delimiters.
///
/// # Returns
///
/// - `String` - Single-quoted token if safe, or original double-quoted token when containing `'` or escapes.
fn normalize_yaml_double_quote(token: &str) -> String {
    if token.len() < 2 || !token.starts_with('"') || !token.ends_with('"') {
        return token.to_string();
    }
    let inner: &str = &token[1..(token.len() - 1)];
    if inner.contains('\'') || inner.contains('\\') {
        return token.to_string();
    }
    let has_control: bool = inner.chars().any(|c: char| (c as u32) < 0x20);
    if has_control {
        return token.to_string();
    }
    format!("'{inner}'")
}

/// Formats an inline YAML flow sequence `[a, b]` matching Prettier (`['a', 'b']`, no outer padding).
///
/// # Arguments
///
/// - `&str` - Inner slice between `[` and `]`.
///
/// # Returns
///
/// - `String` - Formatted `[...]` sequence.
fn format_yaml_flow_sequence(inner: &str) -> String {
    let trimmed: &str = inner.trim();
    if trimmed.is_empty() {
        return "[]".to_string();
    }
    let parts: Vec<&str> = split_yaml_flow_items(trimmed);
    let formatted: Vec<String> = parts
        .into_iter()
        .map(|p: &str| convert_yaml_quotes_and_flows(p.trim()))
        .collect();
    format!("[{}]", formatted.join(", "))
}

/// Formats an inline YAML flow mapping `{ a: 'b' }` matching Prettier (`bracketSpacing: true`).
///
/// # Arguments
///
/// - `&str` - Inner slice between `{` and `}`.
///
/// # Returns
///
/// - `String` - Formatted `{ ... }` mapping.
fn format_yaml_flow_mapping(inner: &str) -> String {
    let trimmed: &str = inner.trim();
    if trimmed.is_empty() {
        return "{}".to_string();
    }
    let parts: Vec<&str> = split_yaml_flow_items(trimmed);
    let mut formatted: Vec<String> = Vec::with_capacity(parts.len());
    for part in parts {
        let item: &str = part.trim();
        if let Some(colon_idx) = find_yaml_colon(item) {
            let k: String = convert_yaml_quotes_and_flows(item[..colon_idx].trim_end());
            let v: String = convert_yaml_quotes_and_flows(item[(colon_idx + 1)..].trim());
            formatted.push(format!("{k}: {v}"));
        } else {
            formatted.push(convert_yaml_quotes_and_flows(item));
        }
    }
    format!("{{ {} }}", formatted.join(", "))
}

/// Splits top-level comma-separated items inside a YAML flow sequence or mapping.
///
/// # Arguments
///
/// - `&str` - Trimmed content inside `[...]` or `{...}`.
///
/// # Returns
///
/// - `Vec<&str>` - Slice for each element.
fn split_yaml_flow_items(inner: &str) -> Vec<&str> {
    let bytes: &[u8] = inner.as_bytes();
    let len: usize = bytes.len();
    let mut out: Vec<&str> = Vec::new();
    let mut start: usize = 0;
    let mut i: usize = 0;
    let mut depth: usize = 0;
    while i < len {
        if bytes[i..].starts_with(YAML_EXPR_OPEN.as_bytes())
            && let Some(close_rel) =
                find_bytes_subslice(&bytes[(i + 3)..], YAML_EXPR_CLOSE.as_bytes())
        {
            i = i + 3 + close_rel + 2;
            continue;
        }
        if bytes[i] == b'\'' {
            i = scan_single_quoted(bytes, i);
            continue;
        }
        if bytes[i] == b'"' {
            i = scan_double_quoted(bytes, i);
            continue;
        }
        match bytes[i] {
            b'[' | b'{' => depth += 1,
            b']' | b'}' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                let piece: &str = inner[start..i].trim();
                if !piece.is_empty() {
                    out.push(piece);
                }
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    let tail: &str = inner[start..].trim();
    if !tail.is_empty() {
        out.push(tail);
    }
    out
}

/// Finds the matching closing flow delimiter (`]` or `}`) on the same line, ignoring quotes and `${{ ... }}`.
///
/// # Arguments
///
/// - `&str` - Source line.
/// - `usize` - Byte offset of the opening delimiter.
/// - `u8` - Opening byte (`b'['` or `b'{'`).
/// - `u8` - Closing byte (`b']'` or `b'}'`).
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of matching closing delimiter on the same line.
fn find_matching_flow_delim(input: &str, open_idx: usize, open: u8, close: u8) -> Option<usize> {
    let bytes: &[u8] = input.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = open_idx + 1;
    let mut depth: usize = 1;
    while i < len {
        if bytes[i..].starts_with(YAML_EXPR_OPEN.as_bytes())
            && let Some(close_rel) =
                find_bytes_subslice(&bytes[(i + 3)..], YAML_EXPR_CLOSE.as_bytes())
        {
            i = i + 3 + close_rel + 2;
            continue;
        }
        if bytes[i] == b'\'' {
            i = scan_single_quoted(bytes, i);
            continue;
        }
        if bytes[i] == b'"' {
            i = scan_double_quoted(bytes, i);
            continue;
        }
        if bytes[i] == open {
            depth += 1;
        } else if bytes[i] == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Scans past a YAML single-quoted string starting at `start` (`''` escapes a literal `'`).
///
/// # Arguments
///
/// - `&[u8]` - Source bytes.
/// - `usize` - Index of opening `'`.
///
/// # Returns
///
/// - `usize` - Byte offset immediately after closing `'`.
fn scan_single_quoted(bytes: &[u8], start: usize) -> usize {
    let len: usize = bytes.len();
    let mut i: usize = start + 1;
    while i < len {
        if bytes[i] == b'\'' {
            if i + 1 < len && bytes[i + 1] == b'\'' {
                i += 2;
                continue;
            }
            return i + 1;
        }
        i += 1;
    }
    len
}

/// Scans past a YAML double-quoted string starting at `start` (`\` escapes next char).
///
/// # Arguments
///
/// - `&[u8]` - Source bytes.
/// - `usize` - Index of opening `"`.
///
/// # Returns
///
/// - `usize` - Byte offset immediately after closing `"`.
fn scan_double_quoted(bytes: &[u8], start: usize) -> usize {
    let len: usize = bytes.len();
    let mut i: usize = start + 1;
    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            return i + 1;
        }
        i += 1;
    }
    len
}

/// Computes the net unclosed flow delimiter (`[`/`{` minus `]`/`}`) delta on a single YAML line.
///
/// # Arguments
///
/// - `&str` - Formatted YAML line.
///
/// # Returns
///
/// - `isize` - Net flow nesting change.
fn scan_yaml_flow_delta(line: &str) -> isize {
    let (code_part, _): (&str, Option<&str>) = split_yaml_comment(line);
    let bytes: &[u8] = code_part.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut delta: isize = 0;
    while i < len {
        if bytes[i..].starts_with(YAML_EXPR_OPEN.as_bytes())
            && let Some(close_rel) =
                find_bytes_subslice(&bytes[(i + 3)..], YAML_EXPR_CLOSE.as_bytes())
        {
            i = i + 3 + close_rel + 2;
            continue;
        }
        if bytes[i] == b'\'' {
            i = scan_single_quoted(bytes, i);
            continue;
        }
        if bytes[i] == b'"' {
            i = scan_double_quoted(bytes, i);
            continue;
        }
        match bytes[i] {
            b'[' | b'{' => delta += 1,
            b']' | b'}' => delta -= 1,
            _ => {}
        }
        i += 1;
    }
    delta
}

/// Returns `true` when a YAML line ends with a block scalar indicator (`|`, `|-`, `|+`, `>`, `>-`, `>+`, `|2`, etc.).
///
/// # Arguments
///
/// - `&str` - Trimmed YAML line.
///
/// # Returns
///
/// - `bool` - Whether the following indented lines belong to a block scalar.
fn line_starts_block_scalar(line: &str) -> bool {
    let (code_part, _): (&str, Option<&str>) = split_yaml_comment(line);
    let body: &str = if is_yaml_sequence_line(code_part) {
        strip_yaml_seq_marker(code_part)
    } else {
        code_part
    };
    let candidate: &str = if let Some(colon_idx) = find_yaml_colon(body) {
        body[(colon_idx + 1)..].trim()
    } else {
        body.trim()
    };
    is_block_scalar_indicator(candidate)
}

/// Checks whether `token` is a valid YAML block scalar header (`|` or `>` with optional chomping/indent modifiers).
///
/// # Arguments
///
/// - `&str` - Trimmed value token.
///
/// # Returns
///
/// - `bool` - Whether `token` introduces a literal or folded block scalar.
fn is_block_scalar_indicator(token: &str) -> bool {
    let bytes: &[u8] = token.as_bytes();
    if bytes.is_empty() || (bytes[0] != b'|' && bytes[0] != b'>') {
        return false;
    }
    if bytes.len() > 3 {
        return false;
    }
    let mut seen_chomp: bool = false;
    let mut seen_digit: bool = false;
    for &b in &bytes[1..] {
        if (b == b'-' || b == b'+') && !seen_chomp {
            seen_chomp = true;
        } else if (b'1'..=b'9').contains(&b) && !seen_digit {
            seen_digit = true;
        } else {
            return false;
        }
    }
    true
}

/// Returns `true` when a YAML mapping line has no inline value and expects a nested block mapping or sequence.
///
/// # Arguments
///
/// - `&str` - Trimmed YAML line.
///
/// # Returns
///
/// - `bool` - Whether nested lines belong under this key.
fn line_expects_block_child(line: &str) -> bool {
    let (code_part, _): (&str, Option<&str>) = split_yaml_comment(line);
    let body: &str = if is_yaml_sequence_line(code_part) {
        strip_yaml_seq_marker(code_part)
    } else {
        code_part
    };
    if let Some(colon_idx) = find_yaml_colon(body) {
        return body[(colon_idx + 1)..].trim().is_empty();
    }
    false
}

/// Searches for the first occurrence of `needle` inside `haystack` safely at the byte level without UTF-8 slicing panics.
///
/// # Arguments
///
/// - `&[u8]` - Byte slice to search.
/// - `&[u8]` - Subslice pattern to locate.
///
/// # Returns
///
/// - `Option<usize>` - Byte offset of the first match.
fn find_bytes_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|w: &[u8]| w == needle)
}
