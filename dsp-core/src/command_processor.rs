use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope, dsp_message::DspMessage};

use crate::{
    command_dispatch::CommandDispatch,
    dispatchers_provider::DispatchersProvider,
    state::{create_shared_state, SharedState},
};

pub struct CommandProcessorConfig {
    pub(crate) state: Option<SharedState>,
}
pub struct CommandProcessor {
    dispatch_provider: DispatchersProvider,
    state: SharedState,
}

impl CommandProcessor {
    pub fn create_processor() -> CommandProcessor {
        CommandProcessor {
            dispatch_provider: DispatchersProvider::new(),
            state: create_shared_state(),
        }
    }
    pub(crate) fn new(
        dispatch_provider: DispatchersProvider,
        state: SharedState,
    ) -> CommandProcessor {
        CommandProcessor {
            dispatch_provider,
            state,
        }
    }

        pub async fn process_command(&mut self, input: DspMessage) -> Result<DspMessageResult, String> {
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
                    self.state.clone(),
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
            DspMessage::Play { .. } => "play",
            DspMessage::Stop { .. } => "stop",
            DspMessage::Pause { .. } => "pause",
            DspMessage::RunScript { .. } => "run-script",
        }
    }
}
