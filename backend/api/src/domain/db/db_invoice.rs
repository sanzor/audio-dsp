use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type InvoiceId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbInvoice {
    pub id: InvoiceId,
    pub user_id: String,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
    pub created_at: String,
}
