use super::*;

/// Formats strict JSON matching Prettier 3.7.4 (`tabWidth: 2`, `printWidth: 80`, `bracketSpacing: true`, LF).
///
/// # Arguments
///
/// - `&str` - Raw JSON source text.
///
/// # Returns
///
/// - `Result<String, io::Error>` - Formatted JSON ending with `\n`, or an I/O error if invalid JSON.
pub fn format_json(input: &str) -> Result<String, io::Error> {
    let normalized: String = normalize_line_endings(input);
    let trimmed: &str = normalized.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let mut parser: JsonParser<'_> = JsonParser::new(trimmed, trimmed.as_bytes(), 0);
    let node: JsonNode = parse_json_value(&mut parser)?;
    skip_json_ws(&mut parser);
    if parser.get_pos() != parser.get_bytes().len() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            ERROR_JSON_PARSE_FAILED,
        ));
    }
    let mut out: String = String::with_capacity(trimmed.len() + 32);
    render_json_node(&node, 0, 0, &mut out);
    out.push(TEXT_LF);
    Ok(out)
}

/// Formats JSONC (JSON with `//` or `/* */` comments and trailing commas) matching Prettier 3.7.4 (`tabWidth: 2`, LF).
///
/// # Arguments
///
/// - `&str` - Raw JSONC source text.
///
/// # Returns
///
/// - `String` - Formatted JSONC preserving comments and trailing commas cleanly.
pub fn format_jsonc(input: &str) -> String {
    let normalized: String = normalize_line_endings(input);
    if !contains_jsonc_syntax(&normalized)
        && let Ok(formatted) = format_json(&normalized)
    {
        return formatted;
    }
    let mut out_lines: Vec<String> = Vec::new();
    let mut depth: usize = 0;
    let mut in_block_comment: bool = false;
    let mut prev_blank: bool = false;
    for raw_line in normalized.split(TEXT_LF) {
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() {
            if !out_lines.is_empty() && !prev_blank {
                out_lines.push(String::new());
                prev_blank = true;
            }
            continue;
        }
        prev_blank = false;
        let (leading_closes, net_opens, next_block): (usize, isize, bool) =
            scan_jsonc_line(trimmed, in_block_comment);
        in_block_comment = next_block;
        let line_depth: usize = depth.saturating_sub(leading_closes);
        let mut line: String =
            String::with_capacity(line_depth * PRETTIER_TAB_WIDTH + trimmed.len());
        push_spaces(&mut line, line_depth * PRETTIER_TAB_WIDTH);
        line.push_str(trimmed);
        out_lines.push(line);
        let updated: isize = (depth as isize) + net_opens;
        depth = if updated < 0 { 0 } else { updated as usize };
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

/// Checks whether a JSON input contains JSONC comments (`//` or `/*`).
///
/// # Arguments
///
/// - `&str` - Normalized text to inspect.
///
/// # Returns
///
/// - `bool` - `true` when comments exist outside string literals.
fn contains_jsonc_syntax(input: &str) -> bool {
    let bytes: &[u8] = input.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut in_string: bool = false;
    while i < len {
        if in_string {
            if bytes[i] == b'\\' && i + 1 < len {
                i += 2;
                continue;
            }
            if bytes[i] == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if bytes[i] == b'"' {
            in_string = true;
            i += 1;
            continue;
        }
        if bytes[i] == b'/' && i + 1 < len && (bytes[i + 1] == b'/' || bytes[i + 1] == b'*') {
            return true;
        }
        i += 1;
    }
    false
}

/// Scans a JSONC line for leading closing braces/brackets and net delimiter delta.
///
/// # Arguments
///
/// - `&str` - Trimmed JSONC line.
/// - `bool` - Whether a `/* */` block comment is active at line entry.
///
/// # Returns
///
/// - `(usize, isize, bool)` - `(leading_closes, net_delta, next_in_block_comment)`.
fn scan_jsonc_line(line: &str, mut in_block_comment: bool) -> (usize, isize, bool) {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut leading_closes: usize = 0;
    let mut seen_non_close: bool = false;
    let mut delta: isize = 0;
    while i < len {
        if in_block_comment {
            if bytes[i] == b'*' && i + 1 < len && bytes[i + 1] == b'/' {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            break;
        }
        if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            in_block_comment = true;
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            seen_non_close = true;
            i += 1;
            while i < len {
                if bytes[i] == b'\\' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        match bytes[i] {
            b'{' | b'[' => {
                seen_non_close = true;
                delta += 1;
            }
            b'}' | b']' => {
                if !seen_non_close {
                    leading_closes += 1;
                }
                delta -= 1;
            }
            b' ' | b'\t' | b',' => {}
            _ => {
                seen_non_close = true;
            }
        }
        i += 1;
    }
    (leading_closes, delta, in_block_comment)
}

/// Skips JSON ASCII whitespace at the parser cursor.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state.
fn skip_json_ws(parser: &mut JsonParser<'_>) {
    let bytes: &[u8] = parser.get_bytes();
    let mut pos: usize = parser.get_pos();
    while pos < bytes.len() && is_json_ws_byte(bytes[pos]) {
        pos += 1;
    }
    let _: &mut JsonParser<'_> = parser.set_pos(pos);
}

/// Returns `true` when `byte` is one of the four RFC 8259 JSON whitespace bytes.
///
/// # Arguments
///
/// - `u8` - Input ASCII byte.
///
/// # Returns
///
/// - `bool` - Whether the byte is space, tab, LF, or CR.
fn is_json_ws_byte(byte: u8) -> bool {
    byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r'
}

/// Parses a single JSON value from the current cursor position.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state.
///
/// # Returns
///
/// - `Result<JsonNode, io::Error>` - Parsed JSON AST node or syntax error.
fn parse_json_value(parser: &mut JsonParser<'_>) -> Result<JsonNode, io::Error> {
    skip_json_ws(parser);
    let bytes: &[u8] = parser.get_bytes();
    let pos: usize = parser.get_pos();
    if pos >= bytes.len() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            ERROR_JSON_PARSE_FAILED,
        ));
    }
    match bytes[pos] {
        b'"' => parse_json_string(parser).map(JsonNode::StringRaw),
        b'[' => parse_json_array(parser),
        b'{' => parse_json_object(parser),
        b't' => parse_json_keyword(parser, JSON_LITERAL_TRUE, JsonNode::Bool(true)),
        b'f' => parse_json_keyword(parser, JSON_LITERAL_FALSE, JsonNode::Bool(false)),
        b'n' => parse_json_keyword(parser, JSON_LITERAL_NULL, JsonNode::Null),
        b'-' | b'0'..=b'9' => parse_json_number(parser),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            ERROR_JSON_PARSE_FAILED,
        )),
    }
}

