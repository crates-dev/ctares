<div align="center">

# tcp-request

**Raw Synchronous TCP Request Client**

[![crates.io](https://flat.badgen.net/crates/v/tcp-request)](https://crates.io/crates/tcp-request)
[![docs.rs](https://img.shields.io/docsrs/tcp-request?style=flat-square)](https://docs.rs/tcp-request)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/tcp-request/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Synchronous TCP request builder and response reader supporting timeouts, redirects, and raw byte-stream framing over `std::net::TcpStream`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `request` — Fluent TCP request configuration and builder
- `response` — Raw response framing, headers/body inspection, and error handling
- `common` — Shared socket timeout and stream helpers

## Installation & Quick Start

```sh
cargo add tcp-request
```

```rust
use tcp_request::*;

// Configure a synchronous TCP request with explicit timeout and inspect the raw response.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p tcp-request --all-features --all-targets -- -D warnings
cargo test -p tcp-request --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
