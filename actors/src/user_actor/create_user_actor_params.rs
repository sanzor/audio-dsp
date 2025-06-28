use std::{collections::HashMap, sync::Arc};

use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::create_user_data::CreateUserData,
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct CreateUserActorParams {
    pub user_data: CreateUserData,
    pub processor: Arc<CommandProcessor>,
    pub tracks: TracksState,
    pub players: Players,
}
