use super::*;

/// Parsed version components following semantic versioning
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub struct Version {
    /// Major version number
    #[get(type(copy))]
    pub(crate) major: u64,
    /// Minor version number
    #[get(type(copy))]
    pub(crate) minor: u64,
    /// Patch version number
    #[get(type(copy))]
    pub(crate) patch: u64,
    /// Optional pre-release identifier (e.g. alpha, beta, rc.1)
    pub(crate) prerelease: Option<String>,
}
