<div align="center">

# lombok-macros

**Lombok-Style Rust Derive Macros**

[![crates.io](https://flat.badgen.net/crates/v/lombok-macros)](https://crates.io/crates/lombok-macros)
[![docs.rs](https://img.shields.io/docsrs/lombok-macros?style=flat-square)](https://docs.rs/lombok-macros)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/lombok-macros/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Procedural derive macro suite generating zero-boilerplate getters (`Getter`, `GetterMut`), setters (`Setter`), combined `Data`, constructors (`New`), `CustomDebug`, and `DisplayDebug` with full generic and lifetime support.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `parse` / `config` / `visibility` — Attribute parsing (`#[get(type(copy))]`, `#[get(pub(crate))]`, `#[set(skip)]`, `#[debug(skip)]`)
- `generate` / `func` — Hygiene-safe method and trait code generation

## Installation & Quick Start

```sh
cargo add lombok-macros
```

```rust
use lombok_macros::*;

#[derive(Clone, Data, Debug, New)]
pub struct ServiceConfig {
    #[get(type(copy))]
    pub(crate) port: u16,
    pub(crate) host: String,
}
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p lombok-macros --all-features --all-targets -- -D warnings
cargo test -p lombok-macros --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
