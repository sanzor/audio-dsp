use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::db_graph::GraphId;

pub type NodeId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbNode {
    pub node_id: NodeId,
    pub graph_id: GraphId,
    pub created_at: DateTime<Utc>,
}

