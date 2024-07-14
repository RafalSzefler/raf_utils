#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod common;
mod validation;
pub mod models;
pub mod serializer;
pub mod deserializer;

mod newick_graph_builder;
pub use newick_graph_builder::*;
