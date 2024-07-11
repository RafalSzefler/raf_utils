use std::io::Read;

use crate::models::{NewickGraph, NewickGraphNewError};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ParseError {
    InvalidContent,
    GraphError(NewickGraphNewError),
}

pub struct ParseOk {
    pub graph: NewickGraph,
    pub read_bytes: usize,
}

/// # Errors
/// * [`ParseError::InvalidContent`] if input does not contain valid Newick graph
/// * [`ParseError::GraphError`] if input is a graph, but doesn't satisfy Newick
/// invariants (see [`NewickGraphNewError`] for details)
pub fn parse<TRead: Read>(input: &mut TRead) -> Result<ParseOk, ParseError> {
    todo!()
}

/// # Errors
/// * [`ParseError::InvalidContent`] if input does not contain valid Newick graph
/// * [`ParseError::GraphError`] if input is a graph, but doesn't satisfy Newick
/// invariants (see [`NewickGraphNewError`] for details)
#[inline(always)]
pub fn parse_str(input: &str) -> Result<ParseOk, ParseError> {
    parse(&mut input.as_bytes())
}
