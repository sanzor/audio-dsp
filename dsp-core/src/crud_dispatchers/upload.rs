use async_trait::async_trait;
use audiolib::audio_parse;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

use std::{path::PathBuf, str::FromStr};

use crate::{command_dispatch::CommandDispatch, state::SharedState};
pub(crate) struct UploadDispatcher {}

#[async_trait]
impl CommandDispatch for UploadDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::Upload {
                user_name,
                track_name,
                filename,
            } => {
                self.internal_dispatch(user_name, track_name, filename, state)
                    .await
            }
            _ => Err("".to_owned()),
        }
    }
}

impl UploadDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        filename: Option<String>,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let track_name = track_name.ok_or_else(|| "Invalid name to upload track")?;
        let user_name = user_name.ok_or_else(|| "Invalid name to upload track")?;
        let track_ref = state.get_track_ref(&user_name, &track_name).await?;
        let file_path_str = filename.unwrap_or_else(|| track_name.clone());
        let path = PathBuf::from_str(&file_path_str).map_err(|err| err.to_string())?;
        let _ = audio_parse::write_wav_file(&track_ref.inner.data, &path)?;
        Ok(DspCommandResult {
            output: format!(
                "Upload file successfully: {}",
                path.to_str().ok_or("invalid path")?.to_string()
            ),
            should_exit: false,
        })
    }
}
