<div align="center">

# stripe-pay-client

**Browser & WebAssembly Stripe Checkout Client**

[![crates.io](https://img.shields.io/crates/v/stripe-pay-client.svg?style=flat-square)](https://crates.io/crates/stripe-pay-client)
[![docs.rs](https://img.shields.io/docsrs/stripe-pay-client?style=flat-square)](https://docs.rs/stripe-pay-client)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/stripe-pay-client.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/stripe-pay-client/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Client-side Stripe payment element configuration and mount abstractions layered on `stripe-pay-core` for WebAssembly checkout flows.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `element` — Stripe payment element configuration builders
- `host` / `mount` — Checkout surface mounting and transport forwarding

## Installation & Quick Start

```sh
cargo add stripe-pay-client
```

```rust
use stripe_pay_client::*;

// Configure Stripe payment elements in Rust/WASM using shared stripe-pay-core types.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p stripe-pay-client --all-features --all-targets -- -D warnings
cargo test -p stripe-pay-client --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