/// Matches a fixed JSON keyword literal (`true`, `false`, `null`).
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state.
/// - `&str` - Expected keyword spelling.
/// - `JsonNode` - AST node returned on match.
///
/// # Returns
///
/// - `Result<JsonNode, io::Error>` - Matched AST node or syntax error.
fn parse_json_keyword(
    parser: &mut JsonParser<'_>,
    expected: &str,
    node: JsonNode,
) -> Result<JsonNode, io::Error> {
    let source: &str = parser.get_source();
    let pos: usize = parser.get_pos();
    if source[pos..].starts_with(expected) {
        let _: &mut JsonParser<'_> = parser.set_pos(pos + expected.len());
        return Ok(node);
    }
    Err(io::Error::new(
        ErrorKind::InvalidData,
        ERROR_JSON_PARSE_FAILED,
    ))
}

/// Parses a raw JSON double-quoted string token preserving escapes verbatim.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state positioned at `"`.
///
/// # Returns
///
/// - `Result<String, io::Error>` - Raw quoted JSON string slice or syntax error.
fn parse_json_string(parser: &mut JsonParser<'_>) -> Result<String, io::Error> {
    let bytes: &[u8] = parser.get_bytes();
    let start: usize = parser.get_pos();
    if start >= bytes.len() || bytes[start] != b'"' {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            ERROR_JSON_PARSE_FAILED,
        ));
    }
    let mut i: usize = start + 1;
    while i < bytes.len() {
        let b: u8 = bytes[i];
        if b < 0x20 {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        if b == b'\\' {
            if i + 1 >= bytes.len() {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    ERROR_JSON_PARSE_FAILED,
                ));
            }
            i += 2;
            continue;
        }
        if b == b'"' {
            let token: String = parser.get_source()[start..=i].to_string();
            let _: &mut JsonParser<'_> = parser.set_pos(i + 1);
            return Ok(token);
        }
        i += 1;
    }
    Err(io::Error::new(
        ErrorKind::InvalidData,
        ERROR_JSON_PARSE_FAILED,
    ))
}

