use std::{collections::HashMap, sync::Arc};

use actors::user_actor::user_actor::UserActor;
use dsp_core::command_processor::CommandProcessor;
use kameo::actor::ActorRef;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppData{
    pub processor:Arc<CommandProcessor>,
    pub user_map:Arc<Mutex<HashMap<String,ActorRef<UserActor>>>>
}