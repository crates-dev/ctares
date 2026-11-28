/// Supported text formatting strategy resolved from a file's name or extension.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileFormatKind {
    /// Rust source file formatted via derive sorting and `cargo fmt`.
    Rust,
    /// Markdown file formatted with VS Code Prettier 3.7.4 alignment.
    Markdown,
    /// YAML / YML file formatted with VS Code Prettier 3.7.4 alignment.
    Yaml,
    /// JSON file formatted with Prettier 3.7.4 `json` parser (`objectWrap: "preserve"`, comments allowed, no trailing commas).
    Json,
    /// JSON manifest (`package.json`, `package-lock.json`, `composer.json`) formatted with Prettier 3.7.4 `json-stringify` parser.
    JsonStringify,
    /// JSONC file (`.jsonc`, `.cjson`) formatted with Prettier 3.7.4 `jsonc` parser (comments and trailing commas preserved).
    Jsonc,
    /// TOML file formatted with VS Code `tamasfe.even-better-toml` (Taplo 0.7.0) defaults.
    Toml,
    /// JavaScript file (`.js`, `.mjs`, `.cjs`, `.wxs`) formatted with Prettier 3.7.4.
    JavaScript,
    /// TypeScript file (`.ts`, `.mts`, `.cts`) formatted with Prettier 3.7.4.
    TypeScript,
    /// TypeScript JSX file (`.tsx`) formatted with Prettier 3.7.4.
    TypeScriptReact,
    /// CSS stylesheet (`.css`, `.wxss`) formatted with Prettier 3.7.4.
    Css,
    /// SCSS stylesheet (`.scss`) formatted with Prettier 3.7.4.
    Scss,
    /// Less stylesheet (`.less`) formatted with Prettier 3.7.4.
    Less,
    /// HTML document (`.html`, `.htm`) formatted with Prettier 3.7.4.
    Html,
    /// Vue Single-File Component (`.vue`) formatted with Prettier 3.7.4.
    Vue,
    /// Generic UTF-8 text/script/config file left verbatim unchanged to match VS Code save behavior.
    PlainText,
    /// Generic UTF-8 text file with syntactically meaningful leading tabs (for example `Makefile`), left verbatim unchanged to match VS Code save behavior.
    PlainTextPreserveTabs,
    /// Binary, lockfile, certificate, or unsupported file left untouched.
    Unsupported,
}
