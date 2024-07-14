#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

use std::collections::{hash_map::Entry, HashMap};

use raf_newick_impl::{
    models::{
        NewickGraph,
        NewickName,
        NewickNodeId,
        NewickReticulation,
        NewickReticulationKind,
        OptionalNewickReticulation,
        OptionalNewickWeight},
    NewickGraphBuilder};

pub fn convert_to_graph(arrows: &[(u32, u32)], names: &[(u32, &str)]) -> NewickGraph {
    let mut successors = HashMap::<u32, Vec<u32>>::new();
    let mut predecessors = HashMap::<u32, Vec<u32>>::new();
    let mut max_node_id = 0;
    for arrow in arrows {
        match successors.entry(arrow.0) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(arrow.1);
            },
            Entry::Vacant(entry) => {
                entry.insert(Vec::from([arrow.1]));
            },
        }

        match predecessors.entry(arrow.1) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(arrow.0);
            },
            Entry::Vacant(entry) => {
                entry.insert(Vec::from([arrow.0]));
            },
        }

        max_node_id = core::cmp::max(
            max_node_id,
            core::cmp::max(arrow.0, arrow.1)
        );
    }

    let mut builder = NewickGraphBuilder::default();
    let names_map: HashMap<u32, &str> = names.iter()
        .map(|p| *p)
        .collect();

    let mut seen = HashMap::new();
    for id in 0..=max_node_id {
        if !predecessors.contains_key(&id) {
            update_builder(
                &mut builder,
                id,
                &successors,
                &predecessors,
                &mut seen,
                &names_map);
        }
    }

    builder.build().unwrap()
}

fn update_builder(
        builder: &mut NewickGraphBuilder,
        node: u32,
        successors: &HashMap<u32, Vec<u32>>,
        predecessors: &HashMap<u32, Vec<u32>>,
        seen: &mut HashMap<u32, NewickNodeId>,
        names: &HashMap<u32, &str>) -> NewickNodeId
{
    if let Some(id) = seen.get(&node) {
        return *id;
    }

    let mut children = Vec::new();
    if let Some(succs) = successors.get(&node) {
        for succ in succs {
            children.push(update_builder(builder, *succ, successors, predecessors, seen, names));
        }
    }

    let ret = if let Some(preds) = predecessors.get(&node) {
        if preds.len() > 1 {
            let kind = NewickReticulationKind::default();
            let ret = unsafe { NewickReticulation::new_unchecked(node+1, kind) };
            OptionalNewickReticulation::some(ret)
        } else {
            OptionalNewickReticulation::none()
        }
    } else {
        OptionalNewickReticulation::none()
    };

    let name = if let Some(text) = names.get(&node) {
        NewickName::new(text).unwrap()
    } else {
        NewickName::default()
    };

    let node_id = builder.add_node(
        name,
        OptionalNewickWeight::none(),
        ret,
        &children);
    seen.insert(node, node_id);
    node_id
}
