use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use dsp_domain::{
    command::{CommandResult, DspCommand},
    envelope::Envelope,
};
pub(crate) struct ListDispatcher {}
impl CommandDispatch for ListDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let guard = state.read().unwrap();
        let result = match envelope.command {
            DspCommand::Ls => self.internal_dispatch(&*guard),
            _ => Err("".to_owned()),
        };
        return result;
    }
}

impl ListDispatcher {
    fn internal_dispatch(&self, state: &State) -> Result<CommandResult, String> {
        let tracks = state.tracks();
        Ok(CommandResult {
            output: serde_json::to_string_pretty(&tracks).unwrap(),
        })
    }
}
