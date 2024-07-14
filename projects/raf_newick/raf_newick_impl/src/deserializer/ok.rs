use crate::models::NewickGraph;

pub struct DeserializeOk {
    pub graph: NewickGraph,
    pub read_bytes: usize,
}
