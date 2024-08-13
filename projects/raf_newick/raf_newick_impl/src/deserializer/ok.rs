use crate::ast::NewickGraph;

pub struct DeserializeOk {
    pub graph: NewickGraph,
    pub read_bytes: usize,
}
