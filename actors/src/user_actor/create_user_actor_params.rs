use std::{collections::HashMap, sync::Arc};

use dsp_core::tracks_provider::TracksProvider;
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        create_user_data::CreateUserData, player_factory::PlayerFactory,
        players_provider::PlayersProvider,
    },
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct CreateUserActorParams {
    pub user_data: CreateUserData,
    pub tracks_provider: Box<dyn TracksProvider + Send + Sync>,
    pub players_provider: Box<dyn PlayersProvider + Send + Sync>,
    pub player_factory: Arc<PlayerFactory>,
}
