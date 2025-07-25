use std::sync::Arc;


#[derive(Clone)]
pub struct AppData {
    pub user_resolver:Arc<UserResolver>
}
