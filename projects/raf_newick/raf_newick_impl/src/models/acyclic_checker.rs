use std::collections::{HashMap, HashSet};

use super::NewickNode;

pub(crate) fn is_acyclic(
    number_of_nodes: i32,
    successors: &HashMap<NewickNode, Vec<NewickNode>>) -> bool
{
    if number_of_nodes == 0 {
        return true;
    }

    if number_of_nodes == 1 {
        let node = unsafe { NewickNode::new_unchecked(0) };
        match successors.get(&node) {
            Some(succs) => {
                return succs.is_empty();
            },
            None => {
                return true;
            },
        }
    }

    let mut nodes: HashSet<NewickNode> = (0..number_of_nodes)
        .map(|id| unsafe { NewickNode::new_unchecked(id) })
        .collect();

    let mut seen = HashSet::new();
    loop {
        let next_node = match nodes.iter().next() {
            Some(node) => *node,
            None => return true,
        };

        if has_cycle(next_node, successors, &mut nodes, &mut seen) {
            return false;
        }
    }
}

fn has_cycle(
        node: NewickNode,
        successors: &HashMap<NewickNode, Vec<NewickNode>>,
        nodes: &mut HashSet<NewickNode>,
        seen: &mut HashSet<NewickNode>) -> bool
{
    if !seen.insert(node) {
        return true;
    }

    if let Some(succs) = successors.get(&node) {
        for succ in succs {
            if has_cycle(*succ, successors, nodes, seen) {
                return true;
            }
        }
    }

    seen.remove(&node);
    nodes.remove(&node);

    return false;
}
