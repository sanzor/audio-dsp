#[derive(Clone, Debug)]
pub struct UpdateOrganizationParams {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub billing_email: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub status: Option<String>,
}
