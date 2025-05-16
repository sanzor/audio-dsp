use crate::{
    command_processor::{CommandProcessor, CommandProcessorConfig},
    dispatchers_provider::DispatchersProvider,
    state::create_shared_state,
};

pub fn create_command_processor() -> CommandProcessor {
    CommandProcessor::new(DispatchersProvider::new(), create_shared_state())
}

pub fn create_command_processor_with_config(config: CommandProcessorConfig) -> CommandProcessor {
    let state = config.state.unwrap_or_else(|| create_shared_state());
    let dispatchers = DispatchersProvider::new();
    let cmd_processor = CommandProcessor::new(dispatchers, state);
    return cmd_processor;
}
