use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct RunScriptDispatcher {}

impl CommandDispatch for RunScriptDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;

        match envelope.command {
            Message::Copy { name, copy_name } => self.internal_dispatch(name, copy_name, state),
            _ => Err("Could not perform copy".to_owned()),
        }
    }
}

impl RunScriptDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        copy_name: Option<String>,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let fname = name.ok_or("Invalid name for copy")?;
        let mut new_track = state
            .get_track_copy(&fname.clone())
            .ok_or("Could not find track")?;

        let copy_name = copy_name.unwrap_or_else(|| new_track.info.name.clone() + "v2");
        new_track.info.name = copy_name.clone();
        let _ = state.upsert_track(new_track);
        Ok(DspCommandResult {
            output: format!("Copied successfully track:{} to {}", fname, copy_name),
            should_exit: false,
        })
    }
}
