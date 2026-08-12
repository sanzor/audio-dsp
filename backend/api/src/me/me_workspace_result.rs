use domain::workspace_role::WorkspaceRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeWorkspaceResult {
    pub workspace_id: i32,
    pub name: String,
    pub role: WorkspaceRole,
}
