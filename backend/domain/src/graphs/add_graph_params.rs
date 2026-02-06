use crate::graphs::graph::Graph;

pub struct AddGraphParams {
    pub region_id: String,
    pub name: String,
}

pub struct AddGraphResult {
    pub graph: Graph,
}
