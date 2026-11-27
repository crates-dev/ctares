<div align="center">

# bin-encode-decode

**Custom-Alphabet Binary Encoding & Decoding**

[![crates.io](https://flat.badgen.net/crates/v/bin-encode-decode)](https://crates.io/crates/bin-encode-decode)
[![docs.rs](https://img.shields.io/docsrs/bin-encode-decode?style=flat-square)](https://docs.rs/bin-encode-decode)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/bin-encode-decode/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

High-performance binary encoder and decoder supporting arbitrary custom character sets beyond standard Base64.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `common` — Alphabet validation, lookup tables, and error types
- `encode` — Deterministic binary-to-text encoding over user-supplied alphabets
- `decode` — Strict character validation and binary reconstruction

## Installation & Quick Start

```sh
cargo add bin-encode-decode
```

```rust
use bin_encode_decode::*;

let charset: &str = "0123456789abcdef";
let encoded: String = encode(b"ctares", charset).unwrap();
let decoded: Vec<u8> = decode(&encoded, charset).unwrap();
assert_eq!(decoded, b"ctares");
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p bin-encode-decode --all-features --all-targets -- -D warnings
cargo test -p bin-encode-decode --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
