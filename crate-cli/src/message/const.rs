/// `alpha` pre-release identifier applied by a version bump.
pub(crate) const PRERELEASE_ALPHA: &str = "alpha";

/// `beta` pre-release identifier applied by a version bump.
pub(crate) const PRERELEASE_BETA: &str = "beta";

/// Literal prefix of a Rust `#[derive(...)]` attribute.
pub(crate) const DERIVE_PREFIX: &str = "#[derive(";

/// Regular expression capturing the trait list of a `#[derive(...)]` attribute.
pub(crate) const DERIVE_PATTERN: &str = r"#\[derive\s*\(([^)]+)\)\]";

/// Panic message when the derive-pattern regular expression fails to compile.
pub(crate) const ERROR_DERIVE_PATTERN_INVALID: &str = "Invalid regex pattern";

/// Error message when installing the clippy component fails.
pub(crate) const ERROR_CLIPPY_INSTALL_FAILED: &str = "failed to install cargo-clippy";

/// Error message when `cargo clippy --fix` exits non-zero.
pub(crate) const ERROR_CLIPPY_FIX_FAILED: &str = "cargo clippy --fix failed";

/// Error message when `cargo fmt` exits non-zero.
pub(crate) const ERROR_FMT_FAILED: &str = "cargo fmt failed";

/// Error message when `[workspace.package].version` is absent from a manifest.
pub(crate) const ERROR_WORKSPACE_PACKAGE_VERSION_MISSING: &str =
    "workspace.package.version not found";

/// Error message when `[package].version` is absent from a manifest.
pub(crate) const ERROR_PACKAGE_VERSION_MISSING: &str = "package.version not found";

/// Error message when a manifest declares neither version location.
pub(crate) const ERROR_NO_VERSION_SLOT: &str =
    "neither [package] nor [workspace.package] found in Cargo.toml";

/// Error message when a manifest version slot is not a string.
pub(crate) const ERROR_VERSION_NOT_STRING: &str = "version field is not a string";

/// Report text emitted when a workspace declares no members.
pub(crate) const REPORT_ZERO_WORKSPACE_MEMBERS: &str = "0 workspace members";

/// Report placeholder used when no shared workspace version exists.
pub(crate) const REPORT_PER_MEMBER_VERSION: &str = "per-member";

/// `cargo publish` stderr fragment for an already uploaded package.
pub(crate) const STDERR_ALREADY_BEEN_UPLOADED: &str = "already been uploaded";

/// `cargo publish` stderr fragment for an already published package.
pub(crate) const STDERR_IS_ALREADY_PUBLISHED: &str = "is already published";

/// `cargo publish` stderr fragment for a version already on the registry index.
pub(crate) const STDERR_ALREADY_ON_INDEX: &str = "already exists on crates.io index";

/// `cargo publish` stderr fragment for a registry rate-limit refusal.
pub(crate) const STDERR_TOO_MANY_REQUESTS: &str = "429 Too Many Requests";

/// `cargo publish` stderr fragment naming the new-crate rate limit.
pub(crate) const STDERR_TOO_MANY_NEW_CRATES: &str =
    "published too many new crates in a short period of time";

/// `cargo publish` stderr fragment introducing the retry deadline.
pub(crate) const STDERR_TRY_AGAIN_AFTER: &str = "Please try again after ";

/// `cargo publish` stderr fragment ending the retry deadline sentence.
pub(crate) const STDERR_TRY_AGAIN_AFTER_END: &str = " GMT";
