/// Manifest file name every Cargo package root is looked up under.
pub const CARGO_TOML: &str = "Cargo.toml";

/// `[workspace]` table key in a Cargo manifest.
pub(crate) const TOML_WORKSPACE: &str = "workspace";

/// `[package]` table key in a Cargo manifest.
pub(crate) const TOML_PACKAGE: &str = "package";

/// `[package].version` key in a Cargo manifest.
pub(crate) const TOML_VERSION: &str = "version";

/// `[package].name` key in a Cargo manifest.
pub(crate) const TOML_NAME: &str = "name";

/// `[workspace.members]` array key in a Cargo manifest.
pub(crate) const TOML_MEMBERS: &str = "members";

/// `[package].publish` key in a Cargo manifest.
pub(crate) const TOML_PUBLISH_KEY: &str = "publish";

/// `[dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_DEPENDENCIES: &str = "dependencies";

/// `[dev-dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_DEV_DEPENDENCIES: &str = "dev-dependencies";

/// `[build-dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_BUILD_DEPENDENCIES: &str = "build-dependencies";

/// `[target]` table key in a Cargo manifest.
pub(crate) const TOML_TARGET: &str = "target";

/// `path` key of a local dependency entry in a Cargo manifest.
pub(crate) const TOML_PATH: &str = "path";

/// `bump` subcommand name accepted by the command-line interface.
pub(crate) const CLI_BUMP: &str = "bump";

/// `publish` subcommand name accepted by the command-line interface.
pub(crate) const CLI_PUBLISH: &str = "publish";

/// `sync` subcommand name accepted by the command-line interface.
pub(crate) const CLI_SYNC: &str = "sync";

/// `-h` / `--help` flag of the command-line interface.
pub(crate) const CLI_FLAG_HELP: &str = "--help";

/// `-v` / `--version` flag of the command-line interface.
pub(crate) const CLI_FLAG_VERSION: &str = "--version";

/// `--patch` flag selecting a patch version bump.
pub(crate) const CLI_FLAG_PATCH: &str = "--patch";

/// `--minor` flag selecting a minor version bump.
pub(crate) const CLI_FLAG_MINOR: &str = "--minor";

/// `--major` flag selecting a major version bump.
pub(crate) const CLI_FLAG_MAJOR: &str = "--major";

/// `--release` flag dropping any pre-release identifier.
pub(crate) const CLI_FLAG_RELEASE: &str = "--release";

/// `--alpha` flag selecting an `alpha` pre-release bump.
pub(crate) const CLI_FLAG_ALPHA: &str = "--alpha";

/// `--beta` flag selecting a `beta` pre-release bump.
pub(crate) const CLI_FLAG_BETA: &str = "--beta";

/// `--rc` flag selecting an `rc` pre-release bump.
pub(crate) const CLI_FLAG_RC: &str = "--rc";

/// `--check` flag switching a subcommand into check-only mode.
pub(crate) const CLI_FLAG_CHECK: &str = "--check";

/// `--manifest-path` flag naming the manifest to operate on.
pub(crate) const CLI_FLAG_MANIFEST_PATH: &str = "--manifest-path";

/// `--max-retries` flag bounding publish retry attempts.
pub(crate) const CLI_FLAG_MAX_RETRIES: &str = "--max-retries";

/// `--fix` flag passed to `cargo clippy`.
pub(crate) const CLI_FLAG_FIX: &str = "--fix";

/// `--workspace` flag passed to `cargo clippy`.
pub(crate) const CLI_FLAG_WORKSPACE: &str = "--workspace";

/// `--all-targets` flag passed to `cargo clippy`.
pub(crate) const CLI_FLAG_ALL_TARGETS: &str = "--all-targets";

/// `--allow-dirty` flag passed to `cargo clippy` and `cargo publish`.
pub(crate) const CLI_FLAG_ALLOW_DIRTY: &str = "--allow-dirty";

/// `--no-verify` flag passed to `cargo publish`.
pub(crate) const CLI_FLAG_NO_VERIFY: &str = "--no-verify";

/// Cargo executable spawned as a child process.
pub(crate) const CARGO: &str = "cargo";

/// Cargo subcommand that reformats the workspace.
pub(crate) const CARGO_FMT: &str = "fmt";

/// Cargo subcommand that uploads a package to the registry.
pub(crate) const CARGO_PUBLISH: &str = "publish";

/// `clippy` subcommand / component name.
pub(crate) const CLIPPY: &str = "clippy";

/// Standalone `cargo-clippy` executable probed on `PATH`.
pub(crate) const CARGO_CLIPPY: &str = "cargo-clippy";

/// Rust toolchain manager executable spawned as a child process.
pub(crate) const RUSTUP: &str = "rustup";

/// `component` subcommand of the Rust toolchain manager.
pub(crate) const RUSTUP_COMPONENT: &str = "component";

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
