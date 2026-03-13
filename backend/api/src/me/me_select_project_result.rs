use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeSelectProjectResult {
    pub project_id: String,
    pub token: String,
}
