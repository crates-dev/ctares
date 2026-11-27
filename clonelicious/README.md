<div align="center">

# clonelicious

**Ergonomic Closure Clone Macro**

[![crates.io](https://img.shields.io/crates/v/clonelicious.svg?style=flat-square)](https://crates.io/crates/clonelicious)
[![docs.rs](https://img.shields.io/docsrs/clonelicious?style=flat-square)](https://docs.rs/clonelicious)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/clonelicious.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/clonelicious/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Declarative macro library providing `clone!` to clone captured handles concisely and execute closures without repetitive boilerplate.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `macro` — `clone!` declarative macro expanding move-clones cleanly before closure evaluation

## Installation & Quick Start

```sh
cargo add clonelicious
```

```rust
use clonelicious::*;
use std::sync::Arc;

let state: Arc<String> = Arc::new("ready".to_string());
let out: String = clone!(state => move || state.to_string())();
assert_eq!(out, "ready");
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p clonelicious --all-features --all-targets -- -D warnings
cargo test -p clonelicious --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
