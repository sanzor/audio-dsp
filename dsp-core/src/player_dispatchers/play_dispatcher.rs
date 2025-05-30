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
            Message::Play { name: name } => self.internal_dispatch(name, state).await,
            _ => Err("".to_string()),
        }
    }
}
impl PlayDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        todo!()
    }
}
