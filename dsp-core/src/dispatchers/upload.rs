use audiolib::audio_parse;
use dsp_domain::{
    dsp_command::DspCommand, dsp_command_result::DspCommandResult, envelope::Envelope,
};

use std::{path::PathBuf, str::FromStr};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
pub(crate) struct UploadDispatcher {}

impl UploadDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        filename: Option<String>,
        state: &State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name to upload track")?;
        let track_ref = state
            .get_track_ref(&name)
            .ok_or_else(|| "Could not find track_ref")?;
        let file_path_str = filename.unwrap_or_else(|| name.clone());
        let path = PathBuf::from_str(&file_path_str).map_err(|err| err.to_string())?;
        let _ = audio_parse::write_wav_file(&track_ref.inner.data, &path)?;
        Ok(DspCommandResult {
            output: format!(
                "Upload file successfully: {}",
                path.to_str().ok_or("invalid path")?.to_string()
            ),
        })
    }
}

impl CommandDispatch for UploadDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;

        match envelope.command {
            DspCommand::Upload { name, filename } => self.internal_dispatch(name, filename, state),
            _ => Err("".to_owned()),
        }
    }
}
