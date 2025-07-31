use std::sync::Arc;

use actors::user_actor::user_actor_deps::UserActorDeps;

use crate::local_user_resolver::LocalUserResolver;

#[derive(Clone)]
pub struct AppData {
    pub user_resolver: Arc<LocalUserResolver>,
    pub user_actor_deps: Arc<UserActorDeps>,
}
