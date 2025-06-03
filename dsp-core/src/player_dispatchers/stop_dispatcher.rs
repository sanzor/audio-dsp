use std::sync::Arc;

use crate::{command_dispatch::CommandDispatch, state::State};
use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, dsp_message::DspMessage};
pub(crate) struct StopDispatcher {}

#[async_trait]
impl CommandDispatch for StopDispatcher {
    async fn dispatch_mut(
        &self,
        envelope: dsp_domain::envelope::Envelope,
        state: Arc<State>,
    ) -> Result<dsp_domain::dsp_message_result::DspMessageResult, String> {
        match envelope.command {
            DspMessage::Stop { user_id, track_id } => self.internal_dispatch(user_id, track_id, state),
            _ => Err("Invalid stop message".to_string()),
        }
    }
}
impl StopDispatcher {
    fn internal_dispatch(
        &self,
        user_id: Option<String>,
        track_id: Option<String>,
        state: Arc<State>,
    ) -> Result<DspMessageResult, String> {
        Err("is".into())
    }
}
