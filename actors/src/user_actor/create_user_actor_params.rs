use std::{collections::HashMap, sync::Arc};

use domain::domain_user::DomainUser;
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
     user_actor_deps::UserActorDeps
    },
};
type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
    pub struct CreateUserActorParams {
        pub user_data: DomainUser,
        pub user_actor_deps:Arc<UserActorDeps>
    }
