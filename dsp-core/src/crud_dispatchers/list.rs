use async_trait::async_trait;
use dsp_domain::{message_result::MessageResult, envelope::Envelope, message::Message};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct ListDispatcher {}

#[async_trait]
impl CommandDispatch for ListDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<MessageResult, String> {
        let result = match envelope.command {
            Message::Ls { user_name } => self.internal_dispatch(user_name, &state).await,
            _ => Err("".to_owned()),
        };
        return result;
    }
}

impl ListDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        state: &State,
    ) -> Result<MessageResult, String> {
        let user_name = user_name.ok_or_else(|| "Invalid user_name")?;
        let tracks = state.get_tracks_for_user(&user_name).await;
        Ok(MessageResult {
            output: serde_json::to_string_pretty(&tracks).unwrap(),
            should_exit: false,
        })
    }
}
