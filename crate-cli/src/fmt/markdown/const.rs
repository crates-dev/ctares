/// YAML frontmatter fence delimiter at the start of a Markdown document.
pub(crate) const MD_FRONTMATTER_FENCE: &str = "---";

/// Normalized thematic break emitted by Prettier for `***`, `___`, and `---`.
pub(crate) const MD_THEMATIC_BREAK: &str = "---";

/// Minimum dash count in a Prettier GFM table separator column.
pub(crate) const MD_TABLE_MIN_COL_WIDTH: usize = 3;

/// Language identifier for embedded JSON fenced code blocks.
pub(crate) const MD_LANG_JSON: &str = "json";

/// Language identifier for embedded JSONC fenced code blocks.
pub(crate) const MD_LANG_JSONC: &str = "jsonc";

/// Language identifier for embedded YAML fenced code blocks.
pub(crate) const MD_LANG_YAML: &str = "yaml";

/// Short language identifier for embedded YML fenced code blocks.
pub(crate) const MD_LANG_YML: &str = "yml";
