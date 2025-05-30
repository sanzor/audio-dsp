use std::path::PathBuf;

use async_trait::async_trait;
use dsp_domain::{
    dsp_command_result::DspCommandResult,
    envelope::Envelope,
    message::Message,
    track::{Track, TrackInfo},
};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct LoadDispatcher {}
#[async_trait]
impl CommandDispatch for LoadDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::Load { track_name, filename } => self.internal_dispatch(name, filename, state).await,
            _ => Err("".to_owned()),
        }
    }
}

impl LoadDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        filename: Option<String>,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let filename = filename.ok_or_else(|| "Invalid file name".to_string())?;
        let filepath = PathBuf::from(&filename);
        let name = name.unwrap_or_else(|| filename.clone());

        let audio_buffer = audiolib::audio_parse::read_wav_file(&filepath)?;
        let new_track = Track {
            info: TrackInfo { name: name.clone() },
            data: audio_buffer,
        };

        state.upsert_track(new_track).await?;

        Ok(DspCommandResult {
            output: format!("Loaded track '{}' from '{}'", name, filepath.display()),
            should_exit: false,
        })
    }
}
