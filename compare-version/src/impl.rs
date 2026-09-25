use super::*;

/// Implements the `Display` trait for `VersionError` to provide user-friendly error messages.
impl fmt::Display for VersionError {
    /// Formats the `VersionError` into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter` - Formatter to write the output to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            VersionError::InvalidRangeFormat => {
                write!(f, "Unsupported range format, only '^' or '~' are supported")
            }
            VersionError::MajorVersionError => write!(f, "Major version parsing error"),
            VersionError::MinorVersionError => write!(f, "Minor version parsing error"),
            VersionError::PatchVersionError => write!(f, "Patch version parsing error"),
        }
    }
}

impl Version {
    /// Parses a version from a string.
    ///
    /// # Arguments
    ///
    /// - `&str` - A string slice that holds the version in the format 'x.y.z'.
    ///
    /// # Returns
    ///
    /// - `Result<Self, VersionError>` - A `Result` indicating the parsed `Version` struct on success, or a `VersionError` on failure.
    pub(crate) fn parse(version: &str) -> Result<Self, VersionError> {
        let mut parts: Vec<&str> = version.split('.').collect();
        let (patch_part, pre_release) = if let Some(patch_with_prerelease) = parts.pop() {
            let mut patch_parts = patch_with_prerelease.splitn(2, '-');
            (
                patch_parts.next().unwrap_or(""),
                patch_parts.next().map(|part: &str| part.to_string()),
            )
        } else {
            return Err(VersionError::ParseError(
                "Version format error, should be in the form 'x.y.z'.".to_string(),
            ));
        };
        let major: u32 = parts
            .first()
            .unwrap_or(&"0")
            .parse::<u32>()
            .map_err(|_| VersionError::MajorVersionError)?;
        let minor: u32 = parts
            .get(1)
            .unwrap_or(&"0")
            .parse::<u32>()
            .map_err(|_| VersionError::MinorVersionError)?;
        let patch: u32 = patch_part
            .parse::<u32>()
            .map_err(|_| VersionError::PatchVersionError)?;
        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }
}

impl CompareVersion {
    /// Compares two version strings and returns a `VersionLevel` enum.
    ///
    /// # Arguments
    ///
    /// - `&str` - The first version string to compare.
    /// - `&str` - The second version string to compare.
    ///
    /// # Returns
    ///
    /// - `Result<VersionLevel, VersionError>` - A `Result` indicating the comparison result on success, or a `VersionError` on failure.
    pub fn compare_version(version1: &str, version2: &str) -> Result<VersionLevel, VersionError> {
        let v1: Version = Version::parse(version1)?;
        let v2: Version = Version::parse(version2)?;
        match v1.cmp(&v2) {
            std::cmp::Ordering::Greater => Ok(VersionLevel::Greater),
            std::cmp::Ordering::Less => Ok(VersionLevel::Less),
            std::cmp::Ordering::Equal => Ok(VersionLevel::Equal),
        }
    }

    /// Checks whether a version matches a specified range, supporting `^` and `~` ranges.
    ///
    /// # Arguments
    ///
    /// - `&str` - The version string to check.
    /// - `&str` - The version range string to match against.
    ///
    /// # Returns
    ///
    /// - `Result<bool, VersionError>` - A `Result` indicating whether the version matches the range on success, or a `VersionError` on failure.
    pub fn matches_version_range(version: &str, range: &str) -> Result<bool, VersionError> {
        let target_version: Version = Version::parse(version)?;
        if let Some(stripped_range) = range.strip_prefix('^') {
            let base_version = Version::parse(stripped_range)?;
            // `^` indicates major version compatibility
            Ok(target_version.major == base_version.major
                && (target_version.minor > base_version.minor
                    || (target_version.minor == base_version.minor
                        && target_version >= base_version)))
        } else if let Some(stripped_range) = range.strip_prefix('~') {
            let base_version: Version = Version::parse(stripped_range)?;
            // `~` indicates minor version compatibility
            Ok(target_version.major == base_version.major
                && target_version.minor == base_version.minor
                && target_version >= base_version)
        } else {
            Err(VersionError::InvalidRangeFormat)
        }
    }
}
