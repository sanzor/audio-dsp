use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};
pub(crate) struct HighPassDispatcher {}

#[async_trait]
impl CommandDispatch for HighPassDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::HighPass { track_name, cutoff } => self.internal_dispatch(name, cutoff, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl HighPassDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to high_pass on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .await
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.high_pass_mut(cutoff);
        Ok(DspCommandResult {
            output: format!("Normalize track {} succesful", name),
            should_exit: false,
        })
    }
}
