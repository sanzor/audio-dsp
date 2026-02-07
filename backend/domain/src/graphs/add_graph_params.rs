use crate::graphs::graph::Graph;
use crate::db::RegionId;

pub struct AddGraphParams {
    pub region_id: RegionId,
    pub name: String,
}

pub struct AddGraphResult {
    pub graph: Graph,
}
