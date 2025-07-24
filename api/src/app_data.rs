use std::{collections::HashMap, sync::Arc};

use actors::user_actor::{player_factory::PlayerFactory, user_actor::UserActor};
use kameo::actor::ActorRef;
use tokio::sync::Mutex;

use crate::user_provider::user_provider::UserProvider;

#[derive(Clone)]
pub struct AppData {
    pub user_map: Arc<Mutex<HashMap<String, ActorRef<UserActor>>>>,
    pub player_factory: Arc<PlayerFactory>,
    pub user_provider:Arc<dyn UserProvider>
}
