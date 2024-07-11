use std::collections::{HashMap, HashSet};

use raf_newick::models::{
    NewickArrow,
    NewickGraph,
    NewickGraphNewError,
    NewickNode,
    NewickWeight};

use rstest::rstest;

fn arr(src: i32, trg: i32) -> NewickArrow {
    NewickArrow::new(
        NewickNode::new(src).unwrap(),
        NewickNode::new(trg).unwrap(),
        NewickWeight::new(0, 0))
}

fn graph(number_of_nodes: i32, raw_arrows: &[(i32, i32)])
    -> Result<NewickGraph, NewickGraphNewError>
{
    let arrows: Vec<NewickArrow> = raw_arrows
        .into_iter()
        .map(|p| arr(p.0, p.1))
        .collect();

    NewickGraph::new(
        number_of_nodes,
        &arrows,
        HashMap::new(),
        HashMap::new())
}

#[rstest]
#[case(1, &[])]
#[case(2, &[(0, 1)])]
#[case(3, &[(0, 1), (0, 2)])]
#[case(3, &[(0, 1), (1, 2)])]
#[case(4, &[(0, 1), (0, 2), (2, 3)])]
#[case(4, &[(0, 1), (0, 2), (1, 3)])]
#[case(4, &[(0, 1), (0, 2), (0, 3)])]
fn test_correct_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows).unwrap();
    assert_eq!(result.number_of_nodes(), number_of_nodes);
    let mut node_ids: HashSet<i32> = result.iter_nodes().map(|n| n.id()).collect();
    for idx in 0..number_of_nodes {
        assert!(node_ids.remove(&idx));
    }

    assert_eq!(node_ids.len(), 0);

    let arrs: Vec<(i32, i32)> = result
        .iter_arrows()
        .map(|arr| (arr.source().id(), arr.target().id()))
        .collect();

    assert_eq!(arrs, raw_arrows);
    assert_eq!(result.root_node().id(), 0);
}

#[rstest]
#[case(1, &[], 0)]
#[case(2, &[(0, 1)], 0)]
#[case(3, &[(1, 0), (1, 2)], 1)]
#[case(3, &[(0, 1), (1, 2)], 0)]
#[case(4, &[(3, 1), (3, 2), (3, 0)], 3)]
fn test_roots(
        #[case] number_of_nodes: i32,
        #[case] raw_arrows: &[(i32, i32)],
        #[case] root: i32)
{
    let result = graph(number_of_nodes, raw_arrows).unwrap();
    assert_eq!(result.number_of_nodes(), number_of_nodes);
    assert_eq!(result.root_node().id(), root);
}

#[rstest]
#[case(2, &[])]
#[case(3, &[(0, 1)])]
#[case(4, &[(0, 1), (0, 2)])]
#[case(4, &[(0, 1), (1, 2)])]
#[case(4, &[(0, 1), (1, 2)])]
fn test_disconnected_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows);
    let err = result.err().unwrap();
    assert_eq!(err, NewickGraphNewError::GraphIsNotConnected);
}

#[rstest]
#[case(1, &[(0, 0)])]
#[case(2, &[(0, 1), (1, 0)])]
#[case(3, &[(0, 1), (1, 2), (2, 0)])]
#[case(4, &[(0, 1), (0, 2), (2, 3), (3, 3)])]
#[case(5, &[(0, 1), (0, 2), (2, 3), (2, 4), (4, 0)])]
fn test_cyclic_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows);
    let err = result.err().unwrap();
    assert_eq!(err, NewickGraphNewError::GraphIsNotAcyclic);
}

#[rstest]
#[case(2, &[(0, 1), (0, 1)])]
#[case(2, &[(0, 1), (0, 1), (0, 1)])]
#[case(4, &[(0, 1), (0, 2), (2, 3), (0, 1)])]
fn test_multiple_arrows_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows);
    let err = result.err().unwrap();
    assert_eq!(err, NewickGraphNewError::MultipleArrowsBetweenNodes);
}

#[rstest]
#[case(1, &[(0, 1)])]
#[case(2, &[(0, 1), (15, 1)])]
fn test_arrows_out_of_range_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows);
    let err = result.err().unwrap();
    assert_eq!(err, NewickGraphNewError::ArrowPointsToNodeOutsideOfRange);
}

#[rstest]
#[case(0, &[])]
#[case(0, &[(0, 0)])]
#[case(0, &[(0, 1)])]
fn test_empty_graph(#[case] number_of_nodes: i32, #[case] raw_arrows: &[(i32, i32)]) {
    let result = graph(number_of_nodes, raw_arrows);
    let err = result.err().unwrap();
    assert_eq!(err, NewickGraphNewError::IsEmpty);
}
