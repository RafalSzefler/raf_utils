#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]
use smallvec::SmallVec;

use super::{
    validation::{validate, TemporaryGraph},
    NewickGraph,
    NewickName,
    NewickNode,
    NewickNodeId,
    OptionalNewickReticulation,
    OptionalNewickWeight};


#[derive(Default)]
pub struct NewickGraphBuilder {
    nodes: Vec<NewickNode>,
    children: Vec<SmallVec<[NewickNodeId; 2]>>,
}

#[derive(Debug)]
pub enum InvalidGraphError {
    EmptyGraph,
    InconsistentNodeIds,
    MultipleRoots,
    Cyclic,    
}

impl NewickGraphBuilder {
    pub fn add_node(&mut self,
        name: NewickName,
        weight: OptionalNewickWeight,
        reticulation: OptionalNewickReticulation,
        children: &[NewickNodeId]) -> NewickNodeId
    {
        let id = self.nodes.len();
        let nid = unsafe { NewickNodeId::new_unchecked(id as i32) };
        let node = unsafe {
            NewickNode::new_unchecked(nid, name, weight, reticulation)
        };
        self.nodes.push(node);
        self.children.push(SmallVec::from(children));
        nid
    }

    /// Builds new [`NewickGraph`]
    /// 
    /// # Errors
    /// * [`InvalidGraphError::EmptyGraph`] if did not add any nodes
    /// * [`InvalidGraphError::InconsistentNodeIds`] if ids are inconsistent, e.g.
    /// when passed `children` with id not pointing to any node.
    /// * [`InvalidGraphError::MultipleRoots`] if graph has more than 1 root
    /// (i.e. node without predecessors)
    /// * [`InvalidGraphError::Cyclic`] if graph contains cycles
    pub fn build(self) -> Result<NewickGraph, InvalidGraphError> {
        let predecessors = self.build_predecessors();
        let tmp_graph = TemporaryGraph {
            nodes: &self.nodes,
            successors: &self.children,
            predecessors: &predecessors,
        };

        validate(&tmp_graph)?;

        let root = get_root(&predecessors);
        let result = unsafe {
            NewickGraph::new_unchecked(self.nodes, self.children, root)
        };

        Ok(result)
    }

    fn build_predecessors(&self) -> Vec<SmallVec<[NewickNodeId; 2]>> {
        let mut result = Vec::<SmallVec<[NewickNodeId; 2]>>::new();
        result.resize_with(self.children.len(), SmallVec::default);

        for (idx, successors) in self.children.iter().enumerate() {
            let source_id = unsafe { NewickNodeId::new_unchecked(idx as i32) };
            for successor_id in successors {
                let target_id = successor_id.value() as usize;
                result[target_id].push(source_id);
            }
        }

        result
    }
}

fn get_root(predecessors: &[SmallVec<[NewickNodeId; 2]>]) -> NewickNodeId {
    let mut optional_root = None;
    for (idx, preds) in predecessors.iter().enumerate() {
        if preds.is_empty() {
            let tmp_root = unsafe { NewickNodeId::new_unchecked(idx as i32) };
            optional_root = Some(tmp_root);
            break;
        }
    }
    optional_root.unwrap()
}
