//! Models for Newick serialization.
mod ok;
mod error;
mod models;

use std::io::Write;

pub use ok::*;
pub use error::*;

use crate::ast::NewickGraph;

/// Serializes instance of [`NewickGraph`] into [`Write`].
/// 
/// # Errors
/// * [`SerializeError::InvalidInput`] if graph is inconsistent
/// * [`SerializeError::OutputError`] if couldn't write to underlying stream
#[inline(always)]
pub fn serialize<TWrite: Write>(output: &mut TWrite, graph: &NewickGraph)
    -> Result<SerializeOk, SerializeError>
{
    let serializer = models::Serializer::new(output, graph);
    serializer.serialize()
}

/// Serializes instance of [`NewickGraph`] into [`String`].
/// 
/// # Errors
/// * [`SerializeError::InvalidInput`] if graph is inconsistent
/// * [`SerializeError::OutputError`] if couldn't write to [`String`]
pub fn serialize_to_string(graph: &NewickGraph) -> Result<String, SerializeError> {
    let mut output = Vec::new();
    let serializer = models::Serializer::new(&mut output, graph);
    serializer.serialize()?;
    let text = unsafe { String::from_utf8_unchecked(output) };
    Ok(text)
}
