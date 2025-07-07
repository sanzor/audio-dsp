use std::{collections::HashMap, sync::Arc};

use dsp_core::{command_processor::CommandProcessor, tracks_provider::TracksState};
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        create_user_data::CreateUserData, local_tracks_store_provider::TrackOperations,
        players_provider::PlayersProvider,
    },
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct CreateUserActorParams {
    pub user_data: CreateUserData,
    pub tracks_provider: Box<dyn TrackOperations + Send + Sync>,
    pub players: Box<dyn PlayersProvider + Send + Sync>,
}
