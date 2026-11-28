# AGENTS.md — ctares Workspace Engineering & Documentation Governance

Mandatory architectural, engineering, formatting, release, and documentation governance rules for all contributors and AI coding agents operating in the `ctares` monorepo (`crates-dev/ctares`).

---

## 1. Repository Architecture & Crate Responsibility Matrix

`ctares` is a Rust 2024 unified Cargo workspace (resolver `2`) housing **24** independently publishable crates sharing a single workspace version in root `Cargo.toml`.

| Layer                               | Crate                         | Path                           | Primary Responsibility                                                                                                                                                                                                                           |
| :---------------------------------- | :---------------------------- | :----------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Toolchain & Formatting**          | `crate-cli`                   | `crate-cli/`                   | Unified workspace CLI (`crate bump`, `crate sync`, `crate publish`, `crate fmt`) and VS Code Prettier 3.7.4 + Taplo 0.7.0 aligned pure-Rust multi-format formatter (`execute_unified_fmt`, `format_workspace_text_files`, `format_text_by_path`) |
| **Procedural & Declarative Macros** | `lombok-macros`               | `lombok-macros/`               | Proc macros for `#[derive(Data, Getter, GetterMut, Setter, New, DisplayDebug, DisplayDebugFormat, CustomDebug)]` with field visibility and copy/clone controls                                                                                   |
| **Procedural & Declarative Macros** | `instrument-level`            | `instrument-level/`            | Level-specific tracing span attribute macros (`#[instrument_trace]`, `#[instrument_debug]`, `#[instrument_info]`, `#[instrument_warn]`, `#[instrument_error]`)                                                                                   |
| **Procedural & Declarative Macros** | `clonelicious`                | `clonelicious/`                | `clone!` macro automating explicit move-clones before closure execution                                                                                                                                                                          |
| **Procedural & Declarative Macros** | `future-fn`                   | `future-fn/`                   | Macros simplifying owned-capture async closures and boxed futures                                                                                                                                                                                |
| **Procedural & Declarative Macros** | `std-macro-extensions`        | `std-macro-extensions/`        | Concise literal and helper macros for `std` collections (`HashMap`, `BTreeMap`, `Vec`, `Arc`, `Mutex`, `RwLock`, `RefCell`, I/O)                                                                                                                 |
| **Logging & Console**               | `color-output`                | `color-output/`                | Atomic ANSI colorized terminal printing builders and utilities                                                                                                                                                                                   |
| **Logging & Console**               | `color-log`                   | `color-log/`                   | Sync and Tokio-async structured file/console logger with size-based rotation                                                                                                                                                                     |
| **Concurrency & Async**             | `tokio-broadcast`             | `tokio-broadcast/`             | Ergonomic typed `Broadcast` and keyed `BroadcastMap` wrappers over `tokio::sync::broadcast`                                                                                                                                                      |
| **Concurrency & Async**             | `recoverable-spawn`           | `recoverable-spawn/`           | Panic-catching sync and async task launchers with automatic recovery callbacks                                                                                                                                                                   |
| **Concurrency & Async**             | `recoverable-thread-pool`     | `recoverable-thread-pool/`     | Fixed-size worker pool built on `recoverable-spawn` that restarts panicked workers                                                                                                                                                               |
| **Data, Time & Versioning**         | `bin-encode-decode`           | `bin-encode-decode/`           | Binary encoder/decoder supporting arbitrary custom alphabets beyond Base64                                                                                                                                                                       |
| **Data, Time & Versioning**         | `chunkify`                    | `chunkify/`                    | Deterministic byte/file chunking and resumable reassembly primitives                                                                                                                                                                             |
| **Data, Time & Versioning**         | `compare_version`             | `compare-version/`             | Semantic version parsing, ordering, and compatibility comparison                                                                                                                                                                                 |
| **Data, Time & Versioning**         | `system-time`                 | `system-time/`                 | Locale-aware system wall-clock formatting and timestamp helpers                                                                                                                                                                                  |
| **Data, Time & Versioning**         | `chinese-identification-card` | `chinese-identification-card/` | Official GB 11643 Chinese resident ID validation, metadata extraction, and checksum verification                                                                                                                                                 |
| **Security & Auth**                 | `jwt-service`                 | `jwt-service/`                 | Async JWT signing, verification, and typed custom claims management                                                                                                                                                                              |
| **Payments (Stripe SDK)**           | `stripe-pay-core`             | `stripe-pay-core/`             | Shared Stripe domain types (`Money`, typed IDs, `PaymentIntent`, `Charge`, `Refund`, bracketed form encoding)                                                                                                                                    |
| **Payments (Stripe SDK)**           | `stripe-pay-server`           | `stripe-pay-server/`           | Server-side Stripe HTTP/webhook transport and constant-time HMAC-SHA256 signature verification                                                                                                                                                   |
| **Payments (Stripe SDK)**           | `stripe-pay-client`           | `stripe-pay-client/`           | Browser/WASM Stripe checkout element configuration built over `stripe-pay-core`                                                                                                                                                                  |
| **Networking**                      | `tcplane`                     | `tcplane/`                     | Tokio-based async TCP server with request/response hooks and stream lifecycle                                                                                                                                                                    |
| **Networking**                      | `tcp-request`                 | `tcp-request/`                 | Synchronous raw TCP request client with redirect and response handling                                                                                                                                                                           |
| **Networking**                      | `udp`                         | `udp/`                         | Tokio-based async UDP server with hook-based datagram dispatch                                                                                                                                                                                   |
| **Networking**                      | `udp-request`                 | `udp-request/`                 | Raw UDP datagram request/response client                                                                                                                                                                                                         |

