<div align="center">

# stripe-pay-core

**Shared Stripe Payment Domain Types**

[![crates.io](https://flat.badgen.net/crates/v/stripe-pay-core)](https://crates.io/crates/stripe-pay-core)
[![docs.rs](https://img.shields.io/docsrs/stripe-pay-core?style=flat-square)](https://docs.rs/stripe-pay-core)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/stripe-pay-core/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Runtime-agnostic Stripe domain crate shared by `stripe-pay-server` and `stripe-pay-client`: decimal money handling, typed Stripe identifiers, payment intents, charges, refunds, structured errors, and bracketed form encoding.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `money` / `id` — Exact `rust_decimal` monetary representation and strongly typed Stripe object IDs
- `payment_intent` / `payment_method` / `charge` / `refund` — Typed Stripe API payloads
- `form` / `protocol` / `error` — Bracketed `application/x-www-form-urlencoded` encoder and typed error contracts

## Installation & Quick Start

```sh
cargo add stripe-pay-core
```

```rust
use stripe_pay_core::*;

// Share exact typed Stripe domain models across backend and WebAssembly checkout clients.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p stripe-pay-core --all-features --all-targets -- -D warnings
cargo test -p stripe-pay-core --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
