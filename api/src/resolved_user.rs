use actors::user_actor::user_actor::UserActor;
use domain::domain_user::DomainUser;
use kameo::actor::ActorRef;

pub struct ResolvedUser {
    pub domain_user: DomainUser,
    pub actor: ActorRef<UserActor>,
}
