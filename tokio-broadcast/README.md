<div align="center">

# tokio-broadcast

**Ergonomic Typed Tokio Broadcast & Keyed BroadcastMap**

[![crates.io](https://img.shields.io/crates/v/tokio-broadcast.svg?style=flat-square)](https://crates.io/crates/tokio-broadcast)
[![docs.rs](https://img.shields.io/docsrs/tokio-broadcast?style=flat-square)](https://docs.rs/tokio-broadcast)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/tokio-broadcast.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/tokio-broadcast/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Lightweight publish-subscribe wrapper over `tokio::sync::broadcast` providing `Broadcast<T>` and keyed `BroadcastMap<K, T>` backed by `DashMap` and `XxHash3_64`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `broadcast` — Single-channel typed pub/sub wrapper with concise `send`/`subscribe` ergonomics
- `broadcast_map` — Keyed multi-room broadcast map with automatic channel lifecycle management

## Installation & Quick Start

```sh
cargo add tokio-broadcast
```

```rust
use tokio_broadcast::*;

let bus: Broadcast<String> = Broadcast::new(16);
let mut rx = bus.subscribe();
let _ = bus.send("hello".to_string());
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p tokio-broadcast --all-features --all-targets -- -D warnings
cargo test -p tokio-broadcast --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
