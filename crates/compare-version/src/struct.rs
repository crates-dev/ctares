/// Represents a semantic version with major, minor, patch, and pre-release components.
///
/// Follows semantic versioning specification (SemVer).
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Version {
    /// Major version number.
    pub(crate) major: u32,
    /// Minor version number.
    pub(crate) minor: u32,
    /// Patch version number.
    pub(crate) patch: u32,
    /// Optional pre-release identifier.
    pub(crate) pre_release: Option<String>,
}

/// Utility for comparing version strings.
///
/// Provides methods for version comparison and range matching.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompareVersion;
