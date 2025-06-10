use dsp_domain::{audio_player_message::AudioPlayerMessage, dsp_message::DspMessage, envelope::Envelope, tracks_message_result::TracksMessageResult};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    command_dispatch::CommandDispatch, dispatchers_provider::DispatchersProvider,
    state::SharedState,
};

pub struct CommandProcessorConfig {
    pub(crate) state: Option<SharedState>,
}
pub struct CommandProcessor {
    dispatch_provider: DispatchersProvider,
}

impl CommandProcessor {
    pub fn create_processor() -> CommandProcessor {
        CommandProcessor {
            dispatch_provider: DispatchersProvider::new(),
        }
    }
    pub(crate) fn new(dispatch_provider: DispatchersProvider) -> CommandProcessor {
        CommandProcessor { dispatch_provider }
    }

    pub async fn process_crud_command(
        &self,
        input: DspMessage,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        let dispatcher_name = self.get_dispatcher_name(&input);
        let dispatcher = self
            .dispatch_provider
            .get_dispatcher_by_name(dispatcher_name)
            .ok_or_else(|| "Could not find dispatcher".to_string())?;
        let result = dispatcher
            .dispatch(
                Envelope {
                    command: input,
                    from: None,
                },
                state,
            )
            .await;
        result
    }



    fn get_dispatcher_name(&self, command: &DspMessage) -> &'static str {
        match command {
            DspMessage::Copy { .. } => "copy",
            DspMessage::Exit { .. } => "exit",
            DspMessage::HighPass { .. } => "high_pass",
            DspMessage::LowPass { .. } => "low_pass",
            DspMessage::Delete { .. } => "delete",
            DspMessage::Info { .. } => "info",
            DspMessage::Load { .. } => "load",
            DspMessage::Ls { .. } => "ls",
            DspMessage::Gain { .. } => "gain",
            DspMessage::Normalize { .. } => "normalize",
            DspMessage::Upload { .. } => "upload",
            DspMessage::RunScript { .. } => "run-script",
        }
    }
}
