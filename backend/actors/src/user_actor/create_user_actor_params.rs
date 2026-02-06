use std::sync::Arc;

use domain::domain_user::DomainUser;

use crate::user_actor::user_actor_deps::UserActorDeps;

pub struct CreateUserActorParams {
    pub user_data: DomainUser,
    pub user_actor_deps: Arc<UserActorDeps>,
}
