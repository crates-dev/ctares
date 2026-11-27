<div align="center">

# compare_version

**Semantic Version Comparison & Compatibility**

[![crates.io](https://flat.badgen.net/crates/v/compare_version)](https://crates.io/crates/compare_version)
[![docs.rs](https://img.shields.io/docsrs/compare_version?style=flat-square)](https://docs.rs/compare_version)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/compare_version/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Parses, orders, and evaluates semantic versioning strings (`major.minor.patch`) for version compatibility checks across tooling and services.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `struct` / `enum` — Parsed version segments and comparison result enums
- `impl` — Ordering, parsing, and compatibility predicates
- `const` — Separator and boundary constants

## Installation & Quick Start

```sh
cargo add compare_version
```

```rust
use compare_version::*;

// Parse and compare SemVer strings deterministically.
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p compare_version --all-features --all-targets -- -D warnings
cargo test -p compare_version --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
