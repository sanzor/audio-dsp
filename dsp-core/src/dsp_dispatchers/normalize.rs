use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

pub(crate) struct NormalizeDispatcher {}

#[async_trait]
impl CommandDispatch for NormalizeDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::Normalize {
                track_name,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(name, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl NormalizeDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or("Invalid name for track to perform normalize on")?;
        let track_ref = state
            .get_track_ref_mut(&name)
            .await
            .ok_or_else(|| "Could not find track ref")?;
        let _ = track_ref.inner.data.normalize_mut();
        Ok(DspCommandResult {
            output: format!("Normalize track {} succesful", name),
            should_exit: false,
        })
    }
}
