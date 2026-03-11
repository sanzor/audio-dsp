#[derive(Clone, Debug)]
pub struct UpdateApiKeyParams {
    pub label: Option<String>,
    pub is_active: Option<bool>,
}
