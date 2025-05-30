use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

pub(crate) struct GainDispatcher {}

#[async_trait]
impl CommandDispatch for GainDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::Gain {
                track_name,
                gain,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(name, gain, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl GainDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to perform gain on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .await
            .ok_or("Could not find track ref")?;
        let _ = track_ref.inner.data.gain_mut(cutoff);
        Ok(DspCommandResult {
            output: format!("Updated gain for track {} succesful", name),
            should_exit: false,
        })
    }
}
