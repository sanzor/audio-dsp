pub mod audio_player_actor;
pub mod user_actor;

use actix::Message;

#[derive(Debug, Message)]
#[rtype(result = "Result<AudioPlayerResult,String>")]
pub enum AudioPlayerMessage {
    Play,
    Pause,
    Seek { position: u32 },
    Stop,
}
pub struct AudioPlayerResult {}
