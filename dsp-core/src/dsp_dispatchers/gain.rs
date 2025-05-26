use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

pub(crate) struct GainDispatcher {}

impl CommandDispatch for GainDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;

        match envelope.command {
            Message::Gain {
                name,
                gain,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(name, gain, state),
            _ => Err("err".to_string()),
        }
    }
}

impl GainDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to perform gain on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .ok_or("Could not find track ref")?;
        let _ = track_ref.inner.data.gain_mut(cutoff);
        Ok(DspCommandResult {
            output: format!("Updated gain for track {} succesful", name),
            should_exit: false,
        })
    }
}
