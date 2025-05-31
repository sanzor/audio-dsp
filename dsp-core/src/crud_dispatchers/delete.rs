use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use dsp_domain::{
    message_result::MessageResult, envelope::Envelope, message::Message, user,
};
pub(crate) struct DeleteDispatcher {}

#[async_trait]
impl CommandDispatch for DeleteDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<MessageResult, String> {
        match envelope.command {
            Message::Delete {
                user_name,
                track_name,
            } => self.internal_dispatch(user_name, track_name, state).await,
            _ => Err("".to_string()),
        }
    }
}
impl DeleteDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        state: SharedState,
    ) -> Result<MessageResult, String> {
        let user_name = user_name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let name = track_name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let _ = state.delete_track(&user_name, &name).await?;
        Ok(MessageResult {
            output: format!("Delete track {} succesful", &name),
            should_exit: false,
        })
    }
}
