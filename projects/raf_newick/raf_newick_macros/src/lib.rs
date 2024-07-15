#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod converter;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
use raf_newick_impl::deserializer::deserialize_from_str;

#[proc_macro]
pub fn newick_graph(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let text = input.value();
    let graph = deserialize_from_str(&text)
        .unwrap()
        .graph;
    converter::convert(&graph).into()
}
