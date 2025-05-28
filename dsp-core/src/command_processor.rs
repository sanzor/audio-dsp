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

    pub fn process_command(&mut self, input: Message) -> Result<DspCommandResult, String> {
        let dispatcher_name = self.get_dispatcher_name(&input);
        let dispatcher = self
            .dispatch_provider
            .get_dispatcher_by_name(dispatcher_name)
            .ok_or_else(|| "Could not find dispatcher".to_string())?;
        let result = dispatcher.dispatch(
            Envelope {
                command: input,
                from: None,
            },
            self.state.clone(),
        );
        result
    }

    fn get_dispatcher_name(&self, command: &Message) -> &'static str {
        match command {
            Message::Copy {
                name: _,
                copy_name: _,
            } => "copy",
            Message::Exit => "exit",
            Message::HighPass { name: _, cutoff: _ } => "high_pass",
            Message::LowPass { name, cutoff } => "low_pass",
            Message::Delete { name } => "delete",
            Message::Info { name } => "info",
            Message::Load { name, filename } => "load",
            Message::Ls => "ls",
            Message::Gain {
                name: _,
                gain: _,
                mode: _,
                parallelism,
            } => "gain",
            Message::Normalize {
                name: _,
                mode,
                parallelism,
            } => "normalize",
            Message::Upload { name, filename } => "upload",
            Message::Play { name } => "play",
            Message::Stop { id }=>"stop",
            Message::Pause { id }=>"pause"
            Message::RunScript { file } => "run-script",
        }
    }
}
