use dsp_domain::{dsp_command::DspCommand, dsp_command_result::DspCommandResult, envelope::Envelope};

use crate::{
    command_dispatch::CommandDispatch, dispatchers_provider::DispatchersProvider,
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
    pub fn create_processor()->CommandProcessor{
        CommandProcessor { dispatch_provider:  DispatchersProvider::new() , state: create_shared_state() }
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

    pub fn process_command(&mut self, input: DspCommand) -> Result<DspCommandResult, String> {
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

    fn get_dispatcher_name(&self, command: &DspCommand) -> &'static str {
        match command {
            DspCommand::Copy { name, copy_name } => "copy",
            DspCommand::Exit => "exit",
            DspCommand::HighPass { name, cutoff } => "high_pass",
            DspCommand::LowPass { name, cutoff } => "low_pass",
            DspCommand::Delete { name } => "delete",
            DspCommand::Info { name } => "info",
            DspCommand::Load { name, filename } => "load",
            DspCommand::Ls => "ls",
            DspCommand::Gain { name, gain, mode ,parallelism} => "gain",
            DspCommand::Normalize { name: _, mode,parallelism } => "normalize",
            DspCommand::Upload { name, filename } => "upload",
            DspCommand::Play { name } => "play",
        }
    }
}
