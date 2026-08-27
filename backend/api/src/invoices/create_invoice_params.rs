pub struct CreateInvoiceParams {
    pub user_id: domain::domain_user::UserId,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
}
