use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeSelectProjectResult {
    pub project_id: i64,
    pub token: String,
}
