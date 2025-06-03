use crate::{
    command_processor::{CommandProcessor, CommandProcessorConfig},
    dispatchers_provider::DispatchersProvider,
};

pub fn create_command_processor() -> CommandProcessor {
    CommandProcessor::new(DispatchersProvider::new())
}

pub fn create_command_processor_with_config(config: CommandProcessorConfig) -> CommandProcessor {
    let dispatchers = DispatchersProvider::new();
    let cmd_processor = CommandProcessor::new(dispatchers);
    return cmd_processor;
}
