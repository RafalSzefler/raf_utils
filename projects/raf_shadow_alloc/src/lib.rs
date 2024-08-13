//! Library for efficient array allocation through so called shadow stack.
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod shadow_stack_size;
mod shadow_stack;

pub use shadow_stack_size::*;
pub use shadow_stack::*;
