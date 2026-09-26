<center>

# color-log

[![](https://img.shields.io/crates/v/color-log.svg)](https://crates.io/crates/color-log)
[![](https://img.shields.io/crates/d/color-log.svg)](https://img.shields.io/crates/d/color-log.svg)
[![](https://docs.rs/color-log/badge.svg)](https://docs.rs/color-log)
[![](https://github.com/crates-dev/ctares/workflows/Rust/badge.svg)](https://github.com/crates-dev/ctares/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/color-log.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/color-log/latest/)

## Description

> A Rust logging library that supports both asynchronous and synchronous logging. It provides multiple log levels, such as error, info, and debug. Users can define custom log handling methods and configure log file paths. The library supports log rotation, automatically creating a new log file when the current file reaches the specified size limit. It allows flexible logging configurations, making it suitable for both high-performance asynchronous applications and traditional synchronous logging scenarios. The asynchronous mode utilizes Tokio's async channels for efficient log buffering, while the synchronous mode writes logs directly to the file system.

## Log Storage Location Description

> Three directories will be created under the user-specified directory: one for error logs, one for info logs, and one for debug logs. Each of these directories will contain a subdirectory named by the date, and the log files within these subdirectories will be named in the format `timestamp.index.log`.

## Installation

To install `color-log` run cmd:

```shell
cargo add color-log
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).