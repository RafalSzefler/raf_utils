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
pub mod ast;
pub mod serializer;
pub mod deserializer;

#[doc(hidden)]
pub mod macro_helpers;
