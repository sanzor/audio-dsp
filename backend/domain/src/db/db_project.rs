use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type ProjectId = String;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbProject {
    pub project_id: ProjectId,
    pub name: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}
