/// Supported text formatting strategy resolved from a file's name or extension.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileFormatKind {
    /// Rust source file formatted via derive sorting and `cargo fmt`.
    Rust,
    /// Markdown file formatted with VS Code Prettier 3.7.4 alignment.
    Markdown,
    /// YAML / YML file formatted with VS Code Prettier 3.7.4 alignment.
    Yaml,
    /// Strict JSON file formatted with Prettier 2-space / 80-column rules.
    Json,
    /// JSONC file formatted with comment-preserving 2-space indentation.
    Jsonc,
    /// TOML file formatted with Taplo/Cargo conventions.
    Toml,
    /// Generic UTF-8 text/script/config file normalized to LF and clean trailing whitespace.
    PlainText,
    /// Generic UTF-8 text file whose leading tabs are syntactically meaningful (for example `Makefile`).
    PlainTextPreserveTabs,
    /// Binary, lockfile, certificate, or unsupported file left untouched.
    Unsupported,
}
