use std::sync::Arc;

use super::{auth_provider::AuthProvider, jwt_provider::JwtProvider};

#[derive(Clone)]
pub struct AuthAppData {
    pub auth_provider: Arc<dyn AuthProvider>,
    pub jwt_provider: Arc<dyn JwtProvider>
}
