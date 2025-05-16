use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use dsp_domain::{
    command::{CommandResult, DspCommand},
    envelope::Envelope,
};
pub(crate) struct InfoDispatcher {}

impl CommandDispatch for InfoDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let result = state
            .try_read()
            .map_err(|e| e.to_string())
            .and_then(|guard| {
                let state_ref = &*guard;
                match envelope.command {
                    DspCommand::Info { name } => self.internal_dispatch(name, state_ref),
                    _ => Err("".to_owned()),
                }
            });
        return result;
    }
}

impl InfoDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        state: &State,
    ) -> Result<CommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name")?;
        let track_info = state
            .get_track_info(&name)
            .ok_or(format!("Could not find track info for {}", name))?;
        Ok(CommandResult {
            output: serde_json::to_string_pretty(&track_info).unwrap(),
        })
    }
}
