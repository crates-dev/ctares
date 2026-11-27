/// Directory names skipped during recursive workspace text formatting.
pub(crate) const FMT_SKIP_DIRS: &[&str] = &[
    ".git",
    ".worktrees",
    ".codegraph",
    ".cache",
    "target",
    "node_modules",
    "dist",
    "tmp",
    "results",
    "results2",
    "data",
    "logs",
    "uploads",
    "debug",
    "pkg",
];

/// File extensions formatted as Markdown (VS Code Prettier-aligned).
pub(crate) const FMT_EXT_MARKDOWN: &[&str] = &["md", "markdown", "mdx"];

/// File extensions formatted as YAML (VS Code Prettier-aligned).
pub(crate) const FMT_EXT_YAML: &[&str] = &["yml", "yaml"];

/// File extensions formatted as strict JSON (VS Code Prettier-aligned).
pub(crate) const FMT_EXT_JSON: &[&str] = &["json"];

/// File extensions formatted as JSONC with comment preservation.
pub(crate) const FMT_EXT_JSONC: &[&str] = &["jsonc"];

/// File extensions formatted as TOML.
pub(crate) const FMT_EXT_TOML: &[&str] = &["toml"];

/// File extensions normalized as plain UTF-8 text (LF, trimmed trailing whitespace, single EOF newline).
pub(crate) const FMT_EXT_PLAIN_TEXT: &[&str] = &[
    "sh",
    "bash",
    "zsh",
    "py",
    "sql",
    "txt",
    "css",
    "scss",
    "less",
    "html",
    "htm",
    "xml",
    "svg",
    "js",
    "mjs",
    "cjs",
    "ts",
    "tsx",
    "jsx",
    "vue",
    "dockerfile",
    "env",
    "gitignore",
    "dockerignore",
    "editorconfig",
];

/// Exact filenames normalized as plain text with leading tabs preserved.
pub(crate) const FMT_NAME_PRESERVE_TABS: &[&str] = &["Makefile", "makefile", ".gitattributes"];

/// Exact extensionless or dotfile names normalized as plain text.
pub(crate) const FMT_NAME_PLAIN_TEXT: &[&str] = &[
    "Dockerfile",
    "LICENSE",
    ".gitignore",
    ".dockerignore",
    ".editorconfig",
    ".env",
    ".env.example",
];

/// File extension for Rust source files.
pub(crate) const FMT_EXT_RUST: &str = "rs";

/// Error message emitted when `--check` detects unformatted text files.
pub(crate) const ERROR_TEXT_FMT_CHECK_FAILED: &str = "unformatted text files detected";
