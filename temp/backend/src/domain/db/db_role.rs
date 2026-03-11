use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

pub type RoleId = i64;
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbRole {
    pub id: RoleId,
    pub org_id: Option<OrganizationId>,
    pub name: String,
    pub is_system: bool,
}
