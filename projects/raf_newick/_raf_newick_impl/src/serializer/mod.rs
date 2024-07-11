use std::io::Write;

use crate::models::NewickGraph;

pub fn serialize<TWrite: Write>(graph: &NewickGraph, output: &mut TWrite) -> usize {
    todo!()
}

#[inline(always)]
pub fn serialize_to_string(graph: &NewickGraph) -> String {
    let mut buffer = Vec::with_capacity(4);
    serialize(graph, &mut buffer);
    unsafe { String::from_utf8_unchecked(buffer) }
}
