use std::sync::Arc;

use crate::users::user_crud_provider::UserCrudProvider;

#[derive(Clone)]
pub struct UsersAppData {
    pub user_provider: Arc<dyn UserCrudProvider>,
}
