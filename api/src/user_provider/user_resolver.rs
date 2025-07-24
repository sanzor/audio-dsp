use std::{collections::HashMap, sync::Arc};

use actors::user_actor::{create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory, user_actor::UserActor, user_actor_registry::UserActorRegistry};
use domain::domain_user::DomainUser;
use dsp_core::tracks_provider::TracksProvider;
use tokio::sync::Mutex;

use crate::{controllers::google_controller::GoogleUserInfo, user_provider::{create_user_params::CreateUserParams, user_provider::UserProvider}};

pub struct UserResolver{
    // actors: Mutex<HashMap<String, UserActor>>,
    // player_factory: Arc<PlayerFactory>,
    // tracks_provider: Arc<dyn TracksProvider + Send + Sync>,
    user_provider:Arc<dyn UserProvider+Send+Sync>,
    user_registry:Arc<UserActorRegistry>,
}

impl UserResolver{
    pub async fn resolve_user(&self,google_user_info:&GoogleUserInfo)->Result<DomainUser,String>{
        let domain_user=match self.user_provider.get_user_by_google_sub_id(&google_user_info.sub).await{
            Some(user)=>user,
            None=>{
                let new_user_params=CreateUserParams{
                    email:google_user_info.email,
                    name:google_user_info.name,
                    picture:google_user_info.picture,
                    google_sub_id:Some(google_user_info.sub)
                };
                self.user_provider.create_user(new_user_params).await?
            }
        };
        self.user_registry.get_or_spawn_user_actor(&domain_user.id, domain_user, build_params)
        Ok(())
    }
}