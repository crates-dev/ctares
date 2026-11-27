<div align="center">

# color-log

**Sync & Async Rotating Structured Logger**

[![crates.io](https://flat.badgen.net/crates/v/color-log)](https://crates.io/crates/color-log)
[![docs.rs](https://img.shields.io/docsrs/color-log?style=flat-square)](https://docs.rs/color-log)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/color-log/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Configurable synchronous and Tokio-async logger supporting error/info/debug levels, custom handlers, and automatic size-based log file rotation.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `struct` / `trait` — Logger configuration, rotation thresholds, and custom sink hooks
- `fn` / `impl` — Sync file writers and Tokio channel-buffered async log writers
- `const` — Default level prefixes and rotation limits

## Installation & Quick Start

```sh
cargo add color-log
```

```rust
use color_log::*;

// Initialize a sync or Tokio-async rotating logger with custom output paths.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p color-log --all-features --all-targets -- -D warnings
cargo test -p color-log --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
