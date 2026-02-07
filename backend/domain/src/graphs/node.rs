use serde::{Deserialize, Serialize};

use crate::db::{GraphId, NodeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub graph_id: GraphId,
}
