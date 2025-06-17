pub mod audio_player_actor;
pub mod user_actor;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub mod user_actor_test;

#[derive(Debug)]
pub enum AudioPlayerMessage {
    Play,
    Pause,
    Seek { position: u32 },
    Stop,
}
pub struct AudioPlayerResult {}
