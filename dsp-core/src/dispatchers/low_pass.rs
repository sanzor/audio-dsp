use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{
    dsp_command::DspCommand, dsp_command_result::DspCommandResult, envelope::Envelope,
};

pub(crate) struct LowPassDispatcher {}

impl CommandDispatch for LowPassDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;

        match envelope.command {
            DspCommand::LowPass { name, cutoff } => self.internal_dispatch(name, cutoff, state),
            _ => Err("err".to_string()),
        }
    }
}

impl LowPassDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to low_pass on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.low_pass_mut(cutoff);
        Ok(DspCommandResult {
            output: format!("Normalize track {} succesful", name),
            should_exit:false
        })
    }
}
