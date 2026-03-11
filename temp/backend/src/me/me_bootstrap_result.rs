use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{me::me_membership_result::MeMembershipResult, users::user_result::UserResult};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeBootstrapResult {
    pub user: UserResult,
    pub organizations: Vec<MeMembershipResult>,
}