/// Parses a JSON numeric literal preserving its exact lexeme.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state positioned at the start of a number.
///
/// # Returns
///
/// - `Result<JsonNode, io::Error>` - Parsed `JsonNode::Number` or syntax error.
fn parse_json_number(parser: &mut JsonParser<'_>) -> Result<JsonNode, io::Error> {
    let bytes: &[u8] = parser.get_bytes();
    let start: usize = parser.get_pos();
    let mut i: usize = start;
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            ERROR_JSON_PARSE_FAILED,
        ));
    }
    if bytes[i] == b'0' {
        i += 1;
    } else {
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    let lexeme: String = parser.get_source()[start..i].to_string();
    let _: &mut JsonParser<'_> = parser.set_pos(i);
    Ok(JsonNode::Number(lexeme))
}

/// Parses a JSON array node.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state positioned at `[`.
///
/// # Returns
///
/// - `Result<JsonNode, io::Error>` - Parsed `JsonNode::Array` or syntax error.
fn parse_json_array(parser: &mut JsonParser<'_>) -> Result<JsonNode, io::Error> {
    let start: usize = parser.get_pos();
    let _: &mut JsonParser<'_> = parser.set_pos(start + 1);
    skip_json_ws(parser);
    let mut items: Vec<JsonNode> = Vec::new();
    if parser.get_pos() < parser.get_bytes().len() && parser.get_bytes()[parser.get_pos()] == b']' {
        let next: usize = parser.get_pos() + 1;
        let _: &mut JsonParser<'_> = parser.set_pos(next);
        return Ok(JsonNode::Array(items));
    }
    loop {
        let item: JsonNode = parse_json_value(parser)?;
        items.push(item);
        skip_json_ws(parser);
        let pos: usize = parser.get_pos();
        if pos >= parser.get_bytes().len() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        match parser.get_bytes()[pos] {
            b',' => {
                let _: &mut JsonParser<'_> = parser.set_pos(pos + 1);
            }
            b']' => {
                let _: &mut JsonParser<'_> = parser.set_pos(pos + 1);
                return Ok(JsonNode::Array(items));
            }
            _ => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    ERROR_JSON_PARSE_FAILED,
                ));
            }
        }
    }
}

/// Parses a JSON object node preserving insertion order of all keys.
///
/// # Arguments
///
/// - `&mut JsonParser<'_>` - Mutable parser state positioned at `{`.
///
/// # Returns
///
/// - `Result<JsonNode, io::Error>` - Parsed `JsonNode::Object` or syntax error.
fn parse_json_object(parser: &mut JsonParser<'_>) -> Result<JsonNode, io::Error> {
    let start: usize = parser.get_pos();
    let _: &mut JsonParser<'_> = parser.set_pos(start + 1);
    skip_json_ws(parser);
    let mut entries: Vec<(String, JsonNode)> = Vec::new();
    if parser.get_pos() < parser.get_bytes().len() && parser.get_bytes()[parser.get_pos()] == b'}' {
        let next: usize = parser.get_pos() + 1;
        let _: &mut JsonParser<'_> = parser.set_pos(next);
        return Ok(JsonNode::Object(entries));
    }
    loop {
        skip_json_ws(parser);
        let key: String = parse_json_string(parser)?;
        skip_json_ws(parser);
        let colon_pos: usize = parser.get_pos();
        if colon_pos >= parser.get_bytes().len() || parser.get_bytes()[colon_pos] != b':' {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        let _: &mut JsonParser<'_> = parser.set_pos(colon_pos + 1);
        let val: JsonNode = parse_json_value(parser)?;
        entries.push((key, val));
        skip_json_ws(parser);
        let pos: usize = parser.get_pos();
        if pos >= parser.get_bytes().len() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                ERROR_JSON_PARSE_FAILED,
            ));
        }
        match parser.get_bytes()[pos] {
            b',' => {
                let _: &mut JsonParser<'_> = parser.set_pos(pos + 1);
            }
            b'}' => {
                let _: &mut JsonParser<'_> = parser.set_pos(pos + 1);
                return Ok(JsonNode::Object(entries));
            }
            _ => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    ERROR_JSON_PARSE_FAILED,
                ));
            }
        }
    }
}

