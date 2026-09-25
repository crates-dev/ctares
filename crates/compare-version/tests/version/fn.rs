use super::*;

#[test]
fn test_compare_version() {
    let version1: &str = "1.2.3-alpha";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Equal);
    let version1: &str = "1.2.3";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Less);
    let version1: &str = "1.2.2";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Less);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Greater);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.3";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Greater);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.5";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Less);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.4";
    let result: Result<VersionLevel, VersionError> =
        CompareVersion::compare_version(version1, version2);
    assert_eq!(result.unwrap(), VersionLevel::Equal);
}

#[test]
fn test_matches_version_range() {
    let version: &str = "1.2.3-alpha";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = CompareVersion::matches_version_range(version, range1).unwrap_or_default();
    assert!(result);
    let result: bool = CompareVersion::matches_version_range(version, range2).unwrap_or_default();
    assert!(result);
    let version: &str = "1.2.5-alpha";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = CompareVersion::matches_version_range(version, range1).unwrap_or_default();
    assert!(result);
    let result: bool = CompareVersion::matches_version_range(version, range2).unwrap_or_default();
    assert!(result);
    let version: &str = "1.3.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = CompareVersion::matches_version_range(version, range1).unwrap_or_default();
    assert!(result);
    let result: bool = CompareVersion::matches_version_range(version, range2).unwrap_or_default();
    assert!(!result);
    let version: &str = "2.0.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = CompareVersion::matches_version_range(version, range1).unwrap_or_default();
    assert!(!result);
    let result: bool = CompareVersion::matches_version_range(version, range2).unwrap_or_default();
    assert!(!result);
    let version: &str = "1.0.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = CompareVersion::matches_version_range(version, range1).unwrap_or_default();
    assert!(!result);
    let result: bool = CompareVersion::matches_version_range(version, range2).unwrap_or_default();
    assert!(!result);
}
