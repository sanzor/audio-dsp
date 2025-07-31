use std::sync::Arc;

use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, user_actor_registry::UserActorRegistry,
};
use data_provider::user_provider::UserProvider;
use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};

use crate::{controllers::google_controller::GoogleUserInfo, resolved_user::ResolvedUser};
pub struct LocalUserResolver {
    user_provider: Arc<dyn UserProvider>,
    user_registry: Arc<UserActorRegistry>,
}

impl LocalUserResolver {
    pub fn new(
        user_provider: Arc<dyn UserProvider>,
        user_registry: Arc<UserActorRegistry>,
    ) -> LocalUserResolver {
        LocalUserResolver {
            user_provider: Arc::clone(&user_provider),
            user_registry: Arc::clone(&user_registry),
        }
    }

    pub async fn resolve_or_create_user<F>(
        &self,
        google_user_info: &GoogleUserInfo,
        build_actor_params: F,
    ) -> Result<DomainUser, String>
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
        let actor_deps = build_actor_params(user)?;
        let actor = self
            .user_registry
            .get_or_spawn_user_actor(&user_id, actor_deps)
            .await?;
        todo!()
    }

    pub async fn resolve_existing_user(&self, user_id: &str) -> Result<ResolvedUser, String> {
        let maybe_actor = match self.user_provider.get_user_by_id(&user_id).await {
            Some(u) => match self.user_registry.get_actor(&u.id).await {
                Some(actor) => Ok(ResolvedUser {
                    actor: actor,
                    domain_user: u,
                }),
                None => Err("Could not find actor".to_string()),
            },
            None => Err("Could not find user".into()),
        };
        maybe_actor
    }
}
