<div align="center">

# chinese-identification-card

**GB 11643 Chinese Resident ID Validator**

[![crates.io](https://img.shields.io/crates/v/chinese-identification-card.svg?style=flat-square)](https://crates.io/crates/chinese-identification-card)
[![docs.rs](https://img.shields.io/docsrs/chinese-identification-card?style=flat-square)](https://docs.rs/chinese-identification-card)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/chinese-identification-card.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/chinese-identification-card/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Validates 18-digit Chinese resident identification card numbers using official administrative region rules, birth date checks, and ISO 7064 MOD 11-2 checksums.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `const` — Official weight factors, check digits, and length boundaries
- `struct` — Typed ID card descriptor encapsulating parsed metadata
- `impl` — Validation, birth date verification, and checksum computation

## Installation & Quick Start

```sh
cargo add chinese-identification-card
```

```rust
use chinese_identification_card::*;

let valid: bool = ChineseIdCard::is_valid("11010519491231002X");
assert!(valid);
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p chinese-identification-card --all-features --all-targets -- -D warnings
cargo test -p chinese-identification-card --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
