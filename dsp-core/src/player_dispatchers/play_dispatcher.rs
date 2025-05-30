use async_trait::async_trait;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub(crate) struct PlayDispatcher {}

#[async_trait]
impl CommandDispatch for PlayDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<dsp_domain::dsp_command_result::DspCommandResult, String> {
        match envelope.command {
            Message::Play {
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
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        todo!()
    }
}
