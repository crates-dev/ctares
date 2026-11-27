<div align="center">

# future-fn

**Move-Capturing Async Closure Macros**

[![crates.io](https://img.shields.io/crates/v/future-fn.svg?style=flat-square)](https://crates.io/crates/future-fn)
[![docs.rs](https://img.shields.io/docsrs/future-fn?style=flat-square)](https://docs.rs/future-fn)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/future-fn.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/future-fn/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Declarative macro collection simplifying async closure construction with owned state moved cleanly into generated futures.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `macro` — Async closure capture helpers eliminating manual `Arc::clone` and `Box::pin` boilerplate

## Installation & Quick Start

```sh
cargo add future-fn
```

```rust
use future_fn::*;

// Wrap owned state into reusable async closures without manual move scaffolding.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p future-fn --all-features --all-targets -- -D warnings
cargo test -p future-fn --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
