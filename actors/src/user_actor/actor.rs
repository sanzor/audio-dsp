use crate::{
    audio_player_actor::actor::AudioPlayerActor,
    user_actor::{create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory},
};
use data_provider::{region_set_provider::RegionSetsProvider, tracks_provider::TracksProvider};
use domain::{domain_user::DomainUser, stored_track::StoredTrack};
use kameo::{actor::ActorRef, Actor};

use std::{collections::HashMap, sync::Arc};

#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    pub(crate) user_data: DomainUser,
    pub(crate) tracks_provider: Arc<dyn TracksProvider + Send + Sync + 'static>,
    pub(crate) region_set_provider: Arc<dyn RegionSetsProvider + Send + Sync + 'static>,
    pub(crate) players_provider: HashMap<String, ActorRef<AudioPlayerActor>>,
    pub(crate) player_factory: Arc<PlayerFactory>,
    pub(crate) cached_tracks: HashMap<String, Arc<StoredTrack>>,
}

impl UserActor {
    pub fn new(actor_params: CreateUserActorParams) -> UserActor {
        UserActor {
            tracks_provider: Arc::clone(&actor_params.user_actor_deps.tracks_provider),
            region_set_provider: Arc::clone(&actor_params.user_actor_deps.region_sets_provider),
            players_provider: HashMap::new(),
            player_factory: Arc::clone(&actor_params.user_actor_deps.player_factory),
            cached_tracks: HashMap::new(),
            user_data: actor_params.user_data,
        }
    }
}
