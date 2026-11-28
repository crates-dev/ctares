use super::*;

/// Structured summary returned by unified workspace text formatting.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct UnifiedFmtReport {
    /// Total number of supported text files scanned.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(skip)]
    pub(crate) files_scanned: usize,
    /// Number of files modified (or needing modification in `--check` mode).
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(skip)]
    pub(crate) files_formatted: usize,
    /// Whether formatting ran in check-only mode.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(skip)]
    pub(crate) check_mode: bool,
    /// Relative or display paths of files that were changed or failed `--check`.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) changed_files: Vec<String>,
}

/// Single parsed rule from a workspace-root `.prettierignore` file.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub(crate) struct PrettierIgnoreRule {
    /// Raw normalized glob pattern (with leading `!`/`/` and trailing `/` stripped).
    #[get(pub(crate))]
    pub(crate) pattern: String,
    /// Whether this rule negates (`!`) an earlier ignore match.
    #[get(pub(crate), type(copy))]
    pub(crate) negated: bool,
    /// Whether this rule applies only to directories (trailing `/`).
    #[get(pub(crate), type(copy))]
    pub(crate) dir_only: bool,
    /// Whether this rule matches against the full relative path rather than a single segment.
    #[get(pub(crate), type(copy))]
    pub(crate) anchored: bool,
}
