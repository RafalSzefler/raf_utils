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
