use std::collections::HashMap;

use actors::user_actor::user_actor::UserActor;

use domain::actors::user_player_command::UserPlayerCommand;
use domain::actors::user_player_command_result::UserPlayerCommandResult;
use domain::dsp_message::DspMessage;
use domain::tracks_message_result::TracksMessageResult;
use dsp_core::{
    api::create_command_processor, command_processor::CommandProcessor, state::TrackState,
};
use kameo::actor::spawn;
use kameo::actor::ActorRef;

use crate::command_parser::*;

pub struct Processor {
    user_actor: ActorRef<UserActor>,
    command_parser: CommandParser,
}

impl Processor {
    pub fn create_processor() -> Processor {
        Processor::new(create_command_processor(), CommandParser {})
    }
    fn new(command_processor: CommandProcessor, command_parser: CommandParser) -> Processor {
        Processor {
            user_actor: spawn(UserActor::new(
                command_processor,
                TrackState::new(),
                HashMap::new(),
            )),
            command_parser: command_parser,
        }
    }

    pub async fn process_crud_command(
        &mut self,
        input: &str,
    ) -> Result<TracksMessageResult, String> {
        let command: DspMessage = self.command_parser.parse_crud_command(input)?;
        if let DspMessage::Exit { user_name } = command {
            return Ok(TracksMessageResult {
                output: "exit".to_string(),
                should_exit: true,
            });
        }
        let result = self
            .user_actor
            .ask(command)
            .await
            .map_err(|e| e.to_string());
        result
    }

    pub async fn process_player_command(
        &mut self,
        input: &str,
    ) -> Result<UserPlayerCommandResult, String> {
        let command: UserPlayerCommand = self.command_parser.parse_player_command(input)?;
        let result = self
            .user_actor
            .ask(command)
            .await
            .map_err(|e| e.to_string());
        result
    }

    pub async fn process_tracks_command(
        &mut self,
        command: DspMessage,
    ) -> Result<TracksMessageResult, String> {
        let result = self
            .user_actor
            .ask(command)
            .await
            .map_err(|e| e.to_string());
        result
    }
}
