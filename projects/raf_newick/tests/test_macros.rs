use raf_newick::{
    ast::NewickGraph,
    deserializer::deserialize_from_str,
    newick_graph};


#[test]
fn test_macro() {
    let mut errors = Vec::<String>::new();
    macro_rules! test_graph {
        ( $lit: expr ) => {
            {
                let macro_graph = newick_graph!($lit);
                if let Some(err) = validate(&macro_graph, $lit).err() {
                    errors.push(err);                
                }
            }
        };
    }

    test_graph!(";");
    test_graph!("();");
    test_graph!("(,);");
    test_graph!("((,),);");
    test_graph!("((,),(,));");
    test_graph!("((,),(,,));");
    test_graph!("((A,B),(,,,C#N13));");
    test_graph!("((A,B),(,,,C#N13),(((D:0.1#15))));");

    if !errors.is_empty() {
        let message = errors.join("");
        panic!("{}", message);
    }
}

fn validate(
    graph: &NewickGraph,
    repr: &str,
) -> Result<(), String>
{
    let dynamic_graph = deserialize_from_str(repr)
        .unwrap()
        .graph;

    if graph != &dynamic_graph {
        let msg = format!("  Unequal graph for case: {}\n", repr);
        return Err(msg);
    }

    Ok(())
}
