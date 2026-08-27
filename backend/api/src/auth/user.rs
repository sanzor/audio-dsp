use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AuthUser {
    pub id: domain::domain_user::UserId,
    pub email: String,
    pub name: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub is_verified: bool,
}
