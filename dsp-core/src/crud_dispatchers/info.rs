use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope, dsp_message::DspMessage};

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub(crate) struct InfoDispatcher {}

#[async_trait]
impl CommandDispatch for InfoDispatcher {
    async fn dispatch_mut(
        &self,
        envelope: Envelope,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Info {
                track_name,
                user_name,
            } => self.internal_dispatch(user_name, track_name, state).await,
            _ => Err("".to_owned()),
        }
    }


}

impl InfoDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        let track_name = track_name.ok_or_else(|| "Invalid name")?;
        let user_name = user_name.ok_or_else(|| "Invalid user name")?;
        let track_info = state
            .get_track_info(&user_name, &track_name)
            .await
            .map_err(|e| format!("Could not find track info for {}", e))?;
        Ok(DspMessageResult {
            output: serde_json::to_string_pretty(&track_info).unwrap(),
            should_exit: false,
        })
    }
}
