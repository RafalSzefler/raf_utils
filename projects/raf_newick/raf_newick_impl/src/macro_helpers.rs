use smallvec::SmallVec;

use crate::ast::NewickNodeId;

#[doc(hidden)]
#[inline(always)]
pub fn broken_node_id() -> NewickNodeId {
    unsafe { NewickNodeId::new_unchecked(-1) }
}

#[doc(hidden)]
#[inline(always)]
pub fn empty_graph_child_vec() -> Vec<SmallVec<[NewickNodeId; 2]>> {
    Vec::new()
}
