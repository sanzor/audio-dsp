use crate::services::api_keys::api_key::ApiKey;

#[derive(Clone, Debug)]
pub struct GetApiKeyResult {
    pub api_key: ApiKey,
}
