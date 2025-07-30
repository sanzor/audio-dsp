use std::sync::Arc;

use actors::user_actor::{create_user_actor_params::CreateUserActorParams, user_actor_registry::UserActorRegistry};
use data_provider::user_provider::UserProvider;
use domain::{ create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};

use crate::{controllers::google_controller::GoogleUserInfo};
pub struct UserResolver{
    user_provider:Arc<dyn UserProvider+Send+Sync>,
    user_registry:Arc<UserActorRegistry>,
}

impl UserResolver{

    pub fn new(user_provider:Arc<dyn UserProvider+Send+Sync>,user_registry:Arc<UserActorRegistry>)->UserResolver{
        UserResolver { user_provider: Arc::clone(&user_provider), user_registry: Arc::clone(&user_registry) }
    }
    pub async fn resolve_user<F>(&self,
        google_user_info:&GoogleUserInfo,
        build_actor_params:F)
        where F:FnOnce(DomainUser)->Result<CreateUserActorParams,String>{
        let user=match self.user_provider.get_user_by_google_sub_id(&google_user_info.sub).await{
            Some(user)=>user,
            None=>{
                let new_user_params=CreateDomainUserParams{
                    email:google_user_info.email.clone(),
                    name:google_user_info.name.clone(),
                    picture:google_user_info.picture.clone(),
                    google_sub_id:Some(google_user_info.sub.clone())
                };
                self.user_provider.create_domain_user(new_user_params).await?
            }
        };
        let actor_deps=build_actor_params(user)?;
        let actor=
            self.user_registry.get_or_spawn_user_actor(&user.id, actor_deps).await?;
        todo!()
    }
}