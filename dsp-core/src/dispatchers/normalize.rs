use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{
    command::{CommandResult, DspCommand},
    envelope::Envelope,
};
pub(crate) struct NormalizeDispatcher {}
impl CommandDispatch for NormalizeDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let rez = state
            .try_write()
            .map_err(|e| e.to_string())
            .and_then(|mut guard| {
                let s = &mut *guard;
                match envelope.command {
                    DspCommand::Normalize { name, mode: _ } => self.internal_dispatch(name, s),
                    _ => Err("err".to_string()),
                }
            });
        return rez;
    }
}

impl NormalizeDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        state: &mut State,
    ) -> Result<CommandResult, String> {
        let name = name.ok_or("Invalid name for track to perform normalize on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.normalize_mut();
        Ok(CommandResult {
            output: format!("Normalize track {} succesful", name),
        })
    }
}
