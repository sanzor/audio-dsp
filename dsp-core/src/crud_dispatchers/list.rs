use async_trait::async_trait;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

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
    ) -> Result<DspCommandResult, String> {
        let result = match envelope.command {
            Message::Ls => self.internal_dispatch(&state).await,
            _ => Err("".to_owned()),
        };
        return result;
    }
}

impl ListDispatcher {
    async fn internal_dispatch(&self, state: &State) -> Result<DspCommandResult, String> {
        let tracks = state.tracks().await;
        Ok(DspCommandResult {
            output: serde_json::to_string_pretty(&tracks).unwrap(),
            should_exit: false,
        })
    }
}
