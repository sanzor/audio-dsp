use std::sync::Arc;

use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, user_actor_registry::UserActorRegistry,
};
use data_provider::users::user_provider::UserProvider;
use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};

use crate::{
    dtos::google_user_info::GoogleUserInfo,
    user_and_actor_resolver::resolved_user_and_actor::ResolvedUserAndActor,
};
pub struct LocalUserAndActorResolver {
    user_provider: Arc<dyn UserProvider>,
    user_registry: Arc<UserActorRegistry>,
}

impl LocalUserAndActorResolver {
    pub fn new(
        user_provider: Arc<dyn UserProvider>,
        actor_registry: Arc<UserActorRegistry>,
    ) -> LocalUserAndActorResolver {
        LocalUserAndActorResolver {
            user_provider: Arc::clone(&user_provider),
            user_registry: Arc::clone(&actor_registry),
        }
    }

    pub async fn resolve_google_user_and_actor<F>(
        &self,
        google_user_info: &GoogleUserInfo,
        build_actor_params: F,
    ) -> Result<ResolvedUserAndActor, String>
    where
        F: FnOnce(DomainUser) -> Result<CreateUserActorParams, String>,
    {
        let user = match self
            .user_provider
            .get_user_by_google_sub_id(&google_user_info.sub)
            .await
        {
            Some(user) => user,
            None => {
                let new_user_params = CreateDomainUserParams {
                    email: google_user_info.email.clone(),
                    name: google_user_info.name.clone(),
                    picture: google_user_info.picture.clone(),
                    google_sub_id: Some(google_user_info.sub.clone()),
                };
                self.user_provider
                    .create_domain_user(new_user_params)
                    .await?
            }
        };
        let user_id = user.id.clone();
        let actor_deps = build_actor_params(user.clone())?;
        let actor = self
            .user_registry
            .get_or_spawn_user_actor(&user_id, actor_deps)
            .await?;
        Ok(ResolvedUserAndActor { actor, user })
    }

    pub async fn resolve_existing_user_and_actor(
        &self,
        user_id: &str,
    ) -> Result<ResolvedUserAndActor, String> {
        let user = self
            .user_provider
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| "Could not find user".to_string())?;
        let actor = self
            .user_registry
            .get_actor(&user.id)
            .await
            .ok_or_else(|| "Could not find actor".to_string())?;
        Ok(ResolvedUserAndActor { actor, user })
    }
}
