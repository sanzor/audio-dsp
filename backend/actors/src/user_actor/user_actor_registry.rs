use std::{collections::HashMap, sync::Arc};

use kameo::actor::{ActorRef, Spawn};
use tokio::sync::Mutex;

use crate::user_actor::{actor::UserActor, create_user_actor_params::CreateUserActorParams};

pub struct UserActorRegistry {
    user_actors: Arc<Mutex<HashMap<i64, ActorRef<UserActor>>>>,
}

impl UserActorRegistry {
    pub fn new() -> UserActorRegistry {
        UserActorRegistry {
            user_actors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn get_or_spawn_user_actor(
        &self,
        user_id: i64,
        build_params: CreateUserActorParams,
    ) -> Result<ActorRef<UserActor>, String> {
        let maybe_actor = {
            let map = self.user_actors.lock().await;
            map.get(&user_id).cloned()
        };
        if let Some(existing_actor) = maybe_actor {
            return Ok(existing_actor);
        }

        let actor = UserActor::spawn(UserActor::new(build_params));

        let mut actors = self.user_actors.lock().await;
        actors.insert(user_id, actor.clone());
        Ok(actor)
    }

    pub async fn get_actor(&self, user_id: i64) -> Option<ActorRef<UserActor>> {
        let guard = self.user_actors.lock().await;
        guard.get(&user_id).cloned()
    }
}
impl Default for UserActorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
