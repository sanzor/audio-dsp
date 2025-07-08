use std::{collections::HashMap, sync::Arc};

use actors::user_actor::{player_factory::PlayerFactory, user_actor::UserActor};
use kameo::actor::ActorRef;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppData{
    pub user_map:Arc<Mutex<HashMap<String,ActorRef<UserActor>>>>,
    pub player_factory:Arc<PlayerFactory>
}