use std::path::PathBuf;

use dsp_domain::{
    dsp_command::DspCommand,
    dsp_command_result::DspCommandResult,
    envelope::Envelope,
    track::{Track, TrackInfo},
};

use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};

pub(crate) struct LoadDispatcher {}

impl CommandDispatch for LoadDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;

        match envelope.command {
            DspCommand::Load { name, filename } => self.internal_dispatch(name, filename, state),
            _ => Err("".to_owned()),
        }
    }
}

impl LoadDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        filename: Option<String>,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let filename = filename.ok_or_else(|| "Invalid file name".to_string())?;
        let filepath = PathBuf::from(&filename);
        let name = name.unwrap_or_else(|| filename.clone());

        let audio_buffer = audiolib::audio_parse::read_wav_file(&filepath)?;
        let new_track = Track {
            info: TrackInfo { name: name.clone() },
            data: audio_buffer,
        };

        state.upsert_track(new_track)?;

        Ok(DspCommandResult {
            output: format!("Loaded track '{}' from '{}'", name, filepath.display()),
            should_exit: false,
        })
    }
}