### Architectural Invariants

1. **Strict Dependency Layering**: Internal workspace edges must stay acyclic and follow root `Cargo.toml` `[workspace] members` publish order (`system-time` -> `color-output` -> `crate-cli` / `color-log`, `recoverable-spawn` -> `recoverable-thread-pool`, `stripe-pay-core` -> `stripe-pay-server` / `stripe-pay-client`).
2. **Single Source of Unified Formatting**: All VS Code-aligned formatting rules (Prettier 3.7.4 for Markdown/YAML/JSON/JSON-Stringify/JSONC/JS/TS/TSX/CSS/SCSS/Less/HTML/Vue, Taplo 0.7.0 4-space for TOML, VS Code no-op preservation for unformatted text/config/script files, workspace `.prettierignore` honoring, `#[derive(...)]` sorting, and `cargo fmt`) live exclusively in `crate-cli/src/fmt/`. Downstream repositories (`euv`, `hyperlane`, `hyperlane-quick-start`, `commerce-suite`) consume `crate-cli` rather than duplicating formatters.
3. **Public API Stability**: Never remove or break existing `pub` signatures in a minor/patch release without a coordinated ecosystem migration plan.

---

## 2. Engineering, Rust & Unified Formatting Standards

1. **Rust Standards (Highest Priority)**: Follow `~/.agents/skills/rust-standards/SKILL.md` without exception:
   - Pure 9-keyword module filenames (`mod.rs`, `const.rs`, `enum.rs`, `struct.rs`, `trait.rs`, `impl.rs`, `fn.rs`, `type.rs`, `static.rs`).
   - Three-section `mod.rs` with zero comments; centralized imports in `lib.rs` and `tests/mod.rs`.
   - Explicit type annotations on every `let` binding and closure parameter; zero `#[allow(...)]`; zero `use ... as ...`; zero direct `self.field` reads/writes outside destructuring; zero comments in `tests/` or `*.toml`.
   - Strict `[dependencies]` / `[workspace.dependencies]` ordering (workspace crates first, blank line, third-party sorted by name length then lexicographically).

2. **Unified Formatting (`crate fmt`)**:
   - Run `cargo run -p crate-cli -- fmt` twice and verify idempotency with `cargo run -p crate-cli -- fmt --check` and `cargo fmt --all -- --check`.
   - Formatting MUST maintain 100% parity with VS Code defaults:
     - **Prettier 3.7.4** (`esbenp.prettier-vscode`: `tabWidth: 2`, `singleQuote: true`, `semi: true`, `endOfLine: "lf"`, `printWidth: 80`, `proseWrap: "preserve"`, `bracketSpacing: true`, `trailingComma: "all"`, `arrowParens: "always"`, `jsxSingleQuote: false`, `objectWrap: "preserve"`, `vueIndentScriptAndStyle: false`, `embeddedLanguageFormatting: "auto"`) for Markdown (`.md`, `.markdown`, `.mdx`), YAML (`.yml`, `.yaml`, `.prettierrc`), JSON / JSON-Stringify / JSONC (`.json`, `package.json`, `package-lock.json`, `composer.json`, `.jsonc`, `.cjson`), JS/TS/TSX (`.js`, `.mjs`, `.cjs`, `.wxs`, `.ts`, `.mts`, `.cts`, `.tsx`), CSS/SCSS/Less (`.css`, `.wxss`, `.scss`, `.less`), HTML (`.html`, `.htm`), and Vue (`.vue`).
     - **Taplo 0.7.0** (`tamasfe.even-better-toml`: `tabSize = 4`, `column_width = 80`, `allowed_blank_lines = 2`, `align_comments = true`, `array_auto_expand = true`, `array_auto_collapse = true`, `compact_arrays = true`) for TOML (`.toml`).
     - **VS Code Save No-Op Preservation**: Unformatted text/config/script files (`LICENSE`, `.gitattributes`, `.gitignore`, `.dockerignore`, `.editorconfig`, `pre-commit`, `.sh`, `.bash`, `.zsh`, `Makefile`, `Dockerfile`, `.env`, `.txt`, `.conf`, `.ini`, `.xml`, `.svg`, `.py`, `.sql`, `.jsx`, `Cargo.lock`, etc.) MUST remain byte-for-byte unchanged, and workspace-root `.prettierignore` rules MUST be honored.
   - Indent nested list items and code blocks under single-digit ordered-list items (`1. `) with 3 spaces (4 spaces under `10. `) per CommonMark / Prettier 3.7.4.

