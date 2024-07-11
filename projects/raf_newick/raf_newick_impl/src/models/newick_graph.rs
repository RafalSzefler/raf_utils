#![allow(clippy::cast_sign_loss)]

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    hash::{Hash, Hasher}};

use crate::models::NewickNode;

use super::{acyclic_checker, connected_checker, NewickArrow};

use raf_array::Array;
use smallvec::SmallVec;

#[derive(Debug, Eq, Clone)]
pub struct NewickGraph {
    number_of_nodes: i32,
    outgoing_arrows: Array<SmallVec<[NewickArrow; 2]>>,
    node_names: HashMap<NewickNode, String>,
}

impl Hash for NewickGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number_of_nodes.hash(state);
        self.outgoing_arrows.hash(state);

        let mut total = 0u64;
        for pair in &self.node_names {
            let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
            pair.hash(&mut hasher);
            total ^= hasher.finish();
        }
        state.write_u64(total);
    }
}

impl PartialEq for NewickGraph {
    fn eq(&self, other: &Self) -> bool {
        self.number_of_nodes == other.number_of_nodes
            && self.outgoing_arrows == other.outgoing_arrows
            && self.node_names == other.node_names
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NewickGraphNewError {
    NegativeNumberOfNodes,
    MaxNumberOfNodesExceeded,
    MultipleArrowsBetweenNodes,
    ArrowPointsToNodeOutsideOfRange,
    GraphIsNotConnected,
    GraphIsNotAcyclic,
    NodeNamesContainNodesOutOfRange,
}

impl NewickGraph {
    /// Creates and validates new [`NewickGraph`] out of raw components.
    /// 
    /// # Errors
    /// * [`NewickGraphNewError::NegativeNumberOfNodes`] when `number_of_nodes < 0`
    /// * [`NewickGraphNewError::MaxNumberOfNodesExceeded`] when `number_of_nodes`
    /// is above [`NewickNode::max_id_value()`]
    /// * [`NewickGraphNewError::MultipleArrowsBetweenNodes`] if there are two nodes
    /// with multiple arrows in between
    /// * [`NewickGraphNewError::ArrowPointsToNodeOutsideOfRange`] if there are arrows
    /// refering to nodes outside of `0..number_of_nodes` range.
    /// * [`NewickGraphNewError::GraphIsNotConnected`] if graph is disconnected
    /// * [`NewickGraphNewError::GraphIsNotAcyclic`] if graph contains oriented cycles
    /// * [`NewickGraphNewError::NodeNamesContainNodesOutOfRange`] if `node_names` contains
    /// nodes outside of `0..number_of_nodes` range
    pub fn new(
        number_of_nodes: i32,
        arrows: &[NewickArrow],
        node_names: HashMap<NewickNode, String>,
    ) -> Result<Self, NewickGraphNewError> {
        if number_of_nodes < 0 {
            return Err(NewickGraphNewError::NegativeNumberOfNodes);
        }

        if number_of_nodes > NewickNode::max_id_value() {
            return Err(NewickGraphNewError::MaxNumberOfNodesExceeded);
        }

        let mut seen_arrows = HashSet::with_capacity(arrows.len());
        let mut successors = HashMap::new();
        let mut neighours = HashMap::new();

        for arrow in arrows {
            if !seen_arrows.insert(arrow) {
                return Err(NewickGraphNewError::MultipleArrowsBetweenNodes);
            }

            let src = arrow.source();
            if src.id() >= number_of_nodes {
                return Err(NewickGraphNewError::ArrowPointsToNodeOutsideOfRange);
            }

            let trg = arrow.target();
            if trg.id() >= number_of_nodes {
                return Err(NewickGraphNewError::ArrowPointsToNodeOutsideOfRange);
            }

            get_succ(&mut successors, &src).push(trg);
            get_neigh(&mut neighours, &src).insert(trg);
            get_neigh(&mut neighours, &trg).insert(src);
        }
        
        if !connected_checker::is_connected(number_of_nodes, &neighours) {
            return Err(NewickGraphNewError::GraphIsNotConnected);
        }

        if !acyclic_checker::is_acyclic(number_of_nodes, &successors) {
            return Err(NewickGraphNewError::GraphIsNotAcyclic);
        }

        for key in node_names.keys() {
            let id = key.id();
            if id < 0 || id >= number_of_nodes {
                return Err(NewickGraphNewError::NodeNamesContainNodesOutOfRange);
            }
        }

        let graph = unsafe {
            Self::new_unchecked(number_of_nodes, arrows, node_names)
        };
        Ok(graph)
    }

    /// Creates new [`NewickGraph`] out of raw components.
    /// 
    /// # Safety
    /// The caller has to ensure that the following invariants are satisfied:
    /// * `number_of_nodes` is non-negative and doesn't exceed [`NewickNode::max_id_value()`]
    /// * arrows have to point to nodes within `0..number_of_nodes` range.
    /// * there is a single arrow between any two nodes
    /// * the graph is connected and acyclic
    /// * `node_names` has to contain nodes present in graph
    #[inline(always)]
    pub unsafe fn new_unchecked(
            number_of_nodes: i32,
            arrows: &[NewickArrow],
            node_names: HashMap<NewickNode, String>,
    ) -> Self {
        let mut outgoing_arrows = Array::<SmallVec<[NewickArrow; 2]>>::new(arrows.len());

        for arrow in arrows {
            let src = arrow.source().id() as usize;
            outgoing_arrows.as_slice_mut()[src].push(*arrow);
        }

        Self {
            number_of_nodes: number_of_nodes,
            outgoing_arrows: outgoing_arrows,
            node_names: node_names }
    }

    #[inline(always)]
    pub fn number_of_nodes(&self) -> i32 { self.number_of_nodes }

    #[inline(always)]
    pub fn iter_nodes(&self) -> impl Iterator<Item = NewickNode> {
        (0..self.number_of_nodes).map(|id| unsafe { NewickNode::new_unchecked(id) })
    }

    #[inline(always)]
    pub fn iter_arrows(&self) -> impl Iterator<Item = &NewickArrow> {
        self.outgoing_arrows
            .as_slice()
            .iter()
            .flat_map(|nested| nested.iter())
    }

    pub fn get_outgoing_arrows(&self, node: NewickNode) -> &[NewickArrow] {
        static EMPTY: &[NewickArrow] = &[];
        let idx = node.id();
        if idx < 0 || idx >= self.number_of_nodes {
            return EMPTY;
        }
        &self.outgoing_arrows.as_slice()[idx as usize]
    }

    #[inline(always)]
    pub fn get_node_name(&self, node: NewickNode) -> Option<&String> {
        self.node_names.get(&node)
    }
}

fn get_succ<'a>(map: &'a mut HashMap<NewickNode, Vec<NewickNode>>, node: &'a NewickNode)
    -> &'a mut Vec<NewickNode>
{
    match map.entry(*node) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => {
            let succs = Vec::with_capacity(2);
            v.insert(succs)
        },
    }
}

fn get_neigh<'a>(map: &'a mut HashMap<NewickNode, HashSet<NewickNode>>, node: &'a NewickNode)
    -> &'a mut HashSet<NewickNode>
{
    match map.entry(*node) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => {
            let succs = HashSet::with_capacity(2);
            v.insert(succs)
        },
    }
}
