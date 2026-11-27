<div align="center">

# system-time

**Locale-Aware System Time & Timestamp Utilities**

[![crates.io](https://img.shields.io/crates/v/system-time.svg?style=flat-square)](https://crates.io/crates/system-time)
[![docs.rs](https://img.shields.io/docsrs/system-time?style=flat-square)](https://docs.rs/system-time)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/system-time.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/system-time/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Zero-dependency wall-clock date/time formatting and timestamp extraction respecting host locale settings.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `fn` — Current date, time, Unix epoch, and formatted timestamp helpers
- `enum` / `impl` / `const` — Timezone offset and formatting definitions

## Installation & Quick Start

```sh
cargo add system-time
```

```rust
use system_time::*;

let now: String = time();
assert!(!now.is_empty());
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p system-time --all-features --all-targets -- -D warnings
cargo test -p system-time --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
