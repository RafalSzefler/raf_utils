//! Models for Newick deserialization.
mod ok;
mod error;
mod models;

use std::io::Read;

pub use ok::*;
pub use error::*;


/// Deserializes instance of [`NewickGraph`][crate::ast::NewickGraph]
/// from [`Read`].
/// 
/// # Errors
/// * [`DeserializeError::FormatError`] if not a valid Newick format
/// * [`DeserializeError::GraphError`] if error on graph building
/// * [`DeserializeError::InputError`] if invalid input
/// * [`DeserializeError::Utf8`] if input is not a valid UTF-8 string
#[inline(always)]
pub fn deserialize<TRead: Read>(input: &mut TRead)
    -> Result<DeserializeOk, DeserializeError>
{
    let deserializer = models::Deserializer::new(input);
    deserializer.deserialize()
}

/// Deserializes instance of [`NewickGraph`][crate::ast::NewickGraph] from
/// [`&str`].
/// 
/// # Errors
/// * [`DeserializeError::FormatError`] if not a valid Newick format
/// * [`DeserializeError::GraphError`] if error on graph building
/// * [`DeserializeError::InputError`] if invalid input
/// * [`DeserializeError::Utf8`] if input is not a valid UTF-8 string
#[inline(always)]
pub fn deserialize_from_str(input: &str) -> Result<DeserializeOk, DeserializeError> {
    let mut stream = input.as_bytes();
    deserialize(&mut stream)
}
