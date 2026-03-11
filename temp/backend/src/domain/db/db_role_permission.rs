use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::{db_permission::PermissionId, db_role::RoleId};
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbRolePermission {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}
