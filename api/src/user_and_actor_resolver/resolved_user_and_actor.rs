use actors::user_actor::user_actor::UserActor;
use domain::domain_user::DomainUser;
use kameo::actor::ActorRef;

pub struct ResolvedUserAndActor {
    pub user: DomainUser,
    pub actor: ActorRef<UserActor>,
}
