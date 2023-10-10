# metis-rs

Idiomatic bindings to [libmetis][METIS], a graph and mesh partitioner.

## Usage

metis-rs requires clang v5.0 or above and Rust v1.60.0 or above.

```sh
# Use the vendored feature to build and link
# statically to METIS. This avoids issues with
# old or otherwise problematic installations.
cargo add metis --features vendored
```

Features:

- `vendored`: build METIS from source and link to it statically. Otherwise,
  metis-rs looks for an existing installation and links to it dynamically.

### When your METIS install is not found

If the vendored feature is disabled (the default), and if METIS is installed in
a non-standard location, you have to set the `CPATH` and `RUSTFLAGS` environment
variables like so:

    export METISDIR=path/to/your/metis/installation
    export CPATH="$METISDIR/include"
    export RUSTFLAGS="-L$METISDIR/lib"

The environment variable `$METISDIR` must point to a directory containing a
`lib/` and a `include/` directory containing the shared libraries and the
headers of METIS, respectively.

### Build the documentation

If the vendored feature is disabled (the default), and if METIS is installed in
a non-standard location, the additional `RUSTDOCFLAGS` environment variable
needs to be set.

    export RUSTDOCFLAGS="-L$METISDIR/lib"

Then you can call `cargo doc --no-deps --open`.

## License

This program is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).  See `LICENSE-APACHE` and `LICENSE-MIT` for
details.

[METIS]: http://glaros.dtc.umn.edu/gkhome/metis/metis/overview
