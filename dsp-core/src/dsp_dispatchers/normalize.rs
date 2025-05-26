use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

pub(crate) struct NormalizeDispatcher {}
impl CommandDispatch for NormalizeDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;
        match envelope.command {
            Message::Normalize {
                name,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(name, state),
            _ => Err("err".to_string()),
        }
    }
}

impl NormalizeDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to perform normalize on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.normalize_mut();
        Ok(DspCommandResult {
            output: format!("Normalize track {} succesful", name),
            should_exit: false,
        })
    }
}
