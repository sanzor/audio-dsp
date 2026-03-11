#[derive(Clone, Debug)]
pub struct CreateOrganizationParams {
    pub name: String,
    pub slug: String,
    pub billing_email: String,
    pub stripe_customer_id: Option<String>,
    pub status: Option<String>,
}
