use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::me::me_workspace_result::MeWorkspaceResult;

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
    pub workspaces: Vec<MeWorkspaceResult>,
}
