use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use dsp_domain::{
    dsp_message::DspMessage, dsp_message_result::DspMessageResult, envelope::Envelope, user,
};
use std::sync::Arc;
use tokio::sync::Mutex;
pub(crate) struct DeleteDispatcher {}

#[async_trait]
impl CommandDispatch for DeleteDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Delete {
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
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        let user_name = user_name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let name = track_name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let mut state_guard = state.lock().await;
        let _ = state_guard.delete_track(&name).await?;
        Ok(DspMessageResult {
            output: format!("Delete track {} succesful", &name),
            should_exit: false,
        })
    }
}
