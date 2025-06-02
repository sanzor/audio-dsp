use dsp_core::{api::create_command_processor, command_processor::CommandProcessor};
use dsp_domain::{dsp_message_result::DspMessageResult, dsp_message::DspMessage};

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

    pub async fn process(&mut self, input: &str) -> Result<DspMessageResult, String> {
        let command = self.command_parser.parse_command(input)?;
        if let DspMessage::Exit { user_name } = command {
            return Ok(DspMessageResult {
                output: "exit".to_string(),
                should_exit: true,
            });
        }
        let result = self.command_processor.process_command(command).await;
        result
    }

    pub async fn process_command(&mut self, command: DspMessage) -> Result<DspMessageResult, String> {
        let result = self.command_processor.process_command(command).await;
        result
    }
}
