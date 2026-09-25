/// Errors that can occur during version parsing and comparison.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VersionError {
    /// An error occurred during parsing, with a specific message.
    ParseError(String),
    /// The format of the version range is invalid.
    InvalidRangeFormat,
    /// An error occurred while parsing the major version component.
    MajorVersionError,
    /// An error occurred while parsing the minor version component.
    MinorVersionError,
    /// An error occurred while parsing the patch version component.
    PatchVersionError,
}

/// Result of comparing two versions.
///
/// Indicates whether the first version is greater than, less than, or equal to the second version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VersionLevel {
    /// Indicates that the first version is greater than the second version.
    Greater,
    /// Indicates that the first version is less than the second version.
    Less,
    /// Indicates that the two versions are equal.
    Equal,
}
