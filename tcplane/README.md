<div align="center">

# tcplane

**Lightweight Tokio Async TCP Server**

[![crates.io](https://flat.badgen.net/crates/v/tcplane)](https://crates.io/crates/tcplane)
[![docs.rs](https://img.shields.io/docsrs/tcplane?style=flat-square)](https://docs.rs/tcplane)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/tcplane/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

High-performance Tokio-based TCP server providing structured configuration, connection context, lifecycle hooks, and async stream handling.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `server` / `config` — Listener binding, concurrency, and socket tuning
- `context` / `stream` / `request` / `response` — Per-connection state and wire framing
- `handler` / `error` — Async handler traits and typed server errors

## Installation & Quick Start

```sh
cargo add tcplane
```

```rust
use tcplane::*;

// Configure and start an async Tokio TCP server with custom connection handlers.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p tcplane --all-features --all-targets -- -D warnings
cargo test -p tcplane --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
