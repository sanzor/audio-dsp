use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};

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

    pub async fn process_command(&mut self, input: Message) -> Result<DspCommandResult, String> {
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

    fn get_dispatcher_name(&self, command: &Message) -> &'static str {
        match command {
            Message::Copy { .. } => "copy",
            Message::Exit { .. } => "exit",
            Message::HighPass { .. } => "high_pass",
            Message::LowPass { .. } => "low_pass",
            Message::Delete { .. } => "delete",
            Message::Info { .. } => "info",
            Message::Load { .. } => "load",
            Message::Ls { .. } => "ls",
            Message::Gain { .. } => "gain",
            Message::Normalize { .. } => "normalize",
            Message::Upload { .. } => "upload",
            Message::Play { .. } => "play",
            Message::Stop { .. } => "stop",
            Message::Pause { .. } => "pause",
            Message::RunScript { .. } => "run-script",
        }
    }
}
