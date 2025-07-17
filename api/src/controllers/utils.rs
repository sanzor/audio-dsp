use actors::user_actor::user_actor::UserActor;
use kameo::actor::ActorRef;

use crate::app_data::AppData;

pub async fn get_user_internal(
    user_id: &str,
    app_state: &AppData,
) -> Result<ActorRef<UserActor>, String> {
    let user_addr = {
        let guard = app_state.user_map.lock().await;
        match guard.get(&user_id.to_string()).cloned() {
            Some(addr) => Ok(addr),
            None => Err("Could not find user".to_string()),
        }
    };
    user_addr
}
