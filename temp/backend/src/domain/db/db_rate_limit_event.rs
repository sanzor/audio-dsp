use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct DbRateLimitEvent {
    pub id: i64,
    pub org_id: i64,
    pub event_type: String,
    pub tokens: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
