use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MeOrganizationResult {
    pub id: OrganizationId,
    pub name: String,
    pub slug: String,
    pub billing_email: String,
    pub stripe_customer_id: Option<String>,
    pub status: String,
}