3. **Commit & PR Hygiene**:
   - All commits, PR titles, PR bodies, comments, docs, and skills MUST be English-only and pass `~/.agents/skills/git-standards/scripts/verify_english_only.py`.
   - Never bypass `~/.git-hooks/pre-commit` (`--no-verify` is forbidden).

---

## 3. Verification & Ecosystem Release Workflow

### Local Verification Gates (Mandatory Before Commit / Push)

```sh
cargo run -p crate-cli -- fmt
cargo run -p crate-cli -- fmt --check
cargo fmt --all -- --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features
cargo check --release --workspace --all-features
python3 ~/.agents/skills/rust-standards/scripts/staged_file_gate.py .
python3 ~/.agents/skills/rust-standards/scripts/verify_dep_order.py .
```

### Ecosystem Dependency-Ordered Release Cascade

`ctares` is **Tier 1** in the cross-repository release topology. Whenever a new `ctares` version is published on `crates.io`:

1. **Tier 1 — `ctares`**: Bump `[workspace.package].version` and internal `[workspace.dependencies]` pins in root `Cargo.toml`, run `crate fmt --check`, push to `master` (or merge PR), and wait until `.github/workflows/rust.yml` (`tests`, `clippy`, `build`, `check`, `publish`, `release`) completes green.
2. **Tier 2 — `euv` & `hyperlane`**: Upgrade `ctares` dependency versions (`crate-cli`, `lombok-macros`, `color-output`, `tokio-broadcast`), run `euv fmt` / `hyperlane fmt`, bump minor version, push/merge to `master`, and wait for CI + `crates.io` publish green.
3. **Tier 3 — `hyperlane-quick-start` & `commerce-suite`**: Upgrade `ctares` and `hyperlane*` dependency versions, run `crate fmt`, bump minor version, push to `master`, and wait for CI green.
4. **Tier 4 — `web-frameworks`**: Update `rust/hyperlane/Cargo.toml` to the newly published exact `hyperlane` version and push.

---

## 4. Unified Documentation & Skill Maintenance Policy

Documentation and agent skills are first-class deliverables maintained under the exact same CI and review bar as Rust code.

### When & How to Update Each Document

| Change Trigger                                              | Target Document(s)                                                                                            | Mandatory Update Procedure                                                                                                             |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| Add, rename, split, or remove a workspace crate             | Root `README.md`, crate `README.md`, root `AGENTS.md`, `.agents/skills/ctares/SKILL.md`                       | Update the 24-crate matrix, Mermaid topology, crate responsibility table, and publish order; create or delete the crate's `README.md`. |
| Change `crate-cli` commands, flags, or formatting rules     | `crate-cli/README.md`, root `AGENTS.md`, `.agents/skills/ctares/SKILL.md`, `~/.agents/skills/ctares/SKILL.md` | Document new flags/invariants, add parity unit tests in `crate-cli/tests/fmt/`, and sync both repo-local and global skills.            |
| Change a crate's public API, macro syntax, or feature flags | `<crate>/README.md`                                                                                           | Update the overview, usage snippet, and verification section so every code snippet compiles against current source.                    |
| Change CI workflows, quality gates, or release order        | Root `AGENTS.md`, `.agents/skills/ctares/SKILL.md`, `~/.agents/skills/ctares/SKILL.md`                        | Sync local gate commands and release cascade steps with `.github/workflows/rust.yml`.                                                  |
| Bump workspace version or dependencies                      | Root `Cargo.toml` (and `crate sync` if needed)                                                                | Never hardcode stale patch numbers in skills; refer to `Cargo.toml` as authoritative.                                                  |

### Post-Edit Documentation Validation

1. Format all modified Markdown, YAML, JSON, TOML, and Web text files with `cargo run -p crate-cli -- fmt` and confirm zero diff with `--check`.
2. Whenever modifying any `SKILL.md` (strictly standards, requirements, and workflows only; `<= 160` lines), keep repo-local `.agents/skills/ctares/SKILL.md` and `~/.agents/skills/ctares/SKILL.md` synchronized and run:

   ```sh
   python3 ~/.agents/scripts/audit_skills.py
   python3 ~/.agents/scripts/update_skill_lock.py
   ```
