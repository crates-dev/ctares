/// YAML document start marker.
pub(crate) const YAML_DOC_START: &str = "---";

/// YAML document end marker.
pub(crate) const YAML_DOC_END: &str = "...";

/// GitHub Actions template expression open delimiter preserved verbatim.
pub(crate) const YAML_EXPR_OPEN: &str = "${{";

/// GitHub Actions template expression close delimiter preserved verbatim.
pub(crate) const YAML_EXPR_CLOSE: &str = "}}";
