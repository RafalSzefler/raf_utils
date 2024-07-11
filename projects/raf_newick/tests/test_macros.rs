use raf_newick::newick_graph;

#[test]
fn test_macro() {
    let x = newick_graph!(124);
    assert_eq!(x, 124);
}
