use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{
    dsp_message_result::DspMessageResult, envelope::Envelope, dsp_message::DspMessage, user,
};

pub(crate) struct GainDispatcher {}

#[async_trait]
impl CommandDispatch for GainDispatcher {
    async fn dispatch_mut(
        &self,
        envelope: Envelope,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Gain {
                user_name,
                track_name,
                gain,
                mode: _,
                parallelism: _,
            } => {
                self.internal_dispatch(user_name, track_name, gain, state)
                    .await
            }
            _ => Err("err".to_string()),
        }
    }
}

impl GainDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        cutoff: f32,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        let user_name = user_name.ok_or("Invalid name for user to perform gain on")?;
        let track_name = track_name.ok_or("Invalid name for track to perform gain on")?;

        let track_ref = state.get_track_ref_mut(&user_name, &track_name).await?;
        let _ = track_ref.inner.data.gain_mut(cutoff);
        Ok(DspMessageResult {
            output: format!("Updated gain for track {} succesful", track_name),
            should_exit: false,
        })
    }
}
