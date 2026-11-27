use super::*;

/// Parsed command line arguments
#[derive(Clone, Debug, Getter, New)]
pub struct Args {
    /// The command to execute
    pub(crate) command: CommandType,
    /// Check mode for fmt
    #[get(type(copy))]
    pub(crate) check: bool,
    /// Manifest path for fmt, bump, publish and sync
    pub(crate) manifest_path: Option<String>,
    /// Bump type for bump command
    pub(crate) bump_type: Option<BumpVersionType>,
    /// Maximum retry attempts for publish command
    #[get(type(copy))]
    pub(crate) max_retries: u32,
}
