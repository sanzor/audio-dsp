use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::db::db_organization::OrganizationId, me::me_membership_result::MeMembershipResult,
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrganizationDataResult {
    pub org_id: OrganizationId,
    pub token: String,
    pub membership: MeMembershipResult,
    pub permissions: Vec<String>,
}
