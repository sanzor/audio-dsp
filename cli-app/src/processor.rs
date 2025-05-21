use dsp_core::{api::create_command_processor, command_processor::CommandProcessor};
use dsp_domain::{dsp_command::DspCommand, dsp_command_result::DspCommandResult};

use crate::command_parser::*;

pub struct Processor {
    command_processor: CommandProcessor,
    command_parser: CommandParser,
}

impl Processor {
    pub fn create_processor() -> Processor {
        Processor::new(create_command_processor(), CommandParser {})
    }
    fn new(command_processor: CommandProcessor, command_parser: CommandParser) -> Processor {
        Processor {
            command_processor: command_processor,
            command_parser: command_parser,
        }
    }

    pub fn process(&mut self, input: &str) -> Result<DspCommandResult, String> {
        let command = self.command_parser.parse_command(input)?;
        if let DspCommand::Exit = command {
            return Ok(DspCommandResult {
                output: "exit".to_string(),
                should_exit: true,
            });
        }
        let result = self.command_processor.process_command(command);
        result
    }

    pub fn process_command(&mut self, command: DspCommand) -> Result<DspCommandResult, String> {
        let result = self.command_processor.process_command(command);
        result
    }
}
