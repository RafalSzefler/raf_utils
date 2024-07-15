#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]

use std::collections::HashSet;

use smallvec::SmallVec;

use super::{InvalidGraphError, NewickNode, NewickNodeId};

pub(crate) struct TemporaryGraph<'a> {
    pub nodes: &'a Vec<NewickNode>,
    pub successors: &'a Vec<SmallVec<[NewickNodeId; 2]>>,
    pub predecessors: &'a Vec<SmallVec<[NewickNodeId; 2]>>,
}

pub(crate) fn validate(graph: &TemporaryGraph) -> Result<(), InvalidGraphError> {
    // It is important for those validations to run in this specific order.
    validate_basic_properties(graph)?;
    let root = validate_and_get_root(graph)?;
    let mut seen = HashSet::new();
    acyclic_scan_with_validation(root, &mut seen, graph)?;
    Ok(())
}

fn acyclic_scan_with_validation(
        node: NewickNodeId,
        seen: &mut HashSet<NewickNodeId>,
        graph: &TemporaryGraph)
    -> Result<(), InvalidGraphError>
{
    if !seen.insert(node) {
        return Err(InvalidGraphError::Cyclic);
    }
    
    let idx = node.value() as usize;
    for succ in &graph.successors[idx] {
        acyclic_scan_with_validation(*succ, seen, graph)?;
    }

    seen.remove(&node);
    Ok(())
}

fn validate_and_get_root(graph: &TemporaryGraph) -> Result<NewickNodeId, InvalidGraphError> {
    let mut root = None;
    for (idx, preds) in graph.predecessors.iter().enumerate() {
        if preds.is_empty() {
            if root.is_some() {
                return Err(InvalidGraphError::MultipleRoots);
            }
            let id = unsafe { NewickNodeId::new_unchecked(idx as i32) };
            root = Some(id);
        }
    }

    if let Some(v) = root {
        Ok(v)
    } else {
        // Graph without roots has to have cycles.
        Err(InvalidGraphError::Cyclic)
    }
}


fn validate_basic_properties(graph: &TemporaryGraph) -> Result<(), InvalidGraphError> {
    let nodes_len = graph.nodes.len();
    if nodes_len == 0 {
        return Err(InvalidGraphError::EmptyGraph);
    }

    if nodes_len != graph.successors.len() || nodes_len != graph.predecessors.len() {
        return Err(InvalidGraphError::InconsistentNodeIds);
    }

    for (idx, node) in graph.nodes.iter().enumerate() {
        let node_id = node.id();
        validate_node_id(node_id, nodes_len)?;
        if (node_id.value() as usize) != idx {
            return Err(InvalidGraphError::InconsistentNodeIds);
        }
    }

    for succs in graph.successors {
        for trg in succs {
            validate_node_id(*trg, nodes_len)?;
        }
    }

    for preds in graph.predecessors {
        for trg in preds {
            validate_node_id(*trg, nodes_len)?;
        }
    }

    Ok(())
}

fn validate_node_id(id: NewickNodeId, len: usize) -> Result<(), InvalidGraphError> {
    let value = id.value();
    if value < 0 || (value as usize) >= len {
        return Err(InvalidGraphError::InconsistentNodeIds);
    }
    Ok(())
}
