use super::*;

/// Normalizes CRLF and lone CR line endings to LF (`\n`).
///
/// # Arguments
///
/// - `&str` - Raw input text with arbitrary line endings.
///
/// # Returns
///
/// - `String` - Text with LF-only line endings.
pub fn normalize_line_endings(input: &str) -> String {
    if !input.contains(TEXT_CR) {
        return input.to_string();
    }
    let replaced_crlf: String = input.replace("\r\n", "\n");
    replaced_crlf.replace(TEXT_CR, "\n")
}

/// Formats generic plain text files with LF line endings, trimmed trailing whitespace, and a single trailing newline.
///
/// # Arguments
///
/// - `&str` - Raw file content.
/// - `bool` - When `true`, preserves leading tabs (for example in `Makefile`) while stripping trailing whitespace.
///
/// # Returns
///
/// - `String` - Normalized text content.
pub fn format_plain_text(input: &str, preserve_tabs: bool) -> String {
    let normalized: String = normalize_line_endings(input);
    let mut lines: Vec<String> = Vec::new();
    for raw_line in normalized.split(TEXT_LF) {
        let trimmed: &str = raw_line.trim_end_matches([TEXT_SPACE, TEXT_TAB]);
        if preserve_tabs {
            lines.push(trimmed.to_string());
        } else {
            lines.push(expand_leading_tabs(trimmed));
        }
    }
    while let Some(last) = lines.last() {
        if last.is_empty() {
            let _: Option<String> = lines.pop();
        } else {
            break;
        }
    }
    if lines.is_empty() {
        return String::new();
    }
    let mut out: String = lines.join("\n");
    out.push(TEXT_LF);
    out
}

/// Expands leading tab characters into `PRETTIER_TAB_WIDTH` spaces while keeping interior characters intact.
///
/// # Arguments
///
/// - `&str` - Single line without trailing whitespace.
///
/// # Returns
///
/// - `String` - Line with leading tabs converted to spaces.
fn expand_leading_tabs(line: &str) -> String {
    if !line.starts_with(TEXT_TAB) {
        return line.to_string();
    }
    let mut out: String = String::with_capacity(line.len() + 8);
    let mut in_leading: bool = true;
    for ch in line.chars() {
        if in_leading && ch == TEXT_TAB {
            push_spaces(&mut out, PRETTIER_TAB_WIDTH);
        } else {
            in_leading = false;
            out.push(ch);
        }
    }
    out
}

/// Computes the terminal/Prettier column display width of a single Unicode scalar value.
///
/// # Arguments
///
/// - `char` - Unicode character to measure.
///
/// # Returns
///
/// - `usize` - Display width (`0` for combining marks, `2` for East Asian wide/fullwidth/emoji characters, `1` otherwise).
pub fn char_display_width(ch: char) -> usize {
    let code: u32 = ch as u32;
    if code < 0x20 || (0x7F..0xA0).contains(&code) {
        return 0;
    }
    if (0x0300..=0x036F).contains(&code)
        || (0x1AB0..=0x1AFF).contains(&code)
        || (0x1DC0..=0x1DFF).contains(&code)
        || (0x20D0..=0x20FF).contains(&code)
        || (0xFE00..=0xFE0F).contains(&code)
        || (0xFE20..=0xFE2F).contains(&code)
        || code == 0x200D
    {
        return 0;
    }
    if is_wide_codepoint(code) {
        return 2;
    }
    1
}

/// Computes the Prettier column display width of a string (CJK and emoji wide characters count as 2).
///
/// # Arguments
///
/// - `&str` - String slice to measure.
///
/// # Returns
///
/// - `usize` - Total column display width.
pub fn str_display_width(text: &str) -> usize {
    let mut width: usize = 0;
    for ch in text.chars() {
        width += char_display_width(ch);
    }
    width
}

/// Returns `true` when `code` falls inside East Asian Wide, Fullwidth, or wide Emoji Unicode ranges.
///
/// # Arguments
///
/// - `u32` - Unicode code point.
///
/// # Returns
///
/// - `bool` - Whether the character occupies two display columns.
fn is_wide_codepoint(code: u32) -> bool {
    (0x1100..=0x115F).contains(&code)
        || (0x2329..=0x232A).contains(&code)
        || (0x2E80..=0x303E).contains(&code)
        || (0x3040..=0xA4CF).contains(&code)
        || (0xAC00..=0xD7A3).contains(&code)
        || (0xF900..=0xFAFF).contains(&code)
        || (0xFE10..=0xFE19).contains(&code)
        || (0xFE30..=0xFE6F).contains(&code)
        || (0xFF00..=0xFF60).contains(&code)
        || (0xFFE0..=0xFFE6).contains(&code)
        || (0x1F300..=0x1FAFF).contains(&code)
        || (0x20000..=0x2FFFD).contains(&code)
        || (0x30000..=0x3FFFD).contains(&code)
}

/// Repeats spaces `count` times into `out`.
///
/// # Arguments
///
/// - `&mut String` - Destination string buffer.
/// - `usize` - Number of ASCII space characters to append.
pub(crate) fn push_spaces(out: &mut String, count: usize) {
    for _ in 0..count {
        out.push(TEXT_SPACE);
    }
}
