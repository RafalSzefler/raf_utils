use raf_newick::{
    ast::{
        NewickGraphBuilder,
        NewickName,
        NewickReticulation,
        NewickReticulationKind,
        NewickWeight,
        OptionalNewickReticulation,
        OptionalNewickWeight},
    serializer::{serialize_to_string, serialize}};
use raf_newick_tests::convert_to_graph;
use rstest::rstest;


#[test]
fn test_first() {
    let mut builder = NewickGraphBuilder::default();
    let leaf1 = builder.add_node(
        NewickName::new("A").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[]);
    let leaf2 = builder.add_node(
        NewickName::new("B").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[]);
    let leaf3 = builder.add_node(
        NewickName::new("C").unwrap(),
        OptionalNewickWeight::some(NewickWeight::new(1, 7).unwrap()),
        OptionalNewickReticulation::none(),
        &[]);
    let internal = builder.add_node(
        NewickName::default(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[leaf1, leaf2]);
    let _root = builder.add_node(
        NewickName::new("ROOT").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[internal, leaf3]);
    let graph = builder.build().unwrap();
    let mut output = Vec::new();
    let result = serialize(&mut output, &graph).unwrap();
    let text = core::str::from_utf8(&output[0..result.written_bytes]).unwrap();
    assert_eq!(text, "((A,B),C:1.7)ROOT;");

    let text2 = serialize_to_string(&graph).unwrap();
    assert_eq!(text, text2);
}


#[rstest]
#[case(&[], &[], ";")]
#[case(&[(0, 1)], &[], "();")]
#[case(&[(0, 1)], &[(0, "foo")], "()foo;")]
#[case(&[(0, 1)], &[(1, "foo")], "(foo);")]
#[case(&[(0, 1), (0, 2), (1, 3), (1, 4)], &[(1, " test ")], "((,)\" test \",);")]
#[case(&[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5)], &[], "((,#5),(#5,));")]
fn test_simple(
    #[case] graph_data: &[(u32, u32)],
    #[case] names_map: &[(u32, &str)],
    #[case] expected: &str
) {
    let graph = convert_to_graph(graph_data, names_map);
    let mut output = Vec::new();
    let result = serialize(&mut output, &graph).unwrap();
    let text = core::str::from_utf8(&output[0..result.written_bytes]).unwrap();
    assert_eq!(text, expected);

    let text2 = serialize_to_string(&graph).unwrap();
    assert_eq!(text, text2);
}


#[test]
fn test_reticulation() {
    let mut builder = NewickGraphBuilder::default();
    let leaf1 = builder.add_node(
        NewickName::new("A").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[]);
    let leaf2 = builder.add_node(
        NewickName::new("B").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::some(
            NewickReticulation::new(1, NewickReticulationKind::default()).unwrap(),
        ),
        &[]);
    let leaf3 = builder.add_node(
        NewickName::new("C").unwrap(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[]);
    let internal1 = builder.add_node(
        NewickName::default(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[leaf1, leaf2]);
    let internal2 = builder.add_node(
        NewickName::default(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[leaf2, leaf3]);
    let _root = builder.add_node(
        NewickName::default(),
        OptionalNewickWeight::none(),
        OptionalNewickReticulation::none(),
        &[internal1, internal2]);
    let graph = builder.build().unwrap();
    let mut output = Vec::new();
    let result = serialize(&mut output, &graph).unwrap();
    let text = core::str::from_utf8(&output[0..result.written_bytes]).unwrap();
    assert_eq!(text, "((A,B#1),(B#1,C));");

    let text2 = serialize_to_string(&graph).unwrap();
    assert_eq!(text, text2);
}