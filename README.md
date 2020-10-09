# Rust-AV examples

[![Actions Status](https://github.com/rust-av/examples/workflows/examples/badge.svg)](https://github.com/rust-av/examples/actions)

A series of some multimedia examples.  
Feel free to add your own example opening up a new PR.

## Build all examples

```bash
cargo build --workspace
```

**NOTE**: To build all the examples, first you need to install on your
operating system all the dynamic libraries required by each example.

## Build a single example

```bash
cargo build --package EXAMPLE_NAME
```

**NOTE**: To build an example, first you need to install on your
operating system all the required dynamic libraries.

For example, if you want to build the `streams-info` example:

```bash
cargo build --package streams-info
```

## Run an example

```bash
cargo run --bin EXAMPLE_NAME -- [EXAMPLE_ARGUMENTS]
```

For example, if you want to run the `streams-info` example:

```bash
cargo run --bin streams-info -- -i /path/to/your/matroska/file
```

## License

Released under the [MIT License](LICENSE).
