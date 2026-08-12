use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{db::{WorkspaceId, db_transform::TransformId}, domain_user::UserId};
pub type GrantId=i64;
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransformGrant {
    pub grant_id: GrantId,
    pub transform_id: TransformId,
    pub grantee_user_id: Option<UserId>,
    pub grantee_workspace_id: Option<WorkspaceId>,
    pub granted_by: UserId,
    pub created_at: DateTime<Utc>,
}
