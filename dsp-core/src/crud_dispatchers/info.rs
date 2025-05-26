use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct InfoDispatcher {}

impl CommandDispatch for InfoDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let guard = state.try_read().map_err(|e| e.to_string())?;
        let state = &*guard;
        match envelope.command {
            Message::Info { name } => self.internal_dispatch(name, state),
            _ => Err("".to_owned()),
        }
    }
}

impl InfoDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        state: &State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name")?;
        let track_info = state
            .get_track_info(&name)
            .ok_or(format!("Could not find track info for {}", name))?;
        Ok(DspCommandResult {
            output: serde_json::to_string_pretty(&track_info).unwrap(),
            should_exit: false,
        })
    }
}
