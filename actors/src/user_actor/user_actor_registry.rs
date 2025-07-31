use std::{collections::HashMap, sync::Arc};

use kameo::{actor::ActorRef, Actor};
use tokio::sync::Mutex;

use crate::user_actor::{create_user_actor_params::CreateUserActorParams, user_actor::UserActor};

pub struct UserActorRegistry{
    user_actors:Arc<Mutex<HashMap<String,ActorRef<UserActor>>>>,
}

impl UserActorRegistry{
    pub async fn get_or_spawn_user_actor(
        &self,
        user_id:&str,
        build_params:CreateUserActorParams)
        ->Result<ActorRef<UserActor>,String>{

        let map=self.user_actors.lock().await;
        if let Some(actor)=map.get(user_id){
            return Ok(actor.clone())
        }

    
        let actor=UserActor::spawn(UserActor::new(build_params));
           
        let mut actors=self.user_actors.lock().await;
        actors.insert(user_id.to_string(),actor.clone());
        Ok(actor)
        
    }

    pub async fn get_actor(&self,user_id:&str)->Option<ActorRef<UserActor>>{
        let guard=self.user_actors.lock().await;
        guard.get(user_id).cloned()
    }

}