use std::sync::{Arc, Mutex};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use async_trait::async_trait;
use dsp_domain::{dsp_message::DspMessage, dsp_message_result::DspMessageResult};
pub(crate) struct StopDispatcher {}

#[async_trait]
impl CommandDispatch for StopDispatcher {
    async fn dispatch(
        &self,
        envelope: dsp_domain::envelope::Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<dsp_domain::dsp_message_result::DspMessageResult, String> {
        match envelope.command {
            DspMessage::Stop { user_id, track_id } => {
                self.internal_dispatch(user_id, track_id, state)
            }
            _ => Err("Invalid stop message".to_string()),
        }
    }
}
impl StopDispatcher {
    fn internal_dispatch(
        &self,
        user_id: Option<String>,
        track_id: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        Err("is".into())
    }
}
