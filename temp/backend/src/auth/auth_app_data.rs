use std::sync::Arc;

use crate::auth::auth_provider::AuthProvider;
use crate::middlewares::jwt::jwt_provider::JwtProvider;

#[derive(Clone)]
pub struct AuthAppData {
    pub auth_provider: Arc<dyn AuthProvider>,
    pub jwt_provider: Arc<dyn JwtProvider>,
}
