use std::sync::Arc;

use crate::{command_dispatch::CommandDispatch, state::State};
use async_trait::async_trait;
use dsp_domain::{dsp_command_result::DspCommandResult, message::Message};
pub(crate) struct StopDispatcher {}

#[async_trait]
impl CommandDispatch for StopDispatcher {
    async fn dispatch(
        &self,
        envelope: dsp_domain::envelope::Envelope,
        state: Arc<State>,
    ) -> Result<dsp_domain::dsp_command_result::DspCommandResult, String> {
        match envelope.command {
            Message::Stop { user_id, track_id } => self.internal_dispatch(user_id, track_id, state),
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
    ) -> Result<DspCommandResult, String> {
        Err("is".into())
    }
}
