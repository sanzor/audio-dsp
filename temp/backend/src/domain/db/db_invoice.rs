use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

pub type InvoiceId = i64;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbInvoice {
    pub id: InvoiceId,
    pub org_id: OrganizationId,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
    pub created_at: Option<String>,
}
