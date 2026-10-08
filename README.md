# ctares

A unified Cargo workspace holding 27 Rust crates under the
[crates-dev](https://github.com/crates-dev) GitHub organization, each a
self-contained directory with its own `Cargo.toml`, `src/`, tests, and examples,
stitched together by the root `Cargo.toml`, with each crate's full commit history
preserved.

## Packages

| crate                       | crates.io                                                                                                                         | docs.rs                                                                                                      |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `bin-encode-decode`         | [![crates.io](https://img.shields.io/crates/v/bin-encode-decode.svg)](https://crates.io/crates/bin-encode-decode)                 | [![docs.rs](https://docs.rs/bin-encode-decode/badge.svg)](https://docs.rs/bin-encode-decode)                 |
| `china_identification_card` | [![crates.io](https://img.shields.io/crates/v/china_identification_card.svg)](https://crates.io/crates/china_identification_card) | [![docs.rs](https://docs.rs/china_identification_card/badge.svg)](https://docs.rs/china_identification_card) |
| `chunkify`                  | [![crates.io](https://img.shields.io/crates/v/chunkify.svg)](https://crates.io/crates/chunkify)                                   | [![docs.rs](https://docs.rs/chunkify/badge.svg)](https://docs.rs/chunkify)                                   |
| `clonelicious`              | [![crates.io](https://img.shields.io/crates/v/clonelicious.svg)](https://crates.io/crates/clonelicious)                           | [![docs.rs](https://docs.rs/clonelicious/badge.svg)](https://docs.rs/clonelicious)                           |
| `color-log`                 | [![crates.io](https://img.shields.io/crates/v/color-log.svg)](https://crates.io/crates/color-log)                                 | [![docs.rs](https://docs.rs/color-log/badge.svg)](https://docs.rs/color-log)                                 |
| `color-output`              | [![crates.io](https://img.shields.io/crates/v/color-output.svg)](https://crates.io/crates/color-output)                           | [![docs.rs](https://docs.rs/color-output/badge.svg)](https://docs.rs/color-output)                           |
| `compare_version`           | [![crates.io](https://img.shields.io/crates/v/compare_version.svg)](https://crates.io/crates/compare_version)                     | [![docs.rs](https://docs.rs/compare_version/badge.svg)](https://docs.rs/compare_version)                     |
| `crate-cli`                 | [![crates.io](https://img.shields.io/crates/v/crate-cli.svg)](https://crates.io/crates/crate-cli)                                 | [![docs.rs](https://docs.rs/crate-cli/badge.svg)](https://docs.rs/crate-cli)                                 |
| `file-operation`            | [![crates.io](https://img.shields.io/crates/v/file-operation.svg)](https://crates.io/crates/file-operation)                       | [![docs.rs](https://docs.rs/file-operation/badge.svg)](https://docs.rs/file-operation)                       |
| `future-fn`                 | [![crates.io](https://img.shields.io/crates/v/future-fn.svg)](https://crates.io/crates/future-fn)                                 | [![docs.rs](https://docs.rs/future-fn/badge.svg)](https://docs.rs/future-fn)                                 |
| `hot-restart`               | [![crates.io](https://img.shields.io/crates/v/hot-restart.svg)](https://crates.io/crates/hot-restart)                             | [![docs.rs](https://docs.rs/hot-restart/badge.svg)](https://docs.rs/hot-restart)                             |
| `instrument-level`          | [![crates.io](https://img.shields.io/crates/v/instrument-level.svg)](https://crates.io/crates/instrument-level)                   | [![docs.rs](https://docs.rs/instrument-level/badge.svg)](https://docs.rs/instrument-level)                   |
| `jwt-service`               | [![crates.io](https://img.shields.io/crates/v/jwt-service.svg)](https://crates.io/crates/jwt-service)                             | [![docs.rs](https://docs.rs/jwt-service/badge.svg)](https://docs.rs/jwt-service)                             |
| `lombok-macros`             | [![crates.io](https://img.shields.io/crates/v/lombok-macros.svg)](https://crates.io/crates/lombok-macros)                         | [![docs.rs](https://docs.rs/lombok-macros/badge.svg)](https://docs.rs/lombok-macros)                         |
| `recoverable-spawn`         | [![crates.io](https://img.shields.io/crates/v/recoverable-spawn.svg)](https://crates.io/crates/recoverable-spawn)                 | [![docs.rs](https://docs.rs/recoverable-spawn/badge.svg)](https://docs.rs/recoverable-spawn)                 |
| `recoverable-thread-pool`   | [![crates.io](https://img.shields.io/crates/v/recoverable-thread-pool.svg)](https://crates.io/crates/recoverable-thread-pool)     | [![docs.rs](https://docs.rs/recoverable-thread-pool/badge.svg)](https://docs.rs/recoverable-thread-pool)     |
| `server-manager`            | [![crates.io](https://img.shields.io/crates/v/server-manager.svg)](https://crates.io/crates/server-manager)                       | [![docs.rs](https://docs.rs/server-manager/badge.svg)](https://docs.rs/server-manager)                       |
| `std-macro-extensions`      | [![crates.io](https://img.shields.io/crates/v/std-macro-extensions.svg)](https://crates.io/crates/std-macro-extensions)           | [![docs.rs](https://docs.rs/std-macro-extensions/badge.svg)](https://docs.rs/std-macro-extensions)           |
| `stripe-pay-client`         | [![crates.io](https://img.shields.io/crates/v/stripe-pay-client.svg)](https://crates.io/crates/stripe-pay-client)                 | [![docs.rs](https://docs.rs/stripe-pay-client/badge.svg)](https://docs.rs/stripe-pay-client)                 |
| `stripe-pay-core`           | [![crates.io](https://img.shields.io/crates/v/stripe-pay-core.svg)](https://crates.io/crates/stripe-pay-core)                     | [![docs.rs](https://docs.rs/stripe-pay-core/badge.svg)](https://docs.rs/stripe-pay-core)                     |
| `stripe-pay-server`         | [![crates.io](https://img.shields.io/crates/v/stripe-pay-server.svg)](https://crates.io/crates/stripe-pay-server)                 | [![docs.rs](https://docs.rs/stripe-pay-server/badge.svg)](https://docs.rs/stripe-pay-server)                 |
| `system-time`               | [![crates.io](https://img.shields.io/crates/v/system-time.svg)](https://crates.io/crates/system-time)                             | [![docs.rs](https://docs.rs/system-time/badge.svg)](https://docs.rs/system-time)                             |
| `tcp-request`               | [![crates.io](https://img.shields.io/crates/v/tcp-request.svg)](https://crates.io/crates/tcp-request)                             | [![docs.rs](https://docs.rs/tcp-request/badge.svg)](https://docs.rs/tcp-request)                             |
| `tcplane`                   | [![crates.io](https://img.shields.io/crates/v/tcplane.svg)](https://crates.io/crates/tcplane)                                     | [![docs.rs](https://docs.rs/tcplane/badge.svg)](https://docs.rs/tcplane)                                     |
| `tokio-broadcast`           | [![crates.io](https://img.shields.io/crates/v/tokio-broadcast.svg)](https://crates.io/crates/tokio-broadcast)                     | [![docs.rs](https://docs.rs/tokio-broadcast/badge.svg)](https://docs.rs/tokio-broadcast)                     |
| `udp`                       | [![crates.io](https://img.shields.io/crates/v/udp.svg)](https://crates.io/crates/udp)                                             | [![docs.rs](https://docs.rs/udp/badge.svg)](https://docs.rs/udp)                                             |
| `udp-request`               | [![crates.io](https://img.shields.io/crates/v/udp-request.svg)](https://crates.io/crates/udp-request)                             | [![docs.rs](https://docs.rs/udp-request/badge.svg)](https://docs.rs/udp-request)                             |

## Source

| crate                       | source                                                                     |
| --------------------------- | -------------------------------------------------------------------------- |
| `bin-encode-decode`         | https://github.com/crates-dev/ctares/tree/master/bin-encode-decode         |
| `china_identification_card` | https://github.com/crates-dev/ctares/tree/master/china_identification_card |
| `chunkify`                  | https://github.com/crates-dev/ctares/tree/master/chunkify                  |
| `clonelicious`              | https://github.com/crates-dev/ctares/tree/master/clonelicious              |
| `color-log`                 | https://github.com/crates-dev/ctares/tree/master/color-log                 |
| `color-output`              | https://github.com/crates-dev/ctares/tree/master/color-output              |
| `compare_version`           | https://github.com/crates-dev/ctares/tree/master/compare-version           |
| `crate-cli`                 | https://github.com/crates-dev/ctares/tree/master/crate-cli                 |
| `file-operation`            | https://github.com/crates-dev/ctares/tree/master/file-operation            |
| `future-fn`                 | https://github.com/crates-dev/ctares/tree/master/future-fn                 |
| `hot-restart`               | https://github.com/crates-dev/ctares/tree/master/hot-restart               |
| `instrument-level`          | https://github.com/crates-dev/ctares/tree/master/instrument-level          |
| `jwt-service`               | https://github.com/crates-dev/ctares/tree/master/jwt-service               |
| `lombok-macros`             | https://github.com/crates-dev/ctares/tree/master/lombok-macros             |
| `recoverable-spawn`         | https://github.com/crates-dev/ctares/tree/master/recoverable-spawn         |
| `recoverable-thread-pool`   | https://github.com/crates-dev/ctares/tree/master/recoverable-thread-pool   |
| `server-manager`            | https://github.com/crates-dev/ctares/tree/master/server-manager            |
| `std-macro-extensions`      | https://github.com/crates-dev/ctares/tree/master/std-macro-extensions      |
| `stripe-pay-client`         | https://github.com/crates-dev/ctares/tree/master/stripe-pay-client         |
| `stripe-pay-core`           | https://github.com/crates-dev/ctares/tree/master/stripe-pay-core           |
| `stripe-pay-server`         | https://github.com/crates-dev/ctares/tree/master/stripe-pay-server         |
| `system-time`               | https://github.com/crates-dev/ctares/tree/master/system-time               |
| `tcp-request`               | https://github.com/crates-dev/ctares/tree/master/tcp-request               |
| `tcplane`                   | https://github.com/crates-dev/ctares/tree/master/tcplane                   |
| `tokio-broadcast`           | https://github.com/crates-dev/ctares/tree/master/tokio-broadcast           |
| `udp`                       | https://github.com/crates-dev/ctares/tree/master/udp                       |
| `udp-request`               | https://github.com/crates-dev/ctares/tree/master/udp-request               |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
