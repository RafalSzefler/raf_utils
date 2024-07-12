use std::io::Read;

use crate::models::NewickGraph;

use super::{ParseError, ParseOk};

/// # Errors
/// * [`ParseError::InvalidContent`] if input does not contain valid Newick graph
/// * [`ParseError::GraphError`] if input is a graph, but doesn't satisfy Newick
/// invariants (see [`NewickGraphNewError`] for details)
/// * [`ParseError::IO`] are errors forwarded from passed [`Read`] instance.
pub fn parse<TRead: Read>(input: &mut TRead) -> Result<ParseOk, ParseError> {
    todo!()
}

/// # Errors
/// * [`ParseError::InvalidContent`] if input does not contain valid Newick graph
/// * [`ParseError::GraphError`] if input is a graph, but doesn't satisfy Newick
/// invariants (see [`NewickGraphNewError`] for details)
/// * [`ParseError::IO`] are errors forwarded from passed [`Read`] instance.
#[inline(always)]
pub fn parse_str(input: &str) -> Result<NewickGraph, ParseError> {
    let result = parse(&mut input.as_bytes())?;
    Ok(result.graph)
}
