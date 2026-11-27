<div align="center">

# crate-cli

**Unified Cargo Workspace CLI & VS Code Prettier-Aligned Formatter**

[![crates.io](https://flat.badgen.net/crates/v/crate-cli)](https://crates.io/crates/crate-cli)
[![docs.rs](https://img.shields.io/docsrs/crate-cli?style=flat-square)](https://docs.rs/crate-cli)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/crate-cli/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Command-line tool and embeddable library managing Cargo workspace lifecycles (`crate bump`, `crate sync`, `crate publish`, `crate fmt`) plus pure-Rust VS Code Prettier-aligned multi-format formatting.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `fmt::markdown` / `fmt::yaml` / `fmt::json` / `fmt::toml` / `fmt::text` — Pure-Rust formatters aligned with VS Code `esbenp.prettier-vscode` (`tabWidth: 2`, `singleQuote: true`, `printWidth: 80`, `proseWrap: "preserve"`, LF endings) and Taplo TOML
- `fmt::driver` — Public library entry points (`execute_unified_fmt`, `format_workspace_text_files`, `format_text_by_path`) consumed by `euv-cli` and `hyperlane-cli`
- `bump` / `sync` / `publish` — Workspace SemVer bumping, member dependency synchronization, and dependency-ordered `crates.io` publishing

## Installation & Quick Start

```sh
cargo install crate-cli
```

```sh
# Format all supported text files (.md, .yml/.yaml, .json/.jsonc, .toml) + Rust derives + rustfmt
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
