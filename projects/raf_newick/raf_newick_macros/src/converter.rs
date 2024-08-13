use raf_newick_impl::ast::{NewickGraph, NewickNode, NewickNodeId};

use quote::quote;
use proc_macro2::TokenStream;

pub fn convert(graph: &NewickGraph) -> TokenStream {
    let mut nodes_stream = TokenStream::new();
    for node in graph.nodes() {
        nodes_stream.extend(convert_node(node));
    }

    let mut children_stream = TokenStream::new();
    for node in graph.nodes() {
        let children = graph.get_children(node.id());
        children_stream.extend(convert_node_children(node, children));
    }

    let root = graph.root_id().value();
    let nodes_len = graph.nodes().len();

    quote! {
        unsafe {
            use raf_newick::{
                macro_helpers::{
                    broken_node_id,
                    empty_graph_child_vec}};

            use raf_newick::{
                ast::{
                    NewickNode,
                    NewickNodeId,
                    NewickGraph,
                    NewickName,
                    NewickReticulation,
                    NewickReticulationKind,
                    NewickWeight,
                    OptionalNewickReticulation,
                    OptionalNewickWeight}};
            
            let empty_name = NewickName::default();
            let empty_kind = NewickReticulationKind::default();
            let empty_weight = OptionalNewickWeight::none();
            let empty_ret = OptionalNewickReticulation::none();

            // Setup nodes...
            let mut nodes = Vec::<NewickNode>::with_capacity(#nodes_len);
            #nodes_stream

            // Setup children map...
            let mut children = empty_graph_child_vec();
            children.resize_with(#nodes_len, Default::default);
            #children_stream

            // Put everything together...
            let root = NewickNodeId::new_unchecked(#root);
            NewickGraph::new_unchecked(nodes, children, root)
        }
    }
}


fn convert_node(node: &NewickNode) -> TokenStream {
    let id = node.id().value();

    let name = node.name().as_str();
    let name_token = if name.is_empty() {
        quote! { let name = empty_name.clone(); }
    } else {
        quote! { let name = NewickName::new_unchecked(#name); }
    };

    let weight_token = if let Some(weight) = node.weight() {
        let i = weight.integral_part();
        let f = weight.fractional_part();
        quote! { let weight = OptionalNewickWeight::some(NewickWeight::new_unchecked(#i, #f)); }
    } else {
        quote! { let weight = empty_weight.clone(); }
    };

    let ret_token = if let Some(ret) = node.reticulation() {
        let ret_id = ret.id();
        let kind = ret.kind().as_str();
        let kind_token = if kind.is_empty() {
            quote! { empty_kind.clone() }
        } else {
            quote! { NewickReticulationKind::new_unchecked(#kind) }
        };
        quote! { let reticulation = OptionalNewickReticulation::some(NewickReticulation::new_unchecked(#ret_id, #kind_token)); }
    } else {
        quote! { let reticulation = empty_ret.clone(); }
    };

    quote! {
        {
            let id = NewickNodeId::new_unchecked(#id);
            #name_token
            #weight_token
            #ret_token
            let node = NewickNode::new_unchecked(id, name, weight, reticulation);
            nodes.push(node);
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn convert_node_children(node: &NewickNode, children: &[NewickNodeId]) -> TokenStream {
    if children.is_empty() {
        return TokenStream::new();
    }

    let children_len = children.len();
    let node_id = node.id().value() as usize;
    let mut ids = TokenStream::new();
    for (idx, child) in children.iter().enumerate() {
        let id = child.value();
        ids.extend(quote! {
            c[#idx] = NewickNodeId::new_unchecked(#id);
        });
    }

    quote! {
        {
            let c = &mut children[#node_id];
            c.resize_with(#children_len, &broken_node_id);
            #ids
        }
    }
}
