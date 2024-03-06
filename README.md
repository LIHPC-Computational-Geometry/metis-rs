# metis-rs

**metis-rs** is a Rust library providing idiomatic bindings to [libmetis][METIS], a library for graph and mesh
partitioning. It is made to be used with Rust version 1.67.0 or above.

## Getting Started

Library released on [crates.io](https://crates.io/crates/metis-rs). To use it, add the following to your `Cargo.toml`:

```toml
[dependencies]
metis-rs = "0.2"
```

The list of available versions and a change log are available in the [CHANGELOG.md](CHANGELOG.md) file.

## Features

### Use of Vendored Feature

The `vendored` feature enables metis-rs to build METIS from source and link to it statically. If not enabled, metis-rs
looks for an existing installation and links to it dynamically.

### Use of System-wide Feature

The `use-system` feature enables metis-rs to use the system-wide installation of METIS. If not enabled, metis-rs will
refer to its own version of METIS.

Please note, `vendored` and `use-system` features are mutually exclusive.

## Guidance for non-standard METIS installations

If you enabled the `use-system` feature and METIS is installed in a non-standard location, you must set the following
environment variables:
```bash
export METISDIR=path/to/your/metis/installation
export CPATH="$METISDIR/include"
export RUSTFLAGS="-L$METISDIR/lib"
```

`$METISDIR` must point to a directory containing both `lib/` and `include/` directories with METIS's shared libraries and headers, respectively.

## Building the documentation

To build the documentation, especially if METIS is installed in a non-standard location, set the `RUSTDOCFLAGS` environment variable:

```bash
export RUSTDOCFLAGS="-L$METISDIR/lib"
```
Then the following command will generate and open the documentation:
```bash
cargo doc --no-deps --open
```

## License

metis-rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0). Refer to `LICENSE-APACHE` and `LICENSE-MIT` for more details.

[METIS]: http://glaros.dtc.umn.edu/gkhome/metis/metis/overview