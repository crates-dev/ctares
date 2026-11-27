use super::*;

/// Parsed metadata of an opening fenced code block delimiter.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub(crate) struct FenceOpen {
    /// Leading indentation in spaces before the opening fence.
    #[get(pub(crate), type(copy))]
    pub(crate) indent: usize,
    /// Fence character (`` ` `` or `~`).
    #[get(pub(crate), type(copy))]
    pub(crate) fence_char: char,
    /// Number of fence characters (`>= 3`).
    #[get(pub(crate), type(copy))]
    pub(crate) fence_len: usize,
    /// Trimmed info string (language identifier and optional metadata).
    #[get(pub(crate))]
    pub(crate) info: String,
}

/// Parsed metadata of a single Markdown list item line.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub(crate) struct ListItemLine {
    /// Raw leading indentation of the marker in spaces.
    #[get(pub(crate), type(copy))]
    pub(crate) raw_indent: usize,
    /// List type (`Unordered` or `Ordered`).
    #[get(pub(crate), type(copy))]
    pub(crate) kind: ListKind,
    /// Normalized list marker (`"-"` or `"1."`).
    #[get(pub(crate))]
    pub(crate) marker: String,
    /// Trimmed content following the marker.
    #[get(pub(crate))]
    pub(crate) body: String,
}
