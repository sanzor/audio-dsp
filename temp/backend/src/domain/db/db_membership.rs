use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbMembership {
    pub user_id: UserId,
    pub org_id: OrganizationId,
}
