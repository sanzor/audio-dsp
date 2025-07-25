use audiolib::audio_buffer::AudioBuffer;
use dsp_core::tracks_provider::TracksProvider;
use kameo::{actor::ActorRef, Actor};
use std::{collections::HashMap, sync::Arc};
use domain::actors::user_actor_init_input::UserActorInitInput;
use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory,
        players_provider::PlayersProvider,
    },
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) tracks_provider: Arc<dyn TracksProvider + Send + Sync>,
    pub(crate) players_provider: Box<dyn PlayersProvider + Send + Sync>,
    pub(crate) user_provider:Arc<dyn UserProvider>,
    pub(crate) player_factory: Arc<PlayerFactory>,
    pub(crate) loaded_payloads: HashMap<String, Arc<AudioBuffer>>,
}

impl UserActor {
    pub async fn new(actor_params: CreateUserActorParams) -> UserActor {
        match actor_params.user_data{
                UserActorInitInput::Existing(E)=>{
                    todo!()
            },
                UserActorInitInput::New(actor_params)=>{
                UserActor {
                    id: actor_params.user_data.id,
                     email: actor_params.user_data.email,
                     name: actor_params.user_data.name,
            tracks_provider: Arc::clone(&actor_params.tracks_provider),
            players_provider: actor_params.players_provider,
            player_factory: Arc::clone(&actor_params.player_factory),
            loaded_payloads: HashMap::new(),
        }
            }
        }
        
    }
}
