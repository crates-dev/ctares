<div align="center">

# std-macro-extensions

**Standard Library Collection & Sync Helper Macros**

[![crates.io](https://img.shields.io/crates/v/std-macro-extensions.svg?style=flat-square)](https://crates.io/crates/std-macro-extensions)
[![docs.rs](https://img.shields.io/docsrs/std-macro-extensions?style=flat-square)](https://docs.rs/std-macro-extensions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/std-macro-extensions.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/std-macro-extensions/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Declarative macros streamlining creation and manipulation of `std` collections (`HashMap`, `BTreeMap`, `HashSet`, `VecDeque`, `BinaryHeap`, `LinkedList`), smart pointers (`Arc`, `Rc`, `Box`), locks (`Mutex`, `RwLock`, `RefCell`), and console I/O.

This package is part of the [`ctares`](../README.md) unified Rust 2024 workspace and follows the repository's strict module purity, formatting, and dependency-ordered release standards.

## Responsibilities & Module Layout

- `hash_map` / `b_tree_map` / `hash_set` / `b_tree_set` / `vector` / `vector_deque` / `binary_heap` / `linked_list` — Collection literal macros
- `arc` / `rc` / `boxed` / `cell` / `ref_cell` / `mutex` / `rw_lock` — Smart pointer and synchronization wrappers
- `cin` / `cout` / `path` / `string` / `execute` — Ergonomic I/O and formatting macros

## Installation & Quick Start

```sh
cargo add std-macro-extensions
```

```rust
use std_macro_extensions::*;

let map = hash_map! { "env" => "prod", "region" => "us" };
assert_eq!(map.get("env"), Some(&"prod"));
```

## Verification

Run crate-scoped formatting, lint, and test checks from the workspace root:

```sh
cargo run -p crate-cli -- fmt --check
cargo clippy -p std-macro-extensions --all-features --all-targets -- -D warnings
cargo test -p std-macro-extensions --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, CLI flag, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project-level standards and release workflows live in [`../.agents/skills/ctares/SKILL.md`](../.agents/skills/ctares/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
