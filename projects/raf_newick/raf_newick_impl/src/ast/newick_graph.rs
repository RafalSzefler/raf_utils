#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss)]

use smallvec::SmallVec;

use super::{NewickNode, NewickNodeId};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NewickGraph {
    nodes: Vec<NewickNode>,
    children: Vec<SmallVec<[NewickNodeId; 2]>>,
    root: NewickNodeId,
}


impl NewickGraph {
    /// Builds new instance of [`NewickGraph`].
    /// 
    /// # Safety
    /// The following invariant have to be satisfied:
    /// * `root` is within `nodes`
    /// * each node's id corresponds to position in `nodes` vector
    /// * each node's children are encoded at node id's position in
    /// `children` vector
    /// * all ids in `children` vectors are valid
    /// * graph is connected, acyclic with single root (i.e. single node
    /// without predecessors corresponding to `root` id)
    #[inline(always)]
    pub unsafe fn new_unchecked(
        nodes: Vec<NewickNode>,
        children: Vec<SmallVec<[NewickNodeId; 2]>>,
        root: NewickNodeId,
    ) -> Self {
        Self { nodes, children, root }
    }

    #[inline(always)]
    pub fn nodes(&self) -> &[NewickNode] {
        &self.nodes
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_node_by_id(&self, id: NewickNodeId) -> Option<&NewickNode> {
        let id_value = id.value();
        if id_value < 0 {
            return None;
        }
        let idx = id_value as usize;
        if idx >= self.nodes.len() {
            return None;
        }

        let node = &self.nodes[idx];
        assert!(node.id().value() == id_value, "Inconsistent IDs");
        Some(node)
    }

    #[inline(always)]
    pub fn get_children(&self, id: NewickNodeId) -> &[NewickNodeId] {
        let idx = id.value() as usize;
        &self.children[idx]
    }

    #[inline(always)]
    pub fn root(&self) -> NewickNodeId { self.root }
}
