use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct DbRateLimitConfig {
    pub org_id: i64,
    pub config: serde_json::Value,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
