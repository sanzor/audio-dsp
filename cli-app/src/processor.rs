use std::{collections::HashMap, sync::Arc};

use actix::Addr;
use dsp_actors::audio_player_actor::audio_player_actor::AudioPlayerActor;
use dsp_actors::user_actor::user_actor::UserActor;

use dsp_core::{
    api::create_command_processor,
    command_processor::CommandProcessor,
    state::{create_state, SharedState},
};
use dsp_domain::{
    audio_player_message::AudioPlayerMessage,
    audio_player_message_result::AudioPlayerMessageResult, dsp_message::DspMessage, tracks_message_result::TracksMessageResult,
};
use tokio::sync::Mutex;

use crate::command_parser::*;

pub struct Processor {
    command_processor: CommandProcessor,
    command_parser: CommandParser,
    state: Arc<Mutex<SharedState>>,
    players:Arc<Mutex<HashMap<String,Addr<AudioPlayerActor>>>>
}

impl Processor {
    pub fn create_processor() -> Processor {
        Processor::new(create_command_processor(), CommandParser {})
    }
    fn new(command_processor: CommandProcessor, command_parser: CommandParser) -> Processor {
        Processor {
            command_processor: command_processor,
            command_parser: command_parser,
            state: create_state(),
            players:Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn process_crud_command(&mut self, input: &str) -> Result<TracksMessageResult, String> {
        let command: DspMessage = self.command_parser.parse_crud_command(input)?;
        if let DspMessage::Exit { user_name } = command {
            return Ok(TracksMessageResult {
                output: "exit".to_string(),
                should_exit: true,
            });
        }
        let result = self
            .command_processor
            .process_crud_command(command, Arc::clone(&self.state))
            .await;
        result
    }

    pub async fn process_player_command(
        &mut self,
        input: &str,
    ) -> Result<AudioPlayerMessageResult, String> {
        let command: AudioPlayerMessage = self.command_parser.parse_player_command(input)?;
        let tracks=Arc::clone(&self.state);
        let players=Arc::clone(&self.players);
        match command{
            AudioPlayerMessage::Play { track_id }=>
                UserActor::handle_play(track_id, Arc::clone(&tracks), Arc::clone(&players)).await,
            AudioPlayerMessage::Pause { track_id }=>
                UserActor::handle_pause(track_id, Arc::clone(&tracks), Arc::clone(&players)).await,
            AudioPlayerMessage::Stop { track_id }=>Ok(AudioPlayerMessageResult {
                output: "exit".to_string(),
                should_exit: true,
            })
        }

    }

    pub async fn process_tracks_command(&mut self, command: DspMessage) -> Result<TracksMessageResult, String> {
        let result = self
            .command_processor
            .process_crud_command(command, Arc::clone(&self.state))
            .await;
        result
    }
}
