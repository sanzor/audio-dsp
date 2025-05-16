use dsp_domain::{command::{DspCommand, CommandResult}, envelope::Envelope};

use crate::{command_dispatch::CommandDispatch, dispatchers_provider::DispatchersProvider, state::SharedState};

pub struct CommandProcessorConfig{
    pub(crate) state:Option<SharedState>
}
pub struct CommandProcessor {
    dispatch_provider: DispatchersProvider,
    state: SharedState,
}

impl CommandProcessor {
    pub(crate) fn new(dispatch_provider: DispatchersProvider, state: SharedState) -> CommandProcessor {
        CommandProcessor {
            dispatch_provider,
            state,
        }
    }

    pub fn process_command(&mut self, input: DspCommand) -> Result<CommandResult, String> {
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
            DspCommand::Gain { name, gain, mode } => "gain",
            DspCommand::Normalize { name: _, mode } => "normalize",
            DspCommand::Upload { name, filename } => "upload",
        }
    }
}
