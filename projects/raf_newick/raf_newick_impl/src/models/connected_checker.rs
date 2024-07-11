use std::collections::{HashMap, HashSet};

use super::NewickNode;

pub(crate) fn is_connected(
    number_of_nodes: i32,
    neighbours: &HashMap<NewickNode, HashSet<NewickNode>>) -> bool
{
    if number_of_nodes == 0 || number_of_nodes == 1 {
        return true;
    }

    let mut nodes: HashSet<NewickNode> = (0..number_of_nodes)
        .map(|id| unsafe { NewickNode::new_unchecked(id) })
        .collect();

    let root = unsafe { NewickNode::new_unchecked(0) };
    scan_node(root, neighbours, &mut nodes);
    
    nodes.is_empty()
}

fn scan_node(
    node: NewickNode,
    neighbours: &HashMap<NewickNode, HashSet<NewickNode>>,
    all_nodes: &mut HashSet<NewickNode>)
{
    if !all_nodes.remove(&node) {
        return;
    }

    let Some(neighs) = neighbours.get(&node) else { return; };

    for neigh in neighs {
        scan_node(*neigh, neighbours, all_nodes);
    }
}
