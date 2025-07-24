use std::{collections::HashMap, sync::Arc};

use domain::domain_user::DomainUser;
use tokio::sync::Mutex;

use crate::user_actor::{create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData, player_factory::PlayerFactory, user_actor::UserActor};

pub struct UserActorRegistry{
    user_actors:Arc<Mutex<HashMap<String,UserActor>>>,
    player_factory:Arc<PlayerFactory>
}

impl UserActorRegistry{
    pub async fn get_or_spawn_user_actor(&mut self,user_id:&str,domain_user:Option<DomainUser>)->Result<UserActor,String>{

        let user_actor=match self.user_actors.lock().await.get(user_id.clone()){
            Some(&u)=>return Ok(u),
            None
        }
        // let actor_params = CreateUserActorParams {
        // player_factory: Arc::clone(&app_state.player_factory),
        // user_data: CreateUserData {
        //     id: id.clone(),
        //     name: request.user_name.clone(),
        //     email: request.email.clone(),
        // },
        // players_provider: Box::new(LocalPlayerProvider::new()),
        // tracks_provider: Box::new(LocalTrackStoreProvider::new())
    
    }
}