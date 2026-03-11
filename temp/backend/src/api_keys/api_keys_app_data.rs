use std::sync::Arc;

use crate::api_keys::api_keys_provider::ApiKeysProvider;

#[derive(Clone)]
pub struct ApiKeysAppData {
    pub api_keys_provider: Arc<dyn ApiKeysProvider>,
}
