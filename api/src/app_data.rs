use std::sync::Arc;

use crate::user_resolver::UserResolver;


#[derive(Clone)]
pub struct AppData {
    pub user_resolver:Arc<UserResolver>
}
