use crate::domain::db::db_api_key::ApiKeyId;

#[derive(Clone, Debug)]
pub struct UpdateApiKeyParams {
    pub id: ApiKeyId,
    pub label: Option<String>,
    pub is_active: Option<bool>,
}
