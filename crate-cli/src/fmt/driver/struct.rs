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
