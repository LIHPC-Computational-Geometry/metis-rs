# scotch-rs

Idiomatic bindings to [libmetis][METIS], a graph and mesh partitioner.

## Building

Bindings to METIS are made on the fly.  You'll need METIS' header files and
shared libraries in order to build these bindings.  If those are in non-standard
locations, please use the following commands:

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
