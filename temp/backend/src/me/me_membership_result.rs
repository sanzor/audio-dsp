use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::db::{db_organization::OrganizationId, db_user::UserId},
    me::me_organization_result::MeOrganizationResult,
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeMembershipResult {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub roles: Vec<String>,
    pub organization: Option<MeOrganizationResult>,
}
