<div align="center">

# udp

**Lightweight Tokio Async UDP Server**

[![crates.io](https://img.shields.io/crates/v/udp.svg?style=flat-square)](https://crates.io/crates/udp)
[![docs.rs](https://img.shields.io/docsrs/udp?style=flat-square)](https://docs.rs/udp)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/udp.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/udp/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Async UDP datagram server built on Tokio with hook-driven request/response dispatch, panic handling, and per-datagram context attributes.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `server` / `socket` / `config` — Async UDP socket binding and datagram loop
- `context` / `attribute` / `request` / `response` — Datagram metadata and reply builders
- `hook` / `panic` / `error` — Pluggable async hooks and panic isolation

## Installation & Quick Start

```sh
cargo add udp
```

```rust
use udp::*;

// Bind an async Tokio UDP server and register datagram hooks.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p udp --all-features --all-targets -- -D warnings
cargo test -p udp --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
