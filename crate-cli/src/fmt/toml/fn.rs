use super::*;

/// Formats TOML files cleanly while preserving key order, dependency grouping, and Taplo/Cargo.toml 4-space array indentation.
///
/// # Arguments
///
/// - `&str` - Raw TOML source text.
///
/// # Returns
///
/// - `String` - Normalized TOML text ending with a single `\n`.
pub fn format_toml(input: &str) -> String {
    let normalized: String = normalize_line_endings(input);
    let mut out_lines: Vec<String> = Vec::new();
    let mut bracket_depth: usize = 0;
    let mut in_multiline_double: bool = false;
    let mut in_multiline_single: bool = false;
    let mut prev_blank: bool = false;
    for raw_line in normalized.split(TEXT_LF) {
        if in_multiline_double || in_multiline_single {
            let trimmed_end: &str = raw_line.trim_end_matches([TEXT_SPACE, TEXT_TAB]);
            out_lines.push(trimmed_end.to_string());
            if in_multiline_double && trimmed_end.contains(TOML_TRIPLE_DOUBLE) {
                in_multiline_double = false;
            }
            if in_multiline_single && trimmed_end.contains(TOML_TRIPLE_SINGLE) {
                in_multiline_single = false;
            }
            prev_blank = false;
            continue;
        }
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() {
            if !out_lines.is_empty() && !prev_blank {
                out_lines.push(String::new());
                prev_blank = true;
            }
            continue;
        }
        prev_blank = false;
        let leading_close: bool = trimmed.starts_with(']');
        let line_depth: usize = if leading_close {
            bracket_depth.saturating_sub(1)
        } else {
            bracket_depth
        };
        let normalized_line: String = if line_depth == 0 {
            normalize_toml_assignment(trimmed)
        } else {
            trimmed.to_string()
        };
        let mut line: String =
            String::with_capacity(line_depth * TOML_ARRAY_INDENT + normalized_line.len());
        push_spaces(&mut line, line_depth * TOML_ARRAY_INDENT);
        line.push_str(&normalized_line);
        out_lines.push(line);
        let (delta, opens_double, opens_single): (isize, bool, bool) =
            scan_toml_line_state(&normalized_line);
        let updated: isize = (bracket_depth as isize) + delta;
        bracket_depth = if updated < 0 { 0 } else { updated as usize };
        in_multiline_double = opens_double;
        in_multiline_single = opens_single;
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

/// Normalizes spacing around the top-level `=` in a TOML `key = value` line without altering strings or inline tables.
///
/// # Arguments
///
/// - `&str` - Trimmed TOML line.
///
/// # Returns
///
/// - `String` - Normalized line.
fn normalize_toml_assignment(line: &str) -> String {
    if line.starts_with('[') || line.starts_with('#') {
        return line.to_string();
    }
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    let mut in_double: bool = false;
    let mut in_single: bool = false;
    while i < len {
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
        if in_single {
            if bytes[i] == b'\'' {
                in_single = false;
            }
            i += 1;
            continue;
        }
        match bytes[i] {
            b'"' => in_double = true,
            b'\'' => in_single = true,
            b'#' => return line.to_string(),
            b'=' => {
                let lhs: &str = line[..i].trim_end();
                let rhs: &str = line[(i + 1)..].trim_start();
                return format!("{lhs} = {rhs}");
            }
            _ => {}
        }
        i += 1;
    }
    line.to_string()
}

/// Scans a single TOML line for multi-line array bracket delta and unclosed triple-quoted strings.
///
/// # Arguments
///
/// - `&str` - Trimmed TOML line.
///
/// # Returns
///
/// - `(isize, bool, bool)` - `(bracket_delta, opens_multiline_double, opens_multiline_single)`.
fn scan_toml_line_state(line: &str) -> (isize, bool, bool) {
    let bytes: &[u8] = line.as_bytes();
    let len: usize = bytes.len();
    if line.starts_with('[') && !line.contains('=') {
        return (0, false, false);
    }
    let mut i: usize = 0;
    let mut delta: isize = 0;
    let mut in_double: bool = false;
    let mut in_single: bool = false;
    let mut multi_double: bool = false;
    let mut multi_single: bool = false;
    while i < len {
        if multi_double {
            if bytes[i..].starts_with(TOML_TRIPLE_DOUBLE.as_bytes()) {
                multi_double = false;
                i += 3;
            } else {
                i += 1;
            }
            continue;
        }
        if multi_single {
            if bytes[i..].starts_with(TOML_TRIPLE_SINGLE.as_bytes()) {
                multi_single = false;
                i += 3;
            } else {
                i += 1;
            }
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
        if in_single {
            if bytes[i] == b'\'' {
                in_single = false;
            }
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(TOML_TRIPLE_DOUBLE.as_bytes()) {
            multi_double = true;
            i += 3;
            continue;
        }
        if bytes[i..].starts_with(TOML_TRIPLE_SINGLE.as_bytes()) {
            multi_single = true;
            i += 3;
            continue;
        }
        match bytes[i] {
            b'"' => in_double = true,
            b'\'' => in_single = true,
            b'#' => break,
            b'[' => delta += 1,
            b']' => delta -= 1,
            _ => {}
        }
        i += 1;
    }
    (delta, multi_double, multi_single)
}
