<div align="center">

# jwt-service

**Async JWT Signing & Claims Verification**

[![crates.io](https://img.shields.io/crates/v/jwt-service.svg?style=flat-square)](https://crates.io/crates/jwt-service)
[![docs.rs](https://img.shields.io/docsrs/jwt-service?style=flat-square)](https://docs.rs/jwt-service)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/jwt-service.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/jwt-service/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

High-performance JWT authentication and authorization library built atop `jsonwebtoken` and `serde_json` for signing, validating, and inspecting typed claims.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `struct` / `enum` — Token payload containers, algorithm selectors, and typed errors
- `impl` — Token generation, signature validation, and expiration checks
- `const` — Claim keys and validation defaults

## Installation & Quick Start

```sh
cargo add jwt-service
```

```rust
use jwt_service::*;

// Construct a JWT service, sign claims, and verify bearer tokens asynchronously.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p jwt-service --all-features --all-targets -- -D warnings
cargo test -p jwt-service --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
