use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbMembershipRole {
    pub user_id: i64,
    pub org_id: i64,
    pub role_id: i64,
}
