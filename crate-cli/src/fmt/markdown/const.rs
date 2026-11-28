/// YAML frontmatter fence delimiter at the start of a Markdown document.
pub(crate) const MD_FRONTMATTER_YAML: &str = "---";

/// TOML frontmatter fence delimiter at the start of a Markdown document.
pub(crate) const MD_FRONTMATTER_TOML: &str = "+++";

/// Opening/closing triple-asterisk emphasis delimiter (`***`).
pub(crate) const MD_TRIPLE_STAR: &str = "***";

/// Opening/closing triple-underscore emphasis delimiter (`___`).
pub(crate) const MD_TRIPLE_UNDER: &str = "___";

/// Backtick fenced code block prefix (` ``` `).
pub(crate) const MD_FENCE_BACKTICK: &str = "```";

/// Tilde fenced code block prefix (`~~~`).
pub(crate) const MD_FENCE_TILDE: &str = "~~~";

/// Prettier-aligned opening delimiter for bold-italic (`**_..._**`).
pub(crate) const MD_PRETTIER_BOLD_ITALIC_OPEN: &str = "**_";

/// Prettier-aligned closing delimiter for bold-italic (`**_..._**`).
pub(crate) const MD_PRETTIER_BOLD_ITALIC_CLOSE: &str = "_**";

/// Maximum recursion depth for embedded `markdown` / `md` fenced code blocks.
pub(crate) const MD_MAX_EMBED_DEPTH: usize = 4;

/// Embedded YAML fence info languages formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_YAML: &[&str] = &["yaml", "yml"];

/// Embedded JSON fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_JSON: &str = "json";

/// Embedded JSONC fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_JSONC: &str = "jsonc";

/// Embedded JSON5 fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_JSON5: &str = "json5";

/// Embedded JavaScript fence info languages formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_JS: &[&str] = &["js", "javascript", "mjs", "cjs", "jsx"];

/// Embedded TypeScript fence info languages formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_TS: &[&str] = &["ts", "typescript", "mts", "cts", "tsx"];

/// Embedded CSS fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_CSS: &str = "css";

/// Embedded SCSS fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_SCSS: &str = "scss";

/// Embedded Less fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_LESS: &str = "less";

/// Embedded HTML fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_HTML: &str = "html";

/// Embedded Vue fence info language formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_VUE: &str = "vue";

/// Embedded Markdown fence info languages formatted by Prettier 3.7.4.
pub(crate) const MD_EMBED_MARKDOWN: &[&str] = &["markdown", "md"];

/// Synthetic path hint for embedded CommonJS fenced code blocks.
pub(crate) const MD_HINT_CJS: &str = "code.cjs";

/// Synthetic path hint for embedded JSX fenced code blocks.
pub(crate) const MD_HINT_JSX: &str = "code.jsx";

/// Synthetic path hint for embedded JavaScript fenced code blocks.
pub(crate) const MD_HINT_JS: &str = "code.js";

/// Synthetic path hint for embedded CommonJS TypeScript fenced code blocks.
pub(crate) const MD_HINT_CTS: &str = "code.cts";

/// Synthetic path hint for embedded TSX fenced code blocks.
pub(crate) const MD_HINT_TSX: &str = "code.tsx";

/// Synthetic path hint for embedded TypeScript fenced code blocks.
pub(crate) const MD_HINT_TS: &str = "code.ts";

/// Synthetic path hint for embedded Vue fenced code blocks.
pub(crate) const MD_HINT_VUE: &str = "code.vue";
