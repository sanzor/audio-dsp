use actors::user_actor::actor::UserActor;
use domain::domain_user::DomainUser;
use kameo::actor::ActorRef;

pub struct ResolvedUserAndActor {
    pub user: DomainUser,
    pub actor: ActorRef<UserActor>,
}
