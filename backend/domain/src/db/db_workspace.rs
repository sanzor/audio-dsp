use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain_user::UserId;

pub type WorkspaceId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbWorkspace {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
}
