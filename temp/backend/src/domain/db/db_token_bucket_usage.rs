use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DbTokenBucketUsage {
    pub org_id: OrganizationId,
    pub limit: Option<i64>,
    pub bucket_tokens: Option<i64>,
}
