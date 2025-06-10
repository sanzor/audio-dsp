use crate::audio_player_message_result::AudioPlayerMessageResult;
use actix::Message;
use serde::{Deserialize, Serialize};
#[derive(clap::Subcommand, Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<AudioPlayerMessageResult, String>")]
pub enum AudioPlayerMessage {
    Play { track_id: Option<String> },
    Stop { track_id: Option<String> },
    Pause { track_id: Option<String> },
}
