#![allow(unknown_lints)]
// Disabled to support old Rust versions.
// Activate these when developing locally on a recent toolchain.
/////////////////////////////
// #![warn(rustdoc::all)] //
// #![warn(unknown_lints)] //
/////////////////////////////
#![forbid(unsafe_code)]
#![forbid(non_ascii_idents)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction,
    explicit_outlives_requirements,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::pub_use,
    clippy::single_char_lifetime_names,
    clippy::missing_docs_in_private_items,
    clippy::std_instead_of_core,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::arithmetic_side_effects,
    clippy::missing_trait_methods,
    clippy::single_call_fn
)]

//! [![Tests](https://github.com/jogru0/disjoint/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/jogru0/disjoint/actions/workflows/tests.yml?query=branch:master)
//! [![Coverage](https://codecov.io/gh/jogru0/disjoint/branch/master/graph/badge.svg?token=D910NJAG7K)](https://app.codecov.io/gh/jogru0/disjoint/tree/master)
//! [![Crate](https://img.shields.io/crates/v/disjoint.svg?color=blue&logo=rust)](https://crates.io/crates/disjoint)
//! [![Docs](https://docs.rs/disjoint/badge.svg)](https://docs.rs/disjoint/latest/disjoint)
//!
//! This crate provides fast [disjoint-set data structure] implementations in 100% safe Rust.
//!
//! [`DisjointSet`] is a very lightweight disjoint-set data structure, with no additional data attached to the set elements. Use this if you manage the data associated to the elements yourself, and just want to keep track which elements are joined.
//!
//! [`DisjointSetVec<T>`] combines a `DisjointSet` with a `Vec<T>`, so it manages contiguous data entries `T` and keeps track of which entries are joined. Use this if you want the disjoint-set data structure to contain some additional data `T` for each element.
//!
//! # Examples
//!
//! Disjoint set data structures can be applied to find the [minimal spanning forest] of an [undirected edge-weighted graph]. Let's assume we work with the following graph interface:
//!
//! ```
//! trait Edge : Copy {
//!     fn first_vertex(&self) -> usize;
//!     fn second_vertex(&self) -> usize;
//! }
//!
//! trait Graph {
//!     type E : Edge;
//!     fn edges_ordered_by_weight(&self) -> Vec<Self::E>;
//!     fn number_vertices(&self) -> usize;
//!     fn new(edges: Vec<Self::E>) -> Self;
//! }
//! ```
//!
//! Then it's very straight-forward to use the provided [`DisjointSet`] struct and its methods [`is_joined`] and [`join`] to implement [Kruskal’s algorithm] to find the minimum spanning forest.
//!
//! ```
//! # trait Edge : Copy {
//! #     fn first_vertex(&self) -> usize;
//! #     fn second_vertex(&self) -> usize;
//! # }
//! #
//! # trait Graph {
//! #     type E : Edge;
//! #     fn edges_ordered_by_weight(&self) -> Vec<Self::E>;
//! #     fn number_vertices(&self) -> usize;
//! #     fn new(edges: Vec<Self::E>) -> Self;
//! # }
//! #
//! use disjoint::DisjointSet;
//!
//! fn minimum_spanning_forest<G : Graph>(graph: &G) -> G {
//!     let mut result_edges = Vec::new();
//!     let mut vertices = DisjointSet::with_len(graph.number_vertices());
//!
//!     for edge in graph.edges_ordered_by_weight() {
//!         if !vertices.is_joined(edge.first_vertex(), edge.second_vertex()) {
//!             vertices.join(edge.first_vertex(), edge.second_vertex());
//!             result_edges.push(edge);
//!         }
//!     }
//!     
//!     Graph::new(result_edges)
//! }
//! ```
//!
//! We can even use the fact that [`join`] returns `true` if the elements have not been joined already, to further simplify the algorithm (this variation is sometimes called Quick-Union):
//!   
//! ```
//! # trait Edge : Copy {
//! #     fn first_vertex(&self) -> usize;
//! #     fn second_vertex(&self) -> usize;
//! # }
//! #
//! # trait Graph {
//! #     type E : Edge;
//! #     fn edges_ordered_by_weight(&self) -> Vec<Self::E>;
//! #     fn number_vertices(&self) -> usize;
//! #     fn new(edges: Vec<Self::E>) -> Self;
//! # }
//! #
//! use disjoint::DisjointSet;
//!
//! fn minimum_spanning_forest_quick_find<G : Graph>(graph: &G) -> G {
//!     let mut result_edges = Vec::new();
//!     let mut vertices = DisjointSet::with_len(graph.number_vertices());
//!
//!     for edge in graph.edges_ordered_by_weight() {
//!         if vertices.join(edge.first_vertex(), edge.second_vertex()) {
//!             result_edges.push(edge);
//!         }
//!     }
//!     
//!     Graph::new(result_edges)
//! }
//! ```
//!
//! [disjoint-set data structure]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
//! [undirected edge-weighted graph]: https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Weighted_graph
//! [minimal spanning forest]: https://en.wikipedia.org/wiki/Minimum_spanning_tree
//! [Kruskal’s algorithm]: https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
//! [`join`]: DisjointSet::join
//! [`is_joined`]: DisjointSet::is_joined
//!
//! ## Changelog
//!
//! This crate maintains a [changelog].
//!
//! [changelog]: https://github.com/jogru0/disjoint/blob/master/CHANGELOG.md
//!
//! ## License
//!
//! Licensed under either of:
//!
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
//! additional terms or conditions.

mod disjoint_set;
pub use self::disjoint_set::DisjointSet;

mod disjoint_set_vec;
pub use self::disjoint_set_vec::DisjointSetVec;

mod macros;
