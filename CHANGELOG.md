# Changelog

## [v0.4.0] - 2021-04-14

### Changed
- `DisjointSet::new` and `DisjointSetVec::new` are not `const` anymore. 
- Improved the minimum supported Rust version from `1.56` to `1.31`.


## [v0.3.0] - 2021-04-14

### Changed
- Changed `DisjointSet::new` to take no arguments and construct an empty `DisjointSet`. 
- Retained the old functionality of `DisjointSet::new` under the new name `DisjointSet::with_len`.

### Added
- Implemented `Eq` for `DisjointSet` and `DisjointSetVec`.

## [v0.2.0] - 2021-04-13

### Added
- Implemented `Debug`, `Clone`, `PartialEq` and `Default` for `DisjointSet`.
- Added `with_capacity`, `add_singleton`, and `get_sets` to `DisjointSet`.  
- Added `DisjointSetVec`.

## [v0.1.0] - 2021-04-12

- Initial release

[v0.1.0]: https://github.com/jogru0/disjoint/commit/15bb8dce2a5f33812fe237d19354a792612fd92c
[v0.2.0]: https://github.com/jogru0/disjoint/compare/v0.1.0...v0.2.0
[v0.3.0]: https://github.com/jogru0/disjoint/compare/v0.2.0...v0.3.0
[v0.4.0]: https://github.com/jogru0/disjoint/compare/v0.3.0...v0.4.0
