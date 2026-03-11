use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

#[derive(Deserialize, ToSchema, Debug)]
pub struct RateLimitPath {
    pub org_id: OrganizationId,
}
