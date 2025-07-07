use std::{collections::HashMap};


use dsp_core::tracks_provider::TracksProvider;
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        create_user_data::CreateUserData,
        players_provider::PlayersProvider,
    },
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct CreateUserActorParams {
    pub user_data: CreateUserData,
    pub tracks_provider: Box<dyn TracksProvider + Send + Sync>,
    pub players: Box<dyn PlayersProvider + Send + Sync>,
}
