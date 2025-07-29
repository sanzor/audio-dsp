use audiolib::audio_buffer::AudioBuffer;
use data_provider::{tracks_provider::TracksProvider, user_provider::UserProvider};
use kameo::{actor::ActorRef, Actor};
use std::{collections::HashMap, sync::Arc};
use domain::{actors::user_actor_init_input::UserActorInitInput, domain_user::DomainUser};
use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory
    },
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    pub(crate) user_data:DomainUser,
    pub(crate) tracks_provider: Arc<dyn TracksProvider+Send+Sync+'static>,
    pub(crate) players_provider: HashMap<String,ActorRef<AudioPlayerActor>>,
    pub(crate) player_factory: Arc<PlayerFactory>,
    pub(crate) loaded_payloads: HashMap<String, Arc<AudioBuffer>>,
}

impl UserActor {
    pub async fn new(actor_params: CreateUserActorParams) -> UserActor {
        match actor_params.user_data{
                UserActorInitInput::Existing(E)=>{
                    todo!()
                },
                UserActorInitInput::New(new_actor_params)=>{
                    let new_user=actor_params.user_actor_deps.user_provider.create_domain_user(new_actor_params).await.unwrap();
                UserActor {
                    
                    tracks_provider: Arc::clone(&actor_params.user_actor_deps.tracks_provider),
                    players_provider: HashMap::new(),
                    player_factory: Arc::clone(&actor_params.user_actor_deps.player_factory),
                    loaded_payloads: HashMap::new(),
                    user_provider:actor_params.user_actor_deps.user_provider
                }
            }
        }
        
    }
}
