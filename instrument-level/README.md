<center>

## instrument-level

[![](https://img.shields.io/crates/v/instrument-level.svg)](https://crates.io/crates/instrument-level)
[![](https://img.shields.io/crates/d/instrument-level.svg)](https://img.shields.io/crates/d/instrument-level.svg)
[![](https://docs.rs/instrument-level/badge.svg)](https://docs.rs/instrument-level)
[![](https://github.com/crates-dev/instrument-level/workflows/Rust/badge.svg)](https://github.com/crates-dev/instrument-level/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/instrument-level.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/instrument-level/latest/)

> A Rust procedural macro collection providing convenient tracing instrumentation macros for different log levels (trace, debug, info, warn, error). This crate simplifies the process of adding tracing spans to functions with pre-configured log levels.

## Installation

To use this crate, you can run cmd:

```shell
cargo add instrument-level
```

## Usage

This crate provides five attribute macros for different tracing log levels:

- `#[instrument_trace]` - Trace Level Instrumentation - Use this macro to add trace-level logging instrumentation to functions. This is the most verbose log level and automatically excludes all function arguments from span fields using `skip_all`.

- `#[instrument_debug]` - Debug Level Instrumentation - Use this macro to add debug-level logging instrumentation to functions. Ideal for development and debugging purposes.

- `#[instrument_info]` - Info Level Instrumentation - Use this macro to add info-level logging instrumentation to functions. Suitable for general informational messages.

- `#[instrument_warn]` - Warning Level Instrumentation - Use this macro to add warning-level logging instrumentation to functions. Use for potentially harmful situations.

- `#[instrument_error]` - Error Level Instrumentation - Use this macro to add error-level logging instrumentation to functions. Use for error conditions that don't necessarily stop program execution. Each macro accepts optional tracing parameters such as `target`, `name`, `skip`, `fields`, etc., which can be used to customize the span behavior according to your needs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
