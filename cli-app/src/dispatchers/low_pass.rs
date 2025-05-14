use audiolib::audio_transform::AudioTransformMut;

use crate::{
    command::{Command, CommandResult},
    command_dispatch::CommandDispatch,
    envelope::Envelope,
    state::{SharedState, State},
};
pub struct LowPassDispatcher {}

impl CommandDispatch for LowPassDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let rez = state
            .try_write()
            .map_err(|e| e.to_string())
            .and_then(|mut guard| {
                let s = &mut *guard;
                match envelope.command {
                    Command::LowPass { name, cutoff } => self.internal_dispatch(name, cutoff, s),
                    _ => Err("err".to_string()),
                }
            });
        return rez;
    }
}

impl LowPassDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: &mut State,
    ) -> Result<CommandResult, String> {
        let name = name.ok_or("Invalid name for track to low_pass on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.low_pass_mut(cutoff);
        Ok(CommandResult {
            output: format!("Normalize track {} succesful", name),
        })
    }
}
