pub mod audio_player_actor;
pub mod user_actor;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub mod user_actor_test;
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
