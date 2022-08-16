# Rust-AV examples

[![Actions Status](https://github.com/rust-av/examples/workflows/examples/badge.svg)](https://github.com/rust-av/examples/actions)

A series of some multimedia examples.  
Feel free to add your own example opening up a new PR.

## Build all examples

```bash
cargo build --workspace
```

**NOTE**: To build all examples, have a look at our [CI](https://github.com/rust-av/examples/blob/master/.github/workflows/examples.yml) to install the dependencies both on Linux and
Windows operating systems.

## Build a single example

```bash
cargo build --package EXAMPLE_NAME
```

**NOTE**: To build a single example, always have a look at [CI](https://github.com/rust-av/examples/blob/master/.github/workflows/examples.yml) to install the necessary dependencies
on all supported operating systems.

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

## Building with vcpkg for Windows x64

To build with [vcpkg](https://vcpkg.io/en/index.html), you need to follow these
steps:

1. Install `pkg-config` through [chocolatey](https://chocolatey.org/)

       choco install pkgconfiglite

2. Install `libvpx`

       vcpkg install libvpx:x64-windows

3. Install `opus`

       vcpkg install opus:x64-windows

4. Add to the `PKG_CONFIG_PATH` environment variable the path `$VCPKG_INSTALLATION_ROOT\installed\x64-windows\lib\pkgconfig`

5. Build code

       cargo build --workspace

To speed up the computation, you can build your packages only in `Release` mode
adding the `set(VCPKG_BUILD_TYPE release)` line to the
`$VCPKG_INSTALLATION_ROOT\triplets\x64-windows.cmake` file.

Building for Windows x86 is the same, just replace `x64` with `x86` in the
steps above.

## License

Released under the [MIT License](LICENSE).
