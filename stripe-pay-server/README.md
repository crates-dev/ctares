<div align="center">

# stripe-pay-server

**Server-Side Stripe Transport & Webhook Verification**

[![crates.io](https://flat.badgen.net/crates/v/stripe-pay-server)](https://crates.io/crates/stripe-pay-server)
[![docs.rs](https://img.shields.io/docsrs/stripe-pay-server?style=flat-square)](https://docs.rs/stripe-pay-server)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/stripe-pay-server/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Server-side Stripe integration built on `stripe-pay-core`, providing constant-time HMAC-SHA256 webhook signature verification and payload parsing.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `signature` — `Stripe-Signature` header parsing, timestamp tolerance, and constant-time HMAC-SHA256 verification
- `webhook` — Typed webhook event decoding layered over `stripe-pay-core`

## Installation & Quick Start

```sh
cargo add stripe-pay-server
```

```rust
use stripe_pay_server::*;

// Verify incoming Stripe-Signature headers with constant-time HMAC-SHA256 checks.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p stripe-pay-server --all-features --all-targets -- -D warnings
cargo test -p stripe-pay-server --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
