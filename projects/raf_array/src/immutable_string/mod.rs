//! Holds atomic/shared variants of strings.
//! 
//! # Examples
//! 
//! ```rust
//! use raf_array::immutable_string::ImmutableString;
//! 
//! let text = ImmutableString::new("test").unwrap();
//! assert_eq!(text.as_str(), "test");
//! ```
//! 
//! Similarly to [`StrongArray`][crate::atomic_array::StrongArray] these can be
//! moved around and copied without paying much (only ref count bump).
mod to_temporary_string_params;
mod errors;
mod temporary_string;
mod cache;
mod string_id;
mod model;

pub use errors::*;
pub use string_id::*;
pub use model::*;
