use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::db_graph::GraphId;

pub type EdgeId = String;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbEdge {
    pub edge_id: EdgeId,
    pub graph_id: GraphId,
    pub created_at: DateTime<Utc>,
}

