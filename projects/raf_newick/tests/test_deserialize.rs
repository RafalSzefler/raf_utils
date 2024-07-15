use raf_newick::deserializer::deserialize_from_str;
use rstest::rstest;

#[rstest]
#[case(";", 1)]
#[case("();", 2)]
#[case("(,);", 3)]
#[case("((,),);", 5)]
#[case("((,),(,));", 7)]
#[case("((,),(,,));", 8)]
fn test_no(
    #[case] graph: &str,
    #[case] expected_no: usize
) {
    let graph = deserialize_from_str(graph)
        .unwrap().graph;
    assert_eq!(graph.nodes().len(), expected_no);
}


#[test]
fn test_first() {
    let graph = deserialize_from_str("((A,B),C:1.7)ROOT;")
        .unwrap().graph;
    let nodes = graph.nodes();
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes[0].name().as_str(), "A");
    assert_eq!(nodes[1].name().as_str(), "B");
    assert_eq!(nodes[2].name().as_str(), "");
    assert_eq!(nodes[3].name().as_str(), "C");
    assert_eq!(nodes[4].name().as_str(), "ROOT");

    let weight = nodes[3].weight().unwrap();
    assert_eq!(weight.integral_part(), 1);
    assert_eq!(weight.fractional_part(), 7);

    for node in nodes {
        assert!(node.reticulation().is_none());
    }
}


#[test]
fn test_second() {
    let graph = deserialize_from_str("((A#1,B#NN2),C:3.1)ROOT;")
        .unwrap().graph;
    let nodes = graph.nodes();
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes[0].name().as_str(), "A");
    assert_eq!(nodes[1].name().as_str(), "B");
    assert_eq!(nodes[2].name().as_str(), "");
    assert_eq!(nodes[3].name().as_str(), "C");
    assert_eq!(nodes[4].name().as_str(), "ROOT");

    let weight = nodes[3].weight().unwrap();
    assert_eq!(weight.integral_part(), 3);
    assert_eq!(weight.fractional_part(), 1);

    let ret1 = nodes[0].reticulation().unwrap();
    assert_eq!(ret1.id(), 1);
    assert_eq!(ret1.kind().as_str(), "");

    let ret2 = nodes[1].reticulation().unwrap();
    assert_eq!(ret2.id(), 2);
    assert_eq!(ret2.kind().as_str(), "NN");
}
