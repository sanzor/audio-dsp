use std::{collections::HashMap, sync::Arc};

use actors::{audio_player_actor::audio_player_actor::AudioPlayerActor, user_actor::user_actor::UserActor};
use kameo::actor::ActorRef;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppData{
    pub user_map:Arc<Mutex<HashMap<String,ActorRef<UserActor>>>>
}