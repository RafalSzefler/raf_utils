use crate::models::NewickGraph;

pub struct ParseOk {
    pub graph: NewickGraph,
    pub read_bytes: usize,
}
