use rust_shared::organization_id::OrganizationId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::db::db_role::RoleId;
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrgMemberRole {
    pub id: RoleId,
    pub name: String,
    pub is_system: bool,
    pub org_id: Option<OrganizationId>,
}
