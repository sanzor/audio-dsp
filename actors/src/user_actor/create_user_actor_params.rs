use std::{collections::HashMap, sync::Arc};

use domain::actors::user_actor_init_input::UserActorInitInput;
use dsp_core::tracks_provider::TracksProvider;
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        player_factory::PlayerFactory,
        players_provider::PlayersProvider,
    },
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
    pub struct CreateUserActorParams {
        pub user_data: UserActorInitInput,
        pub tracks_provider: Arc<dyn TracksProvider + Send + Sync>,
        pub players_provider: Box<dyn PlayersProvider + Send + Sync>,
        pub player_factory: Arc<PlayerFactory>,
    }
