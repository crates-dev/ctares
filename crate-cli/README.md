<div align="center">

# crate-cli

**Unified Cargo Workspace CLI & VS Code Prettier/Taplo-Aligned Formatter**

[![crates.io](https://flat.badgen.net/crates/v/crate-cli)](https://crates.io/crates/crate-cli)
[![docs.rs](https://img.shields.io/docsrs/crate-cli?style=flat-square)](https://docs.rs/crate-cli)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/crate-cli/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Command-line tool and embeddable library managing Cargo workspace lifecycles (`crate bump`, `crate sync`, `crate publish`, `crate fmt`) plus pure-Rust formatting aligned with VS Code `esbenp.prettier-vscode` (Prettier 3.7.4), `tamasfe.even-better-toml` (Taplo 0.7.0), and `rustfmt`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Supported Formats & VS Code Parity Options

| Category                     | Files / Extensions                                                                                                                                                                                                              | Engine / Rules                                                                                                                                                                                                         |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Markdown**                 | `.md`, `.markdown`, `.mdx`                                                                                                                                                                                                      | Prettier 3.7.4 (`tabWidth: 2`, `singleQuote: true`, `printWidth: 80`, `proseWrap: "preserve"`, ATX headings, CJK/emoji GFM tables, empty fence blank line, Prettier-only embedded fences)                              |
| **YAML**                     | `.yml`, `.yaml`, `.prettierrc`                                                                                                                                                                                                  | Prettier 3.7.4 (`tabWidth: 2`, `singleQuote: true`, `printWidth: 80`, `bracketSpacing: true`, GitHub Actions `${{ ... }}` preservation, `.prettierrc` JSON/YAML auto-detection)                                        |
| **JSON / Stringify / JSONC** | `.json`, `package.json`, `package-lock.json`, `composer.json`, `.jsonc`, `.cjson`                                                                                                                                               | Prettier 3.7.4 (`objectWrap: "preserve"` + comments allowed / trailing commas stripped for `.json`, always-expanded `json-stringify` for manifests, comments + `trailingComma: "all"` preserved for `.jsonc`/`.cjson`) |
| **TOML**                     | `.toml`                                                                                                                                                                                                                         | Taplo 0.7.0 (`tabSize = 4`, `column_width = 80`, `allowed_blank_lines = 2`, `align_comments = true`, `array_auto_expand = true`, `array_auto_collapse = true`, `compact_arrays = true`, LF)                            |
| **JS / TS / TSX**            | `.js`, `.mjs`, `.cjs`, `.wxs`, `.ts`, `.mts`, `.cts`, `.tsx`                                                                                                                                                                    | Prettier 3.7.4 (`semi: true`, `singleQuote: true`, `jsxSingleQuote: false`, `tabWidth: 2`, `trailingComma: "all"`, `arrowParens: "always"`, `jsdoc: false`)                                                            |
| **CSS / SCSS / Less**        | `.css`, `.wxss`, `.scss`, `.less`                                                                                                                                                                                               | Prettier 3.7.4 (`singleQuote: true`, `tabWidth: 2`, `printWidth: 80`, LF)                                                                                                                                              |
| **HTML / Vue**               | `.html`, `.htm`, `.vue`                                                                                                                                                                                                         | Prettier 3.7.4 (`htmlWhitespaceSensitivity: "css"`, `vueIndentScriptAndStyle: false`, embedded `<script>` and `<style>` formatting)                                                                                    |
| **Rust**                     | `.rs`                                                                                                                                                                                                                           | Deterministic `#[derive(...)]` sorting + `cargo fmt`                                                                                                                                                                   |
| **VS Code Save No-Op**       | `LICENSE`, `.gitattributes`, `.gitignore`, `.dockerignore`, `.editorconfig`, `pre-commit`, `.sh`/`.bash`/`.zsh`, `Makefile`, `Dockerfile`, `.env`, `.txt`, `.conf`, `.ini`, `.xml`, `.svg`, `.py`, `.sql`, `.jsx`, `Cargo.lock` | Preserved byte-for-byte unchanged in `format_text_by_path` and skipped during workspace traversal (matching VS Code save behavior)                                                                                     |

- **Syntax Error Fallback**: Invalid TOML, YAML, JSON/JSONC, JS/TS/TSX, CSS/SCSS/Less, HTML, and Vue inputs are preserved verbatim without crashing or corrupting files.
- **`.prettierignore` & Symlink Safety**: Workspace traversal honors workspace-root `.prettierignore` glob rules (including directory patterns and `!` negations), skips standard build/artifact directories, and never follows symlinks (preventing symlink loops).

## Responsibilities & Module Layout

- `fmt::options` — Canonical VS Code Prettier 3.7.4 and Taplo 0.7.0 option builders plus embedded markup formatting helpers
- `fmt::markdown` / `fmt::yaml` / `fmt::json` / `fmt::toml` / `fmt::web` / `fmt::text` — Pure-Rust multi-language formatters and backward-compatible plain-text / Unicode display-width utilities
- `fmt::driver` — Public library entry points, `FileFormatKind` classification, `.prettierignore` walker, `#[derive(...)]` sorter, and `cargo fmt` / `cargo clippy --fix` orchestration
- `bump` / `sync` / `publish` — Workspace SemVer bumping, member dependency synchronization, and dependency-ordered `crates.io` publishing

## Public Library APIs

- **Workspace & Path Drivers**: `execute_unified_fmt`, `format_workspace_text_files`, `format_text_by_path`, `format_path`, `execute_fmt`, `detect_format_kind`, `FileFormatKind`, `UnifiedFmtReport`
- **Language Formatters**: `format_markdown`, `format_yaml`, `format_json`, `format_json_stringify`, `format_jsonc`, `format_toml`, `format_javascript`, `format_typescript`, `format_tsx`, `format_css`, `format_scss`, `format_less`, `format_html`, `format_vue`
- **Backward-Compatible Text Helpers**: `format_plain_text`, `normalize_line_endings`, `char_display_width`, `str_display_width`

## Installation & Quick Start

```sh
cargo install crate-cli
```

```sh
# Format all VS Code-managed text/web/config files + Rust derives + rustfmt
crate fmt
crate fmt --check

# Synchronize workspace member versions and publish in dependency order
crate sync
crate publish
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p crate-cli --all-features --all-targets -- -D warnings
cargo test -p crate-cli --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
