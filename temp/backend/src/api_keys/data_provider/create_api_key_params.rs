#[derive(Clone, Debug)]
pub struct CreateApiKeyParams {
    pub org_id: u64,
    pub label: Option<String>,
    pub created_by: Option<u64>,
}
