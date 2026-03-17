use std::sync::Arc;

use crate::user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver;

#[derive(Clone)]
pub struct UserAppData {
    pub user_resolver: Arc<LocalUserAndActorResolver>,
}
