//! A library for serializing and deserializing Newick models.
//! 
//! # Examples
//! 
//! ```rust
//! use raf_newick::newick_graph;
//! 
//! let graph = newick_graph!("(,(,));");
//! assert_eq!(graph.nodes().len(), 5);
//! let root_id = graph.root_id();
//! let root_children = graph.get_children(root_id);
//! assert_eq!(root_children.len(), 2);
//! assert!(graph.get_children(root_children[0]).is_empty());
//! let right_children = graph.get_children(root_children[1]);
//! for right_child in right_children {
//!     assert!(graph.get_children(*right_child).is_empty());
//! }
//! ```
//! 
//! generates a graph of the form
//! 
//! ```plain
//! Root --> L
//!      --> R1 --> A
//!             --> B
//! ```
//! 
//! The same can be achieved at runtime instead of macro:
//! 
//! ```rust
//! use raf_newick::deserializer::deserialize_from_str;
//! 
//! let graph = deserialize_from_str("(,(,));").unwrap().graph;
//! assert_eq!(graph.nodes().len(), 5);
//! let root_id = graph.root_id();
//! let root_children = graph.get_children(root_id);
//! assert_eq!(root_children.len(), 2);
//! assert!(graph.get_children(root_children[0]).is_empty());
//! let right_children = graph.get_children(root_children[1]);
//! for right_child in right_children {
//!     assert!(graph.get_children(*right_child).is_empty());
//! }
//! ```
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

#[doc(hidden)]
extern crate raf_newick_impl;

#[doc(hidden)]
extern crate raf_newick_macros;

pub use raf_newick_impl::*;
pub use raf_newick_macros::*;
