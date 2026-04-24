use crate::db::{GraphId, RegionId};

pub struct CopyGraphParams {
    pub source_graph_id: GraphId,
    pub destination_region_id: RegionId,
    pub copy_name: String,
}
