use std::{collections::HashMap, sync::Arc};

use kameo::{actor::ActorRef, Actor};
use tokio::sync::Mutex;

use crate::user_actor::{create_user_actor_params::CreateUserActorParams, user_actor::UserActor};

pub struct UserActorRegistry {
    user_actors: Arc<Mutex<HashMap<String, ActorRef<UserActor>>>>,
}

impl UserActorRegistry {
    pub fn new() -> UserActorRegistry {
        UserActorRegistry {
            user_actors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn get_or_spawn_user_actor(
        &self,
        user_id: &str,
        build_params: CreateUserActorParams,
    ) -> Result<ActorRef<UserActor>, String> {
        let maybe_actor = {
            let map = self.user_actors.lock().await;
            map.get(user_id).cloned()
        };
        if let Some(existing_actor) = maybe_actor {
            return Ok(existing_actor);
        }

        let actor = UserActor::spawn(UserActor::new(build_params));

        let mut actors = self.user_actors.lock().await;
        actors.insert(user_id.to_string(), actor.clone());
        Ok(actor)
    }

    pub async fn get_actor(&self, user_id: &str) -> Option<ActorRef<UserActor>> {
        let guard = self.user_actors.lock().await;
        guard.get(user_id).cloned()
    }
}