/// Renders a JSON AST node onto a single line to measure Prettier group width.
///
/// # Arguments
///
/// - `&JsonNode` - AST node to render inline.
///
/// # Returns
///
/// - `String` - Single-line representation.
fn render_json_inline(node: &JsonNode) -> String {
    match node {
        JsonNode::Null => JSON_LITERAL_NULL.to_string(),
        JsonNode::Bool(true) => JSON_LITERAL_TRUE.to_string(),
        JsonNode::Bool(false) => JSON_LITERAL_FALSE.to_string(),
        JsonNode::Number(raw) => raw.clone(),
        JsonNode::StringRaw(raw) => raw.clone(),
        JsonNode::Array(items) => {
            if items.is_empty() {
                return JSON_EMPTY_ARRAY.to_string();
            }
            let parts: Vec<String> = items.iter().map(render_json_inline).collect();
            format!("[{}]", parts.join(", "))
        }
        JsonNode::Object(entries) => {
            if entries.is_empty() {
                return JSON_EMPTY_OBJECT.to_string();
            }
            let parts: Vec<String> = entries
                .iter()
                .map(|(k, v): &(String, JsonNode)| format!("{k}: {}", render_json_inline(v)))
                .collect();
            format!("{{ {} }}", parts.join(", "))
        }
    }
}

/// Renders a JSON AST node using Prettier 3.7.4 group-breaking semantics (`tabWidth: 2`, `printWidth: 80`).
///
/// # Arguments
///
/// - `&JsonNode` - AST node to print.
/// - `usize` - Indentation column in spaces for multi-line children.
/// - `usize` - Current line column offset before this node.
/// - `&mut String` - Output buffer.
fn render_json_node(node: &JsonNode, indent: usize, current_col: usize, out: &mut String) {
    match node {
        JsonNode::Null => out.push_str(JSON_LITERAL_NULL),
        JsonNode::Bool(true) => out.push_str(JSON_LITERAL_TRUE),
        JsonNode::Bool(false) => out.push_str(JSON_LITERAL_FALSE),
        JsonNode::Number(raw) => out.push_str(raw),
        JsonNode::StringRaw(raw) => out.push_str(raw),
        JsonNode::Array(items) => {
            if items.is_empty() {
                out.push_str(JSON_EMPTY_ARRAY);
                return;
            }
            let inline: String = render_json_inline(node);
            if current_col + str_display_width(&inline) <= PRETTIER_PRINT_WIDTH {
                out.push_str(&inline);
                return;
            }
            let child_indent: usize = indent + PRETTIER_TAB_WIDTH;
            out.push('[');
            out.push(TEXT_LF);
            for (idx, item) in items.iter().enumerate() {
                push_spaces(out, child_indent);
                render_json_node(item, child_indent, child_indent, out);
                if idx + 1 < items.len() {
                    out.push(',');
                }
                out.push(TEXT_LF);
            }
            push_spaces(out, indent);
            out.push(']');
        }
        JsonNode::Object(entries) => {
            if entries.is_empty() {
                out.push_str(JSON_EMPTY_OBJECT);
                return;
            }
            let inline: String = render_json_inline(node);
            if current_col + str_display_width(&inline) <= PRETTIER_PRINT_WIDTH {
                out.push_str(&inline);
                return;
            }
            let child_indent: usize = indent + PRETTIER_TAB_WIDTH;
            out.push('{');
            out.push(TEXT_LF);
            for (idx, (key, val)) in entries.iter().enumerate() {
                push_spaces(out, child_indent);
                out.push_str(key);
                out.push_str(": ");
                let val_col: usize = child_indent + str_display_width(key) + 2;
                render_json_node(val, child_indent, val_col, out);
                if idx + 1 < entries.len() {
                    out.push(',');
                }
                out.push(TEXT_LF);
            }
            push_spaces(out, indent);
            out.push('}');
        }
    }
}
