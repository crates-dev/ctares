<div align="center">

# recoverable-thread-pool

**Self-Healing Worker Thread Pool**

[![crates.io](https://img.shields.io/crates/v/recoverable-thread-pool.svg?style=flat-square)](https://crates.io/crates/recoverable-thread-pool)
[![docs.rs](https://img.shields.io/docsrs/recoverable-thread-pool?style=flat-square)](https://docs.rs/recoverable-thread-pool)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/recoverable-thread-pool.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/recoverable-thread-pool/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Fixed-capacity worker pool layered over `recoverable-spawn` that automatically detects worker panics and respawns healthy workers.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `thread_pool` — Job channel scheduling, pool lifecycle, and graceful shutdown
- `worker` — Panic-recovering worker loop powered by `recoverable-spawn`

## Installation & Quick Start

```sh
cargo add recoverable-thread-pool
```

```rust
use recoverable_thread_pool::*;

// Create a fixed-size self-healing thread pool and submit closures safely.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p recoverable-thread-pool --all-features --all-targets -- -D warnings
cargo test -p recoverable-thread-pool --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
