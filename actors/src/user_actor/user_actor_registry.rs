use std::{collections::HashMap, sync::Arc};

use domain::domain_user::DomainUser;
use kameo::{actor::ActorRef, Actor};
use tokio::sync::Mutex;

use crate::user_actor::{create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData, player_factory::PlayerFactory, user_actor::UserActor};

pub struct UserActorRegistry{
    user_actors:Arc<Mutex<HashMap<String,ActorRef<UserActor>>>>,
}

impl UserActorRegistry{
    pub async fn get_or_spawn_user_actor(
        &mut self,
        user_id:&str,
        domain_user:Option<DomainUser>,
        build_params:impl FnOnce(&DomainUser)->CreateUserActorParams)
        ->Result<ActorRef<UserActor>,String>{

        let map=self.user_actors.lock().await;
        if let Some(actor)=map.get(user_id){
            return Ok(actor.clone())
        }
        let domain_user=match domain_user{
            Some(u)=>u,
            None=>return Err("Invalid domain user provided".into())
        };
        let params=build_params(&domain_user);
        let actor=UserActor::spawn(UserActor::new(params));
           
        let mut actors=self.user_actors.lock().await;
        actors.insert(domain_user.id,actor.clone());
        Ok(actor)
        
    }
}