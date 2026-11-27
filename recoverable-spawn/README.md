<div align="center">

# recoverable-spawn

**Panic-Recovering Sync & Async Task Spawner**

[![crates.io](https://img.shields.io/crates/v/recoverable-spawn.svg?style=flat-square)](https://crates.io/crates/recoverable-spawn)
[![docs.rs](https://img.shields.io/docsrs/recoverable-spawn?style=flat-square)](https://docs.rs/recoverable-spawn)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/recoverable-spawn.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/recoverable-spawn/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Spawns synchronous threads and Tokio async tasks wrapped in panic guards so failed tasks invoke recovery hooks or restart without crashing the host process.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `sync` — `catch_unwind`-guarded thread spawning with restart policies
- `async` — Tokio task wrappers inspecting `JoinError` panics
- `common` — Shared panic payload formatting and hook utilities

## Installation & Quick Start

```sh
cargo add recoverable-spawn
```

```rust
use recoverable_spawn::*;

// Launch a panic-isolated sync thread or async task with automatic recovery handling.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p recoverable-spawn --all-features --all-targets -- -D warnings
cargo test -p recoverable-spawn --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
