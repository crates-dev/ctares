<center>

# std-macro-extensions

[![](https://img.shields.io/crates/v/std-macro-extensions.svg)](https://crates.io/crates/std-macro-extensions)
[![](https://img.shields.io/crates/d/std-macro-extensions.svg)](https://img.shields.io/crates/d/std-macro-extensions.svg)
[![](https://docs.rs/std-macro-extensions/badge.svg)](https://docs.rs/std-macro-extensions)
[![](https://github.com/crates-dev/std-macro-extensions/workflows/Rust/badge.svg)](https://github.com/crates-dev/std-macro-extensions/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/std-macro-extensions.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/std-macro-extensions/latest/)

> A collection of macro extensions for Rust's standard library data structures, simplifying the creation and manipulation of common collections such as HashMap, Vec, and more.

## Features

- **Simplified Initialization**: Use macros to easily create instances of common data structures.
- **Supports Various Data Structures**: Includes macros for `Vec`, `HashMap`, `Arc`, and more.
- **Easy to Use**: Intuitive syntax for quick data structure creation.

## Installation

To install `std-macro-extensions` run cmd:

```sh
cargo add std-macro-extensions
```

## Available Macros

- `arc!`: Creates an `Arc<T>`.
- `vector!`: Creates a `Vec<T>`.
- `map!`: Creates a `HashMap<K, V>`.
- `set!`: Creates a `HashSet<T>`.
- `b_tree_map!`: Creates a `BTreeMap<K, V>`.
- `b_tree_set!`: Creates a `BTreeSet<T>`.
- `list!`: Creates a `LinkedList<T>`.
- `heap!`: Creates a `BinaryHeap<T>`.
- `string!`: Creates a `String`.
- `boxed!`: Creates a `Box<T>`.
- `rc!`: Creates an `Rc<T>`.
- `arc!`: Creates an `Arc<T>`.
- `mutex!`: Creates a `Mutex<T>`.
- `rw_lock!`: Creates a `RwLock<T>`.
- `cell!`: Creates a `Cell<T>`.
- `ref_cell!`: Creates a `RefCell<T>`.
- `vector_deque!`: Creates a `VecDeque<T>`.
- `join_paths!`: Combines multiple paths into a single valid path, handling overlapping slashes.
- `cin!`: Reads a line of input from the standard input.
- `cin_parse!`: Parses input into a specified type.
- `cout!`: Prints formatted output to the standard output.
- `endl!`: Prints a newline character to the standard output.
- `cout_endl!`: Prints formatted output followed by a newline character to the standard output.
- `execute!`: Executes a function with the provided arguments.
- `execute_async!`: Executes an asynchronous function with the provided arguments.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
