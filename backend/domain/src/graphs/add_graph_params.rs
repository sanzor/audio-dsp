use crate::db::RegionId;
use crate::graphs::graph::Graph;

pub struct AddGraphParams {
    pub region_id: RegionId,
    pub name: String,
}

pub struct AddGraphResult {
    pub graph: Graph,
}
