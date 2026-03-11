use crate::domain::db::db_organization::OrganizationId;

pub struct CreateTokenPackCheckoutParams {
    pub org_id: OrganizationId,
    pub org_slug: String,
    pub product_id: i64,
    pub success_url: Option<String>,
    pub cancel_url: Option<String>,
}
