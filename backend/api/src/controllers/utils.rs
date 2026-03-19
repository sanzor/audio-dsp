use actors::user_actor::actor::UserActor;
use kameo::actor::ActorRef;

use crate::app_data::AppData;

pub async fn get_user_actor_internal(
    user_id: i64,
    app_state: &AppData,
) -> Result<ActorRef<UserActor>, String> {
    let user_addr = {
        let guard = app_state
            .user_resolver
            .resolve_existing_user_and_actor(user_id)
            .await;
        match guard {
            Ok(r_user) => Ok(r_user.actor),
            Err(_e) => Err("Could not find user".to_string()),
        }
    };
    user_addr
}
