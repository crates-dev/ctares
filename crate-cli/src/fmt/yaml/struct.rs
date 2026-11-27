use super::*;

/// Tracks one active YAML indentation scope mapping raw source indent to Prettier 2-space indent.
#[derive(Clone, Copy, Debug, Eq, Getter, New, PartialEq)]
pub(crate) struct YamlScope {
    /// Source indentation column of the block header or entry.
    #[get(pub(crate), type(copy))]
    pub(crate) raw_indent: usize,
    /// Normalized 2-space output indentation column.
    #[get(pub(crate), type(copy))]
    pub(crate) out_indent: usize,
    /// Whether this scope was introduced by a mapping key expecting a nested block value.
    #[get(pub(crate), type(copy))]
    pub(crate) expects_block_child: bool,
    /// Whether this scope represents a sequence `- ` item level.
    #[get(pub(crate), type(copy))]
    pub(crate) is_sequence: bool,
}
