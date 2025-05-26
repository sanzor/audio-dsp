use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct ListDispatcher {}
impl CommandDispatch for ListDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let guard = state.try_read().map_err(|e| e.to_string())?;
        let state = &*guard;
        let result = match envelope.command {
            Message::Ls => self.internal_dispatch(state),
            _ => Err("".to_owned()),
        };
        return result;
    }
}

impl ListDispatcher {
    fn internal_dispatch(&self, state: &State) -> Result<DspCommandResult, String> {
        let tracks = state.tracks();
        Ok(DspCommandResult {
            output: serde_json::to_string_pretty(&tracks).unwrap(),
            should_exit: false,
        })
    }
}
