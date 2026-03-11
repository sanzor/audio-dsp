use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::db::db_user::{DbUser, UserId};
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub full_name: String,
    pub is_active: bool,
    pub is_verified: bool,
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        Self {
            id: value.id,
            email: value.email,
            full_name: value.full_name,
            is_active: value.is_active,
            is_verified: value.is_verified,
        }
    }
}
