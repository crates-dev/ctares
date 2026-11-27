<div align="center">

# color-output

**Atomic ANSI Colored Console Output**

[![crates.io](https://flat.badgen.net/crates/v/color-output)](https://crates.io/crates/color-output)
[![docs.rs](https://img.shields.io/docsrs/color-output?style=flat-square)](https://docs.rs/color-output)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/color-output/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Atomic terminal output library supporting foreground/background ANSI colors via direct helper functions and fluent builders, re-exporting `system-time`.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `color` — ANSI foreground/background color enums and escape formatters
- `text` — Styled text spans and segment composition
- `task` / `utils` — Atomic stdout writer and convenience macros/helpers

## Installation & Quick Start

```sh
cargo add color-output
```

```rust
use color_output::*;

OutputBuilder::new()
    .text("ctares ready")
    .color(ColorType::Color256(0x00ff00))
    .println();
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p color-output --all-features --all-targets -- -D warnings
cargo test -p color-output --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
