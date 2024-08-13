//! Holds models for templates. Template is a piece of `LogDataHolder` that
//! should be filled with other parameters to generate a proper log string.
#![allow(clippy::missing_panics_doc)]

mod models;
mod parser;

pub use models::*;
