# Changelog

This changelog keeps track of all API changes.

Changes of implementation details, documentation, tests, etc., are not documented here. Take a look at the commit history linked for each version bump if you are interested in that as well.


## [v0.7.0] - 2024-02-06

### Added
- Implemented `root_of` for `DisjointSet` and `DisjointSetVec<T>`.

## [v0.6.0] - 2023-04-23

### Added
- Implemented `Deref<Target = [T]>` and `DerefMut` for `DisjointSetVec<T>`.
    - This enables all the (mutable) slice methods for `DisjointSetVec<T>`, e.g. `get_mut`, `first`, and many more.

### Removed
- Tidied up `DisjointSetVec<T>`.
    - Removed methods: `len`, `is_empty`, `get`, `iter`, and `iter_mut`.
    - Removed implementations: `Index<usize>`, and `IndexMut<usize>`.
    - All this functionality can still be used due to the new `Deref<Target = [T]>` and `DerefMut` implementations (`dsv.len()`, `dsv[0]`, etc., still work).

## [v0.5.0] - 2023-04-15

### Removed
- Removed `DisjointSetVec::get_index_sets`.
    - Migration: Replace `dsv.get_index_sets()` with `dsv.indices().sets()`.

### Changed
- Renamed `DisjointSet::get_sets` to `DisjointSet::sets`.

### Added
- Implemented `From<IntoVec>` for `DisjointSetVec<T>` for all `IntoVec` that satisfy `Vec<T> : From<IntoVec>`.
- Added `values` and `indices` to `DisjointSetVec<T>`.
- Added `disjoint_set_vec!` macro.

## [v0.4.0] - 2023-04-14

### Changed
- `DisjointSet::new` and `DisjointSetVec::new` are not `const` anymore. 
- Improved the minimum supported Rust version from `1.56` to `1.31`.

## [v0.3.0] - 2023-04-14

### Changed
- Changed `DisjointSet::new` to take no arguments and construct an empty `DisjointSet`. 
- Retained the old functionality of `DisjointSet::new` under the new name `DisjointSet::with_len`.

### Added
- Implemented `Eq` for `DisjointSet` and `DisjointSetVec<T>`.

## [v0.2.0] - 2023-04-13

### Added
- Implemented `Debug`, `Clone`, `PartialEq` and `Default` for `DisjointSet`.
- Added `with_capacity`, `add_singleton`, and `get_sets` to `DisjointSet`.  
- Added `DisjointSetVec<T>`.

## [v0.1.0] - 2023-04-12

- Initial release

[v0.1.0]: https://github.com/jogru0/disjoint/commit/15bb8dce2a5f33812fe237d19354a792612fd92c
[v0.2.0]: https://github.com/jogru0/disjoint/compare/v0.1.0...v0.2.0
[v0.3.0]: https://github.com/jogru0/disjoint/compare/v0.2.0...v0.3.0
[v0.4.0]: https://github.com/jogru0/disjoint/compare/v0.3.0...v0.4.0
[v0.5.0]: https://github.com/jogru0/disjoint/compare/v0.4.0...v0.5.0
[v0.6.0]: https://github.com/jogru0/disjoint/compare/v0.5.0...v0.6.0
[v0.7.0]: https://github.com/jogru0/disjoint/compare/v0.6.0...v0.7.0