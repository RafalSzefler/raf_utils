#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap)]

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    hash::{Hash, Hasher}};

use crate::models::NewickNode;

use super::{
    acyclic_checker,
    connected_checker,
    NewickArrow,
    NewickName,
    NewickReticulationType};

use raf_array::Array;
use smallvec::SmallVec;

#[derive(Debug, Eq, Clone)]
pub struct NewickGraph {
    number_of_nodes: i32,
    hash: u64,
    incoming_arrows: Array<SmallVec<[NewickArrow; 2]>>,
    outgoing_arrows: Array<SmallVec<[NewickArrow; 2]>>,
    node_names: HashMap<NewickNode, NewickName>,
    reticulation_types: HashMap<NewickNode, NewickReticulationType>,
    root: NewickNode,
}

impl Hash for NewickGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for NewickGraph {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && self.number_of_nodes == other.number_of_nodes
            && self.outgoing_arrows == other.outgoing_arrows
            && self.node_names == other.node_names
            && self.reticulation_types == other.reticulation_types
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NewickGraphNewError {
    IsEmpty,
    NegativeNumberOfNodes,
    MaxNumberOfNodesExceeded,
    MultipleArrowsBetweenNodes,
    ArrowPointsToNodeOutsideOfRange,
    GraphIsNotConnected,
    GraphIsNotAcyclic,
    NodeNamesContainNodesOutOfRange,
    ReticulationMapContainsNonReticulations,
}

impl NewickGraph {
    /// Creates and validates new [`NewickGraph`] out of raw components.
    /// 
    /// # Errors
    /// * [`NewickGraphNewError::IsEmpty`] when `number_of_nodes == 0`
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
    /// * [`NewickGraphNewError::ReticulationMapContainsNonReticulations`] if `reticulation_types`
    /// contains nodes which are not reticulations (i.e. of in-degree at least 2).
    pub fn new(
        number_of_nodes: i32,
        arrows: &[NewickArrow],
        node_names: HashMap<NewickNode, NewickName>,
        reticulation_types: HashMap<NewickNode, NewickReticulationType>,
    ) -> Result<Self, NewickGraphNewError> {
        if number_of_nodes < 0 {
            return Err(NewickGraphNewError::NegativeNumberOfNodes);
        }

        if number_of_nodes == 0 {
            return Err(NewickGraphNewError::IsEmpty);
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

        let nodes_count = number_of_nodes as usize;
        let mut incoming_arrows = Array::<SmallVec<[NewickArrow; 2]>>::new(nodes_count);
        let mut outgoing_arrows = Array::<SmallVec<[NewickArrow; 2]>>::new(nodes_count);

        for arrow in arrows {
            let src = arrow.source().id() as usize;
            outgoing_arrows.as_slice_mut()[src].push(*arrow);

            let trg = arrow.target().id() as usize;
            incoming_arrows.as_slice_mut()[trg].push(*arrow);
        }

        for key in reticulation_types.keys() {
            let id = key.id();
            if id < 0 || id >= number_of_nodes {
                return Err(NewickGraphNewError::ReticulationMapContainsNonReticulations);
            }

            if incoming_arrows.as_slice()[id as usize].len() < 2 {
                return Err(NewickGraphNewError::ReticulationMapContainsNonReticulations);
            }
        }

        let graph = unsafe {
            Self::new_unchecked(
                number_of_nodes,
                incoming_arrows,
                outgoing_arrows,
                node_names,
                reticulation_types)
        };
        Ok(graph)
    }

    /// Creates new [`NewickGraph`] out of raw components.
    /// 
    /// # Safety
    /// The caller has to ensure that the following invariants are satisfied:
    /// * `number_of_nodes` is positive and doesn't exceed [`NewickNode::max_id_value()`]
    /// * `incoming_arrows` and `outgoing_arrows` have to be within `0..number_of_nodes` range
    /// * the graph is connected and acyclic
    /// * `node_names` has to contain nodes present in graph
    /// * `reticulation_types` has reticulation nodes as keys
    #[inline(always)]
    pub unsafe fn new_unchecked(
            number_of_nodes: i32,
            incoming_arrows: Array<SmallVec<[NewickArrow; 2]>>,
            outgoing_arrows: Array<SmallVec<[NewickArrow; 2]>>,
            node_names: HashMap<NewickNode, NewickName>,
            reticulation_types: HashMap<NewickNode, NewickReticulationType>,
    ) -> Self {

        // Root has to exist in an acyclic graph, and has to be unique if
        // additionally connected.
        let mut root = NewickNode::new_unchecked(-1);
        for (idx, arrows) in incoming_arrows.as_slice().iter().enumerate() {
            if arrows.is_empty() {
                root = NewickNode::new_unchecked(idx as i32);
                break;
            }
        }

        let mut state = raf_fnv1a_hasher::FNV1a32Hasher::new();
        number_of_nodes.hash(&mut state);
        outgoing_arrows.hash(&mut state);
        calc_map_hash(&node_names).hash(&mut state);
        calc_map_hash(&reticulation_types).hash(&mut state);

        Self {
            number_of_nodes: number_of_nodes,
            hash: state.finish(),
            incoming_arrows: incoming_arrows,
            outgoing_arrows: outgoing_arrows,
            node_names: node_names,
            reticulation_types: reticulation_types,
            root: root,
        }
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

    pub fn get_incoming_arrows(&self, node: NewickNode) -> &[NewickArrow] {
        let idx = node.id();
        if idx < 0 || idx >= self.number_of_nodes {
            return EMPTY;
        }
        &self.incoming_arrows.as_slice()[idx as usize]
    }

    pub fn get_outgoing_arrows(&self, node: NewickNode) -> &[NewickArrow] {
        let idx = node.id();
        if idx < 0 || idx >= self.number_of_nodes {
            return EMPTY;
        }
        &self.outgoing_arrows.as_slice()[idx as usize]
    }

    #[inline(always)]
    pub fn root_node(&self) -> NewickNode { self.root }

    #[inline(always)]
    pub fn get_node_name(&self, node: NewickNode)
        -> Option<&NewickName>
    {
        self.node_names.get(&node)
    }

    #[inline(always)]
    pub fn get_reticulation_type(&self, node: NewickNode)
        -> Option<&NewickReticulationType>
    {
        self.reticulation_types.get(&node)
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

static EMPTY: &[NewickArrow] = &[];

fn calc_map_hash<T, K>(map: &HashMap<T, K>) -> u64
    where T: Hash,
          K: Hash
{
    let mut total = 0u64;
    for pair in map {
        let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
        pair.hash(&mut hasher);
        total ^= hasher.finish();
    }
    total
}
