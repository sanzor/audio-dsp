use crate::services::api_keys::api_key::ApiKey;

#[derive(Clone, Debug)]
pub struct ListApiKeysResult {
    pub api_keys: Vec<ApiKey>,
}
