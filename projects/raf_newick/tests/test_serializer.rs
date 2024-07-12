use std::collections::HashMap;

use raf_newick::{models::{
    NewickArrow, NewickGraph, NewickGraphNewError, NewickName, NewickNode, OptionalNewickWeight}, serializer::serialize_to_string};

use rstest::rstest;

fn arr(src: i32, trg: i32) -> NewickArrow {
    NewickArrow::new(
        NewickNode::new(src).unwrap(),
        NewickNode::new(trg).unwrap(),
        OptionalNewickWeight::none())
}

fn build_graph(number_of_nodes: i32, raw_arrows: &[(i32, i32)])
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

fn build_graph_with_names(number_of_nodes: i32, raw_arrows: &[(i32, i32)], names: &[(i32, &str)])
    -> Result<NewickGraph, NewickGraphNewError>
{
    let arrows: Vec<NewickArrow> = raw_arrows
        .into_iter()
        .map(|p| arr(p.0, p.1))
        .collect();

    let node_names = names
        .into_iter()
        .map(|p| {
            (
                unsafe { NewickNode::new_unchecked(p.0) },
                NewickName::new(p.1)
            )
        })
        .collect::<HashMap<NewickNode, NewickName>>();

    NewickGraph::new(
        number_of_nodes,
        &arrows,
        node_names,
        HashMap::new())
}


#[rstest]
#[case(1, &[], ";")]
#[case(2, &[(0, 1)], "();")]
#[case(3, &[(0, 1), (1, 2)], "(());")]
#[case(3, &[(0, 1), (0, 2)], "(,);")]
#[case(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)], "((,),(,));")]
fn test_serialization(
        #[case] number_of_nodes: i32,
        #[case] raw_arrows: &[(i32, i32)],
        #[case] expected: &str)
{
    let graph = build_graph(number_of_nodes, raw_arrows).unwrap();
    assert_eq!(graph.number_of_nodes(), number_of_nodes);
    let serialized = serialize_to_string(&graph).unwrap();   
    assert_eq!(serialized, expected);
}

#[rstest]
#[case(1, &[], &[(0, "A")], "A;")]
#[case(2, &[(0, 1)], &[(0, "A"), (1, "B")], "(B)A;")]
#[case(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)], &[(5, "XYZ"), (0, "F")], "((,),(XYZ,))F;")]
fn test_serialization_with_names(
        #[case] number_of_nodes: i32,
        #[case] raw_arrows: &[(i32, i32)],
        #[case] names: &[(i32, &str)],
        #[case] expected: &str)
{
    let graph = build_graph_with_names(number_of_nodes, raw_arrows, names).unwrap();
    assert_eq!(graph.number_of_nodes(), number_of_nodes);
    let serialized = serialize_to_string(&graph).unwrap();   
    assert_eq!(serialized, expected);
}

#[rstest]
#[case(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5), (4, 6)], &[(4, "R")], "((,()R#1),(R#1,));")]
#[case(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5), (4, 6), (5, 6)], &[(4, "R")], "((,(#2)R#1),(R#1,(#2)));")]
#[case(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5), (4, 6), (5, 6)], &[(4, "R"), (6, " fez ")], "((,(\" fez \"#2)R#1),(R#1,(\" fez \"#2)));")]
fn test_reticulation_serialization_with_names(
        #[case] number_of_nodes: i32,
        #[case] raw_arrows: &[(i32, i32)],
        #[case] names: &[(i32, &str)],
        #[case] expected: &str)
{
    let graph = build_graph_with_names(number_of_nodes, raw_arrows, names).unwrap();
    assert_eq!(graph.number_of_nodes(), number_of_nodes);
    let serialized = serialize_to_string(&graph).unwrap();   
    assert_eq!(serialized, expected);
}
