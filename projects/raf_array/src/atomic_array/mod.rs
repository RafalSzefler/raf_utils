//! Holds atomic/shared variants of arrays.
//! 
//! # Examples
//! 
//! ```rust
//! use raf_array::atomic_array::StrongArrayBuilder;
//! 
//! let mut counter = 0;
//! let factory = || {
//!     counter += 1;
//!     counter
//! };
//! let arr = StrongArrayBuilder::default().build_from_factory(4, factory).unwrap();
//! assert_eq!(arr.as_slice(), &[1, 2, 3, 4]);
//! let clone = arr.clone();  // This is very fast, doesn't copy the array.
//! assert_eq!(clone.as_slice(), &[1, 2, 3, 4]);
//! ```
//! 
//! will generate an array of length 4 filled with values generated from factory.
//! 
//! We can even verify lifetimes:
//! 
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use raf_array::atomic_array::StrongArrayBuilder;
//! 
//! struct KeepAlive {
//!     pub died: Arc<Mutex<i32>>,
//! }
//! 
//! impl Drop for KeepAlive {
//!     fn drop(&mut self) {
//!         let mut guard = self.died.lock().unwrap();
//!         *guard = *guard + 1;
//!     }
//! }
//! 
//! let counter = Arc::new(Mutex::new(0));
//! let counter_clone = counter.clone();
//! let factory = || {
//!     KeepAlive {
//!         died: counter_clone.clone(),
//!     }
//! };
//! let get = || {
//!     let guard = counter.lock().unwrap();
//!     *guard
//! };
//! let arr = StrongArrayBuilder::default().build_from_factory(5, factory).unwrap();
//! assert_eq!(get(), 0);
//! let clone = arr.clone();
//! assert_eq!(get(), 0);
//! drop(arr);
//! assert_eq!(get(), 0);
//! drop(clone);
//! assert_eq!(get(), 5);
//! ```
//! 
//! # Notes
//! * [`StrongArray`] is immutable. It cannot change its own size nor values.
//!   You can still use internal mutability through cells though.
//! * You can create weak references through [`StrongArray::downgrade()`] in
//!   case you want to avoid circular references or do some other things that
//!   weak references are suitable for.
//! * [`StrongArrayBuilder`] exists so that you can set additional data on it.
//!   This will get moved around with the instance of [`StrongArray`].
mod macros;
mod errors;
mod layout_holder;
mod array_id;
mod internal_array;
mod internal_array_impls;
mod strong_array;
mod strong_array_default;
mod final_strong_array;
mod weak_array;
mod final_weak_array;
mod strong_array_builder;

pub use errors::*;
pub use array_id::*;
pub use strong_array::*;
pub use weak_array::*;
pub use final_strong_array::*;
pub use final_weak_array::*;
pub use strong_array_builder::*;
