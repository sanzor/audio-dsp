use std::thread;

use dsp_domain::track::Track;

use crate::{
    audio_sink::AudioSink, command_receiver::CommandReceiver, player_params::PlayerParams,
    player_ref::AudioPlayerRef,
};

use super::Player;

pub fn spawn_player(
    track: Track,
    sink: impl AudioSink + Send + Sync + 'static,
    f: impl Fn() -> (Box<dyn AudioPlayerRef + Send>, Box<dyn CommandReceiver + Send>),
) -> Box<dyn AudioPlayerRef> {
    let (tx, rx) = f();

    let _handle = thread::spawn(move || {
        let mut player = Player::new(PlayerParams { track: track }, sink, rx, None);
        player.run();
    });
    tx
}
