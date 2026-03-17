use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type UserId = i64;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbUser {
    pub id: UserId,
    pub email: String,
    pub full_name: String,
    pub is_active: bool,
    pub is_verified: bool,
}
