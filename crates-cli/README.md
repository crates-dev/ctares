# crates-cli

A command-line tool for managing Cargo package lifecycles in workspaces and monorepos.

The package is named `crates-cli`; the installed command is `cc`.

## Features

- **bump**: Semantic version bump in `Cargo.toml` (patch / minor / major / alpha / beta / rc / release), preserving manifest formatting byte-for-byte outside the version literal.
- **sync**: Align every local path entry under `[workspace.dependencies]` with the workspace root version, and the dep alias with each member crate's actual `[package].name`.
- **publish**: Publish workspace packages to crates.io in `[workspace.members]` declaration order (root package last), validated against workspace-local dependencies, with per-package retries and idempotent handling of already-published versions.
- **fmt**: Sort `#[derive(...)]` traits, run `cargo fmt` and `cargo clippy --fix` across the workspace.

## Install

```sh
cargo install crates-cli
```

## Usage

```sh
cc bump --minor
cc sync
cc publish
cc fmt
cc --help
```
