<div align="center">

# instrument-level

**Level-Scoped Tracing Instrumentation Proc Macros**

[![crates.io](https://img.shields.io/crates/v/instrument-level.svg?style=flat-square)](https://crates.io/crates/instrument-level)
[![docs.rs](https://img.shields.io/docsrs/instrument-level?style=flat-square)](https://docs.rs/instrument-level)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/instrument-level.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/instrument-level/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Procedural attribute macros (`#[instrument_trace]`, `#[instrument_debug]`, `#[instrument_info]`, `#[instrument_warn]`, `#[instrument_error]`) wrapping `tracing::instrument(skip_all, level = ...)`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `lib` — Five level-specific attribute proc-macro entry points
- `fn` — Token expansion attaching `skip_all` and forwarding custom `tracing::instrument` arguments

## Installation & Quick Start

```sh
cargo add instrument-level tracing
```

```rust
use instrument_level::*;

#[instrument_info]
fn handle_request() {}
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p instrument-level --all-features --all-targets -- -D warnings
cargo test -p instrument-level --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
