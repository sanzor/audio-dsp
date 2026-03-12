use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::project_role::ProjectRole;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbMembership {
    pub project_id: String,
    pub user_id: String,
    pub role: ProjectRole,
    pub joined_at: DateTime<Utc>,
}
