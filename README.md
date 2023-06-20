# metis-rs

Idiomatic bindings to [libmetis][METIS], a graph and mesh partitioner.

- Documentation (latest stable): <https://docs.rs/metis>
- Documentation (master branch):
  <https://lihpc-computational-geometry.github.io/metis-rs/metis/>

## Usage

Example with the `sprs` sparse matrix crate.

```sh
# Enable the `sprs` feature to partition sprs matrices.
# See below the example for other features.
cargo add metis --features sprs
```

```rust
use metis::Partition as _;

// Build this graph:
//
//     0 -- 1
//     |    |
//     3 -- 2
//
const NUM_VERTICES: usize = 4;
let mut a = sprs::TriMatI::new((NUM_VERTICES, NUM_VERTICES));
a.add_triplet(0, 1, 1);
a.add_triplet(1, 2, 1); // All edges have the
a.add_triplet(2, 3, 1); // same weight (1).
a.add_triplet(3, 0, 1);
let a = a.to_csr();

let mut partition = [0; NUM_VERTICES];
a.setup_partition(2) // Partition with 2 parts.
    .set_vwgt(&[2, 2, 1, 1]) // Set the weights of the vertices.
    .part_kway(&mut partition)?;

eprintln!("{:?}", partition);
```

This crate can currently work with:

- [sprs](https://docs.rs/sprs)

Do not hesitate to send a PR to support your graph crate.

## Building

Prerequisites:

- METIS
- clang v5.0 or above
- Rust v1.60.0 or above

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
