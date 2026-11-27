/// Indentation width in spaces for multi-line TOML arrays matching Cargo.toml conventions.
pub(crate) const TOML_ARRAY_INDENT: usize = 4;

/// Triple-quote delimiter for multi-line basic TOML strings.
pub(crate) const TOML_TRIPLE_DOUBLE: &str = "\"\"\"";

/// Triple-quote delimiter for multi-line literal TOML strings.
pub(crate) const TOML_TRIPLE_SINGLE: &str = "'''";
