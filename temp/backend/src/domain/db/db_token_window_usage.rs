use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DbTokenWindowUsage {
    pub org_id: OrganizationId,
    pub limit: Option<i64>,
    pub window_size_secs: Option<i64>,
    pub window_counter: Option<i64>,
    pub window_start: Option<i64>,
}
