//! Implements tagged pointers, i.e. pointers with additional bits set.
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

#![no_std]

mod macros;
mod bit;
mod tagged_pointer;

pub use bit::{BitNewError, Bit};
pub use tagged_pointer::{TaggedPointerNewError, TaggedPointer};
