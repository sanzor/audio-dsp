use std::sync::Arc;

use actors::user_actor::user_actor_deps::UserActorDeps;

use crate::user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver;

#[derive(Clone)]
pub struct AppData {
    pub user_resolver: Arc<LocalUserAndActorResolver>,
    pub user_actor_deps: Arc<UserActorDeps>,
}
