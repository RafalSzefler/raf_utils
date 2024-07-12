use std::io::Write;

use crate::models::NewickGraph;

use super::{internal::InternalSerializer, SerializeError, SerializerOk};

/// Serializes [`NewickGraph`] into passed [`Write`] instance.
/// 
/// # Errors
/// * [`SerializeError::IO`] are errors forwarded from passed
/// [`Write`] instance.
#[inline(always)]
pub fn serialize<TWrite: Write>(
        graph: &NewickGraph,
        output: &mut TWrite)
    -> Result<SerializerOk, SerializeError>
{
    let internal_serializer = InternalSerializer::new(graph, output);
    internal_serializer.serialize()
}

/// Serializes [`NewickGraph`] into [`String`] instance.
/// 
/// # Errors
/// * [`SerializeError::IO`] are errors forwarded from the intermediate
/// [`Write`] instance. The should never occure under normal circumstances.
#[inline(always)]
pub fn serialize_to_string(graph: &NewickGraph)
    -> Result<String, SerializeError>
{
    let mut buffer = Vec::with_capacity(4);
    serialize(graph, &mut buffer)?;
    let str = unsafe { String::from_utf8_unchecked(buffer) };
    Ok(str)
}
