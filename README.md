# ctares

A unified Cargo workspace that consolidates all 24 Rust crates from the
[crates-dev](https://github.com/crates-dev) GitHub organization into a single
repository, preserving full commit history for each crate.

## Layout

```
ctares/
├── Cargo.toml                 # workspace manifest
├── LICENSE                    # MIT
├── README.md                  # this file
├── <crate>/ ...
    ├── bin-encode-decode/         # ← git subtree from crates-dev/bin-encode-decode
    ├── china_identification_card/
    ├── chunkify/
    ├── clonelicious/
    ├── color-log/
    ├── color-output/
    ├── compare-version/
    ├── file-operation/
    ├── future-fn/
    ├── hot-restart/
    ├── instrument-level/
    ├── jwt-service/
    ├── lombok-macros/
    ├── recoverable-spawn/
    ├── recoverable-thread-pool/
    ├── server-manager/
    ├── std-macro-extensions/
    ├── system-time/
    ├── tcp-request/
    ├── tcplane/
    ├── tokio-broadcast/
    ├── udp/
    └── udp-request/
```

Each subdirectory in the repository root is a self-contained crate with its own
`Cargo.toml`, `src/`, tests, and examples. They are stitched together into a
single workspace via the root `Cargo.toml`.

## Working with the workspace

```bash
# Build everything
cargo build --workspace

# Test everything
cargo test --workspace

# Build / test one crate
cargo build -p jwt-service

# Lint / format all
cargo clippy --workspace --all-targets
cargo fmt --all
```

## Packages

| crate | crates.io | docs.rs |
| ----- | --------- | ------- |
| `bin-encode-decode` | [![crates.io](https://img.shields.io/crates/v/bin-encode-decode.svg)](https://crates.io/crates/bin-encode-decode) | [![docs.rs](https://docs.rs/bin-encode-decode/badge.svg)](https://docs.rs/bin-encode-decode) |
| `china_identification_card` | [![crates.io](https://img.shields.io/crates/v/china_identification_card.svg)](https://crates.io/crates/china_identification_card) | [![docs.rs](https://docs.rs/china_identification_card/badge.svg)](https://docs.rs/china_identification_card) |
| `chunkify` | [![crates.io](https://img.shields.io/crates/v/chunkify.svg)](https://crates.io/crates/chunkify) | [![docs.rs](https://docs.rs/chunkify/badge.svg)](https://docs.rs/chunkify) |
| `clonelicious` | [![crates.io](https://img.shields.io/crates/v/clonelicious.svg)](https://crates.io/crates/clonelicious) | [![docs.rs](https://docs.rs/clonelicious/badge.svg)](https://docs.rs/clonelicious) |
| `color-log` | [![crates.io](https://img.shields.io/crates/v/color-log.svg)](https://crates.io/crates/color-log) | [![docs.rs](https://docs.rs/color-log/badge.svg)](https://docs.rs/color-log) |
| `color-output` | [![crates.io](https://img.shields.io/crates/v/color-output.svg)](https://crates.io/crates/color-output) | [![docs.rs](https://docs.rs/color-output/badge.svg)](https://docs.rs/color-output) |
| `compare-version` | [![crates.io](https://img.shields.io/crates/v/compare-version.svg)](https://crates.io/crates/compare-version) | [![docs.rs](https://docs.rs/compare-version/badge.svg)](https://docs.rs/compare-version) |
| `crate-cli` | [![crates.io](https://img.shields.io/crates/v/crate-cli.svg)](https://crates.io/crates/crate-cli) | [![docs.rs](https://docs.rs/crate-cli/badge.svg)](https://docs.rs/crate-cli) |
| `file-operation` | [![crates.io](https://img.shields.io/crates/v/file-operation.svg)](https://crates.io/crates/file-operation) | [![docs.rs](https://docs.rs/file-operation/badge.svg)](https://docs.rs/file-operation) |
| `future-fn` | [![crates.io](https://img.shields.io/crates/v/future-fn.svg)](https://crates.io/crates/future-fn) | [![docs.rs](https://docs.rs/future-fn/badge.svg)](https://docs.rs/future-fn) |
| `hot-restart` | [![crates.io](https://img.shields.io/crates/v/hot-restart.svg)](https://crates.io/crates/hot-restart) | [![docs.rs](https://docs.rs/hot-restart/badge.svg)](https://docs.rs/hot-restart) |
| `instrument-level` | [![crates.io](https://img.shields.io/crates/v/instrument-level.svg)](https://crates.io/crates/instrument-level) | [![docs.rs](https://docs.rs/instrument-level/badge.svg)](https://docs.rs/instrument-level) |
| `jwt-service` | [![crates.io](https://img.shields.io/crates/v/jwt-service.svg)](https://crates.io/crates/jwt-service) | [![docs.rs](https://docs.rs/jwt-service/badge.svg)](https://docs.rs/jwt-service) |
| `lombok-macros` | [![crates.io](https://img.shields.io/crates/v/lombok-macros.svg)](https://crates.io/crates/lombok-macros) | [![docs.rs](https://docs.rs/lombok-macros/badge.svg)](https://docs.rs/lombok-macros) |
| `recoverable-spawn` | [![crates.io](https://img.shields.io/crates/v/recoverable-spawn.svg)](https://crates.io/crates/recoverable-spawn) | [![docs.rs](https://docs.rs/recoverable-spawn/badge.svg)](https://docs.rs/recoverable-spawn) |
| `recoverable-thread-pool` | [![crates.io](https://img.shields.io/crates/v/recoverable-thread-pool.svg)](https://crates.io/crates/recoverable-thread-pool) | [![docs.rs](https://docs.rs/recoverable-thread-pool/badge.svg)](https://docs.rs/recoverable-thread-pool) |
| `server-manager` | [![crates.io](https://img.shields.io/crates/v/server-manager.svg)](https://crates.io/crates/server-manager) | [![docs.rs](https://docs.rs/server-manager/badge.svg)](https://docs.rs/server-manager) |
| `std-macro-extensions` | [![crates.io](https://img.shields.io/crates/v/std-macro-extensions.svg)](https://crates.io/crates/std-macro-extensions) | [![docs.rs](https://docs.rs/std-macro-extensions/badge.svg)](https://docs.rs/std-macro-extensions) |
| `system-time` | [![crates.io](https://img.shields.io/crates/v/system-time.svg)](https://crates.io/crates/system-time) | [![docs.rs](https://docs.rs/system-time/badge.svg)](https://docs.rs/system-time) |
| `tcp-request` | [![crates.io](https://img.shields.io/crates/v/tcp-request.svg)](https://crates.io/crates/tcp-request) | [![docs.rs](https://docs.rs/tcp-request/badge.svg)](https://docs.rs/tcp-request) |
| `tcplane` | [![crates.io](https://img.shields.io/crates/v/tcplane.svg)](https://crates.io/crates/tcplane) | [![docs.rs](https://docs.rs/tcplane/badge.svg)](https://docs.rs/tcplane) |
| `tokio-broadcast` | [![crates.io](https://img.shields.io/crates/v/tokio-broadcast.svg)](https://crates.io/crates/tokio-broadcast) | [![docs.rs](https://docs.rs/tokio-broadcast/badge.svg)](https://docs.rs/tokio-broadcast) |
| `udp` | [![crates.io](https://img.shields.io/crates/v/udp.svg)](https://crates.io/crates/udp) | [![docs.rs](https://docs.rs/udp/badge.svg)](https://docs.rs/udp) |
| `udp-request` | [![crates.io](https://img.shields.io/crates/v/udp-request.svg)](https://crates.io/crates/udp-request) | [![docs.rs](https://docs.rs/udp-request/badge.svg)](https://docs.rs/udp-request) |

## Crate origin

| crate                          | upstream                                              |
| ------------------------------ | ----------------------------------------------------- |
| `bin-encode-decode`            | https://github.com/crates-dev/bin-encode-decode       |
| `china_identification_card`    | https://github.com/crates-dev/china_identification_card |
| `chunkify`                     | https://github.com/crates-dev/chunkify                |
| `clonelicious`                 | https://github.com/crates-dev/clonelicious            |
| `color-log`                    | https://github.com/crates-dev/color-log               |
| `color-output`                 | https://github.com/crates-dev/color-output            |
| `compare-version`              | https://github.com/crates-dev/compare-version         |
| `crate-cli`                   | native to this repository (not a subtree import)      |
| `file-operation`               | https://github.com/crates-dev/file-operation          |
| `future-fn`                    | https://github.com/crates-dev/future-fn               |
| `hot-restart`                  | https://github.com/crates-dev/hot-restart             |
| `instrument-level`             | https://github.com/crates-dev/instrument-level        |
| `jwt-service`                  | https://github.com/crates-dev/jwt-service             |
| `lombok-macros`                | https://github.com/crates-dev/lombok-macros           |
| `recoverable-spawn`            | https://github.com/crates-dev/recoverable-spawn       |
| `recoverable-thread-pool`      | https://github.com/crates-dev/recoverable-thread-pool |
| `server-manager`               | https://github.com/crates-dev/server-manager          |
| `std-macro-extensions`         | https://github.com/crates-dev/std-macro-extensions    |
| `system-time`                  | https://github.com/crates-dev/system-time             |
| `tcp-request`                  | https://github.com/crates-dev/tcp-request             |
| `tcplane`                      | https://github.com/crates-dev/tcplane                 |
| `tokio-broadcast`              | https://github.com/crates-dev/tokio-broadcast         |
| `udp`                          | https://github.com/crates-dev/udp                     |
| `udp-request`                  | https://github.com/crates-dev/udp-request             |

## Merge provenance

Every import was added with `git subtree add --prefix=<name>`. Each crate's
original history is intact and reachable; the merge commit at the top of each
crate's tree has the message `merge: import <name> from crates-dev/<name>@master`.

To pull new commits from upstream into a single crate:

```bash
git subtree pull --prefix=jwt-service \
    https://github.com/crates-dev/jwt-service.git master \
    -m "merge: pull jwt-service from crates-dev"
```

## License

MIT — see `LICENSE`.