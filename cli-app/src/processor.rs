use std::collections::HashMap;
use std::sync::Arc;

use actors::user_actor::create_user_data::CreateUserData;
use actors::user_actor::user_actor::UserActor;

use crate::command_parser::*;
use actors::user_actor::create_user_actor_params::CreateUserActorParams;
use domain::actors::user_player_command::UserPlayerCommand;
use domain::actors::user_player_command_result::UserPlayerCommandResult;
use domain::dsp_message::DspMessage;
use domain::tracks_message_result::TracksMessageResult;
use dsp_core::{api::create_command_processor, command_processor::CommandProcessor, state::Tracks};
use kameo::actor::spawn;
use kameo::actor::ActorRef;

pub struct Processor {
    user_actor: ActorRef<UserActor>,
    command_parser: CommandParser,
}

impl Processor {
    pub fn create_processor() -> Processor {
        Processor::new(Arc::new(create_command_processor()), CommandParser {})
    }
    fn new(command_processor: Arc<CommandProcessor>, command_parser: CommandParser) -> Processor {
        let dummy_id = "someid".to_string();
        Processor {
            user_actor: spawn(UserActor::new(CreateUserActorParams {
                user_data: CreateUserData {
                    email: dummy_id.clone(),
                    id: dummy_id.clone(),
                    name: dummy_id,
                },
                processor: command_processor,
                tracks: Tracks::new(),
                players: HashMap::new(),
            })),

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
