# metis-rs

Idiomatic bindings to [libmetis][METIS], a graph and mesh partitioner.

## Building

Prerequisites:

- METIS
- clang v5.0 or above
- Rust v1.61.0 or above

Bindings to METIS are made on the fly.  If METIS is installed in a non-standard
location, please use the following commands:

    export METISDIR=path/to/your/metis/installation
    export CPATH="$METISDIR/include"
    export RUSTFLAGS="-L$METISDIR/lib"

The environment variable `$METISDIR` must point to a directory containing a
`lib/` and a `include/` directory containing the shared libraries and the
headers of METIS, respectively.

Once these variables are set, you can build the bindings with `cargo build`.

### Build the documentation

If your METIS installation lies in a non-standard path, you will need to set
the `RUSTDOCFLAGS` environment variable to build the documentation:

    export RUSTDOCFLAGS="-L$METISDIR/lib"

Then you can call `cargo doc --no-deps --open`.

## License

This program is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).  See `LICENSE-APACHE` and `LICENSE-MIT` for
details.

[METIS]: http://glaros.dtc.umn.edu/gkhome/metis/metis/overview
