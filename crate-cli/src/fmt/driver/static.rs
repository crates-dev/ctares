use super::*;

/// Regex pattern to match derive attribute
///
/// This pattern matches `#[derive(...)]` attributes in Rust code.
pub(crate) static DERIVE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| regex::Regex::new(DERIVE_PATTERN).expect(ERROR_DERIVE_PATTERN_INVALID));
