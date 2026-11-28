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

/// Workspace ignore filename honored alongside `FMT_SKIP_DIRS` (matching VS Code Prettier).
pub(crate) const FMT_PRETTIER_IGNORE_FILE: &str = ".prettierignore";

/// File extension for Rust source files.
pub(crate) const FMT_EXT_RUST: &str = "rs";

/// File extensions formatted as Markdown (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_MARKDOWN: &[&str] = &["md", "markdown", "mdx"];

/// File extensions formatted as YAML (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_YAML: &[&str] = &["yml", "yaml"];

/// Exact filenames formatted via Prettier's YAML/JSON auto-detecting RC parser.
pub(crate) const FMT_NAME_YAML: &[&str] = &[".prettierrc"];

/// Exact filenames formatted with Prettier's `json-stringify` parser.
pub(crate) const FMT_NAME_JSON_STRINGIFY: &[&str] =
    &["package.json", "package-lock.json", "composer.json"];

/// File extensions formatted with Prettier's `json` parser.
pub(crate) const FMT_EXT_JSON: &[&str] = &["json"];

/// File extensions formatted with Prettier's `jsonc` parser.
pub(crate) const FMT_EXT_JSONC: &[&str] = &["jsonc", "cjson"];

/// File extensions formatted as TOML (VS Code `tamasfe.even-better-toml` / Taplo 0.7.0).
pub(crate) const FMT_EXT_TOML: &[&str] = &["toml"];

/// File extensions formatted as JavaScript (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_JAVASCRIPT: &[&str] = &["js", "mjs", "cjs", "wxs"];

/// File extensions formatted as TypeScript (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_TYPESCRIPT: &[&str] = &["ts", "mts", "cts"];

/// File extensions formatted as TypeScript JSX (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_TSX: &[&str] = &["tsx"];

/// File extensions formatted as CSS (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_CSS: &[&str] = &["css", "wxss"];

/// File extensions formatted as SCSS (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_SCSS: &[&str] = &["scss"];

/// File extensions formatted as Less (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_LESS: &[&str] = &["less"];

/// File extensions formatted as HTML (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_HTML: &[&str] = &["html", "htm"];

/// File extensions formatted as Vue SFC (VS Code Prettier 3.7.4).
pub(crate) const FMT_EXT_VUE: &[&str] = &["vue"];

/// File extensions classified as plain UTF-8 text (VS Code save no-op).
pub(crate) const FMT_EXT_PLAIN_TEXT: &[&str] = &[
    "sh",
    "bash",
    "zsh",
    "txt",
    "mk",
    "dockerfile",
    "env",
    "gitignore",
    "dockerignore",
    "editorconfig",
    "conf",
    "cfg",
    "ini",
    "properties",
    "xml",
    "svg",
];

/// Exact filenames classified as plain text with significant leading tabs (VS Code save no-op).
pub(crate) const FMT_NAME_PRESERVE_TABS: &[&str] = &["Makefile", "makefile", ".gitattributes"];

/// Exact extensionless or dotfile names classified as plain text (VS Code save no-op).
pub(crate) const FMT_NAME_PLAIN_TEXT: &[&str] = &[
    "Dockerfile",
    "LICENSE",
    ".gitignore",
    ".dockerignore",
    ".npmignore",
    ".prettierignore",
    ".eslintignore",
    ".vscodeignore",
    ".editorconfig",
    ".gitconfig",
    ".npmrc",
    ".env",
    ".env.example",
    ".bashrc",
    ".zshrc",
    "pre-commit",
    "commit-msg",
    "pre-push",
];

/// Filename prefix matching multi-dot Dockerfile variants (`Dockerfile.dev`, `Dockerfile.perf`).
pub(crate) const FMT_PREFIX_DOCKERFILE: &str = "Dockerfile.";

/// Filename prefix matching multi-dot environment files (`.env.local`, `.env.production`).
pub(crate) const FMT_PREFIX_DOTENV: &str = ".env.";

/// Error message emitted when `--check` detects unformatted text files.
pub(crate) const ERROR_TEXT_FMT_CHECK_FAILED: &str = "unformatted text files detected";
