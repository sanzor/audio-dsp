use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::members::OrgMemberRole;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrgMember {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub email: String,
    pub full_name: String,
    pub is_active: bool,
    pub roles: Vec<OrgMemberRole>,
}
