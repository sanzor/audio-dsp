use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::db_region::RegionId;

pub type GraphId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbGraph {
    pub graph_id: GraphId,
    pub region_id: Option<RegionId>,
    pub name: String,
    #[sqlx(json)]
    pub graph_state: Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
