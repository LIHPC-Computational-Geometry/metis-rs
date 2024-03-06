# Changelog

## Unreleased
[metis-sys-0.2.1...HEAD](https://github.com/LIHPC-Computational-Geometry/metis-rs/compare/metis-sys-0.2.1...HEAD)

### Added

- Builtin metis with the new `vendored` feature, enabled by default [#16](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/16)
- Convert from sprs matrices [#10](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/10)
- Add unchecked constructors [#13](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/13)

### Changed

- Remove mutability requirement on input from public facing API [#18](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/18)
- Remove numbering feature, now only Rust (or C) 0-based arrays are supported [#13](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/13)

### Documentation

- Better documentation from metis user guide [#14](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/14)
- Improve examples to use `?` instead of `unwrap` [#9](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/9)

### Contributors

Thanks to all contributors to this release:

- @cedricchevalier19
- @gfaster
- @hhirtz
- @oisyn

## Version 0.2.1

[metis-sys-0.2.0...metis-sys-0.2.1](https://github.com/LIHPC-Computational-Geometry/metis-rs/compare/metis-sys-0.2.0...metis-sys-0.2.1)

## Changed

- Link to libclang at runtime [#7](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/7)
- Update to bindgen 0.66 [#7](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/7)

## Version 0.2.0

[metis-sys-0.1.0...metis-sys-0.2.0](https://github.com/LIHPC-Computational-Geometry/metis-rs/compare/metis-sys-0.1.0...metis-sys-0.2.0)

## Breaking change

- Only METIS-related functions are now exposed. For access to other functions, use the `libc` crate. If a METIS function you were using has been mistakenly removed, please file a [bug report]. Thank you!

## Other changes

- Update `bindgen` to v0.65
- Update to rust edition 2021

[bug report]: https://github.com/LIHPC-Computational-Geometry/metis-rs/issues