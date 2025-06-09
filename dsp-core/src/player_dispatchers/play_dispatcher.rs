use async_trait::async_trait;
use dsp_domain::{
    dsp_message::DspMessage, dsp_message_result::DspMessageResult, envelope::Envelope,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub struct PlayDispatcher {}

#[async_trait]
impl CommandDispatch for PlayDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<dsp_domain::dsp_message_result::DspMessageResult, String> {
        match envelope.command {
            DspMessage::Play {
                user_id: user_id,
                track_id: track_id,
            } => self.internal_dispatch(user_id, track_id, state).await,
            _ => Err("".to_string()),
        }
    }
}
impl PlayDispatcher {
    async fn internal_dispatch(
        &self,
        user_id: Option<String>,
        track_id: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        todo!()
    }
}
