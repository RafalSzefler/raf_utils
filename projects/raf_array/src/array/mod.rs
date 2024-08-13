//! Holds an array struct with its implementation.
//! 
//! # Examples
//! 
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use raf_array::array::Array;
//! 
//! let index = &mut 1;
//! let factory = || {
//!     *index = *index * 2;
//!     *index
//! };
//! let arr = Array::from_factory(5, factory).unwrap();
//! assert_eq!(arr.as_slice(), &[2, 4, 8, 16, 32]);
//! ```
//! 
//! will generate an array of length 5 filled with values generated from factory.
//! 
//! If `T` implements `Default`, then it can be used instead:
//! ```rust
//! use raf_array::array::Array;
//! 
//! let mut arr = Array::<bool>::new_default(4).unwrap();
//! assert_eq!(arr.as_slice(), &[false, false, false, false]);
//! let mut_slice = arr.as_slice_mut();
//! mut_slice[2] = true;
//! mut_slice[3] = true;
//! assert_eq!(arr.as_slice(), &[false, false, true, true]);
//! ```
//! 
//! # Notes
//! * While `Array` is mutable, it cannot change its own size. It is slightly
//!   more efficient than `Vec` because it doesn't keep capacity around.
mod models;
mod impls;

pub use models::*;
