<div align="center">

# chunkify

**Deterministic Byte & File Chunking**

[![crates.io](https://img.shields.io/crates/v/chunkify.svg?style=flat-square)](https://crates.io/crates/chunkify)
[![docs.rs](https://img.shields.io/docsrs/chunkify?style=flat-square)](https://docs.rs/chunkify)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/chunkify.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/chunkify/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

High-throughput synchronous and Tokio-async chunking and resumable reassembly primitives backed by `DashMap` and `XxHash3_64`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `struct` / `enum` / `trait` — Chunk descriptors, storage abstractions, and error enums
- `impl` — Streaming split, integrity-hashed chunk storage, and ordered merge
- `static` — Shared lock-free chunk registry helpers

## Installation & Quick Start

```sh
cargo add chunkify
```

```rust
use chunkify::*;

// Configure chunk size and split/reassemble payloads deterministically.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p chunkify --all-features --all-targets -- -D warnings
cargo test -p chunkify --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
