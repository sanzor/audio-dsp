use std::sync::Arc;

use crate::users::user_provider::UserProvider;

#[derive(Clone)]
pub struct UsersAppData {
    pub user_provider: Arc<dyn UserProvider>,
}
