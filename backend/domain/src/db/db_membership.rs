use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{db::WorkspaceId, domain_user::UserId, workspace_role::WorkspaceRole};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbMembership {
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
    pub role: WorkspaceRole,
    pub joined_at: DateTime<Utc>,
}
