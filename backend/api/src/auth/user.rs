use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub is_active: bool,
    pub is_verified: bool,
}
