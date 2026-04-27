use serde_json::Value;

use crate::db::db_graph::GraphId;

pub struct SaveGraphStateParams {
    pub graph_id: GraphId,
    pub state: Value,
}
