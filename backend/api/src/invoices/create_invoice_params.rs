use crate::domain::db::db_organization::OrganizationId;

pub struct CreateInvoiceParams {
    pub org_id: OrganizationId,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
}
