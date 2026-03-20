pub mod audio_player_actor;
pub mod region_sets;
pub mod tracks;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub mod user_actor_test;

#[derive(Debug)]
pub enum AudioPlayerInternalMessage {
    Play,
    Pause,
    Seek { position: u32 },
    Stop,
}
pub struct AudioPlayerResult {}
