use crate::domain::db::db_api_key::DbApiKey;

#[derive(Clone, Debug)]
pub struct UpdateApiKeyResult {
    pub api_key: DbApiKey,
}
