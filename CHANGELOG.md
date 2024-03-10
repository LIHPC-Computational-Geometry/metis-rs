# Changelog

## Version 0.2.1 (2024-03-10)

[metis-sys-0.2.0...0.2.1](https://github.com/LIHPC-Computational-Geometry/metis-rs/compare/metis-0.2.0...metis-0.2.1)

### Fixed

- move `vendor` library in `metis-sys` [#29](https://github.com/LIHPC-Computational-Geometry/metis-rs/pull/29)

## Version 0.2.0 (2024-03-06)

[metis-sys-0.1.2...0.2.0](https://github.com/LIHPC-Computational-Geometry/metis-rs/compare/metis-0.1.2...metis-0.2.0)

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
