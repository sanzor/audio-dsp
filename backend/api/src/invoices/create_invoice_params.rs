pub struct CreateInvoiceParams {
    pub user_id: i64,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
}
