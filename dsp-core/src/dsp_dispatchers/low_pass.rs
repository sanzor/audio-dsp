use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

pub(crate) struct LowPassDispatcher {}

#[async_trait]
impl CommandDispatch for LowPassDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::LowPass { track_name, cutoff } => self.internal_dispatch(name, cutoff, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl LowPassDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        cutoff: f32,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to low_pass on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .await
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.low_pass_mut(cutoff);
        Ok(DspCommandResult {
            output: format!("Normalize track {} succesful", name),
            should_exit: false,
        })
    }
}
