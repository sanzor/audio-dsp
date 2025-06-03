use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, dsp_message::DspMessage};
pub(crate) struct PauseDispatcher {}

#[async_trait]
impl CommandDispatch for PauseDispatcher {
    async fn dispatch_mut(
        &self,
        envelope: dsp_domain::envelope::Envelope,
        state: crate::state::SharedState,
    ) -> Result<dsp_domain::dsp_message_result::DspMessageResult, String> {
        match envelope.command {
            DspMessage::Pause { user_id, track_id } => {
                self.internal_dispatch(user_id, track_id, state).await
            }
            _ => Err("Invalid pause command".to_string()),
        }
    }
}
impl PauseDispatcher {
    async fn internal_dispatch(
        &self,
        user_id: Option<String>,
        track_id: Option<String>,
        state: SharedState,
    ) -> Result<DspMessageResult, String> {
        let name = track_id.ok_or_else(|| "invalid name".to_string())?;
        Err("".into())
    }
}
