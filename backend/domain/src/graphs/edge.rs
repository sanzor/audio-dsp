use serde::{Deserialize, Serialize};

use crate::db::{EdgeId, GraphId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub graph_id: GraphId,
}
