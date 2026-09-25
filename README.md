# crates-monorepo

A unified Cargo workspace that consolidates all 21 Rust crates from the
[crates-dev](https://github.com/crates-dev) GitHub organization into a single
repository, preserving full commit history for each crate.

## Layout

```
crates-monorepo/
├── Cargo.toml                 # workspace manifest
├── LICENSE                    # MIT
├── README.md                  # this file
└── crates/
    ├── bin-encode-decode/         # ← git subtree from crates-dev/bin-encode-decode
    ├── china_identification_card/
    ├── chunkify/
    ├── clonelicious/
    ├── color-output/
    ├── compare-version/
    ├── file-operation/
    ├── future-fn/
    ├── gtl/
    ├── hot-restart/
    ├── instrument-level/
    ├── jwt-service/
    ├── lombok-macros/
    ├── recoverable-spawn/
    ├── recoverable-thread-pool/
    ├── server-manager/
    ├── std-macro-extensions/
    ├── tcp-request/
    ├── tcplane/
    ├── udp/
    └── udp-request/
```

Each subdirectory under `crates/` is a self-contained crate with its own
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
cargo test -p gtl

# Lint / format all
cargo clippy --workspace --all-targets
cargo fmt --all
```

## Crate origin

| crate                          | upstream                                              |
| ------------------------------ | ----------------------------------------------------- |
| `bin-encode-decode`            | https://github.com/crates-dev/bin-encode-decode       |
| `china_identification_card`    | https://github.com/crates-dev/china_identification_card |
| `chunkify`                     | https://github.com/crates-dev/chunkify                |
| `clonelicious`                 | https://github.com/crates-dev/clonelicious            |
| `color-output`                 | https://github.com/crates-dev/color-output            |
| `compare-version`              | https://github.com/crates-dev/compare-version         |
| `file-operation`               | https://github.com/crates-dev/file-operation          |
| `future-fn`                    | https://github.com/crates-dev/future-fn               |
| `gtl`                          | https://github.com/crates-dev/gtl                     |
| `hot-restart`                  | https://github.com/crates-dev/hot-restart             |
| `instrument-level`             | https://github.com/crates-dev/instrument-level        |
| `jwt-service`                  | https://github.com/crates-dev/jwt-service             |
| `lombok-macros`                | https://github.com/crates-dev/lombok-macros           |
| `recoverable-spawn`            | https://github.com/crates-dev/recoverable-spawn       |
| `recoverable-thread-pool`      | https://github.com/crates-dev/recoverable-thread-pool |
| `server-manager`               | https://github.com/crates-dev/server-manager          |
| `std-macro-extensions`         | https://github.com/crates-dev/std-macro-extensions    |
| `tcp-request`                  | https://github.com/crates-dev/tcp-request             |
| `tcplane`                      | https://github.com/crates-dev/tcplane                 |
| `udp`                          | https://github.com/crates-dev/udp                     |
| `udp-request`                  | https://github.com/crates-dev/udp-request             |

## Merge provenance

Every import was added with `git subtree add --prefix=crates/<name>`. Each crate's
original history is intact and reachable; the merge commit at the top of each
crate's tree has the message `merge: import <name> from crates-dev/<name>@master`.

To pull new commits from upstream into a single crate:

```bash
git subtree pull --prefix=crates/jwt-service \
    https://github.com/crates-dev/jwt-service.git master \
    -m "merge: pull jwt-service from crates-dev"
```

## License

MIT — see `LICENSE`.