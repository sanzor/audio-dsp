use domain::project_role::ProjectRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeProjectResult {
    pub project_id: i32,
    pub name: String,
    pub role: ProjectRole,
}
