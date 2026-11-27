<div align="center">

# udp-request

**Raw Synchronous UDP Datagram Client**

[![crates.io](https://flat.badgen.net/crates/v/udp-request)](https://crates.io/crates/udp-request)
[![docs.rs](https://img.shields.io/docsrs/udp-request?style=flat-square)](https://docs.rs/udp-request)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/udp-request/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Simple, predictable UDP client over `std::net::UdpSocket` for sending datagrams and awaiting bounded responses.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `request` — UDP target addressing, timeout, and datagram payload builder
- `response` — Received datagram buffer and peer address inspection
- `common` — Shared socket configuration helpers

## Installation & Quick Start

```sh
cargo add udp-request
```

```rust
use udp_request::*;

// Send a raw UDP datagram with explicit read/write timeouts and inspect the reply.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p udp-request --all-features --all-targets -- -D warnings
cargo test -p udp-request --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
