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
