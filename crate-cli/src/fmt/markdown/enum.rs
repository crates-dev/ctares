/// Column text alignment parsed from a GFM table delimiter row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TableAlign {
    /// Default alignment (`---`), left-padded like `Left`.
    None,
    /// Left alignment (`:---`).
    Left,
    /// Center alignment (`:---:`).
    Center,
    /// Right alignment (`---:`).
    Right,
}

/// Distinguishes consecutive list kinds so Prettier inserts a blank line between unordered and ordered lists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ListKind {
    /// Unordered bullet list (`-`, `*`, `+`).
    Unordered,
    /// Ordered numbered list (`1.`, `1)`).
    Ordered,
}

/// High-level category of the previously emitted Markdown block for Prettier inter-block spacing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MdBlockKind {
    /// No block emitted yet.
    None,
    /// YAML frontmatter block.
    Frontmatter,
    /// ATX heading (`# ...`).
    Heading,
    /// GFM pipe table.
    Table,
    /// Fenced code block.
    CodeFence,
    /// Blockquote or GitHub alert.
    Blockquote,
    /// Thematic break (`---`).
    ThematicBreak,
    /// Unordered or ordered list.
    List(ListKind),
    /// Raw HTML block line.
    Html,
    /// Regular prose paragraph.
    Paragraph,
}
