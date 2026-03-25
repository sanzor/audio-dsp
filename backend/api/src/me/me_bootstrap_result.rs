use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::me::me_project_result::MeProjectResult;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeUserResult {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_verified: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeBootstrapResult {
    pub user: MeUserResult,
    pub projects: Vec<MeProjectResult>,
}
