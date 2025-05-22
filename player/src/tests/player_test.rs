use audiolib::audio_buffer::AudioBuffer;
use audiolib::Channels;
use dsp_domain::track::Track;
use dsp_domain::track::TrackInfo;
use rstest::rstest;

use crate::player_command::PlayerCommand;
use crate::player_core::player_state::PlayerState;
use crate::player_core::Player;
use crate::player_params::PlayerParams;
use crate::player_test::test_receiver::TestReceiver;
use crate::player_test::test_sink::TestSink;

#[rstest]

fn can_run_player() {
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams { track: track };

    let audio_sink = TestSink { written: vec![] };
    let receiver = TestReceiver::new(vec![PlayerCommand::Play, PlayerCommand::Stop]);
    let mut player = Player::new(player_params, audio_sink, Box::new(receiver));
    let result = player.run();
    assert!(result.is_ok());
    assert_eq!(player.cursor_position(), 0);
    assert_eq!(player.player_state(), PlayerState::Stopped);
}


#[rstest]

fn can_run_and_write_frame() {
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams { track: track };

    let audio_sink = TestSink { written: vec![] };
    let receiver = TestReceiver::new(vec![PlayerCommand::Play]);
    let mut player = Player::new(player_params, audio_sink, Box::new(receiver));
    let result = player.run();
    assert!(result.is_ok());
    assert_eq!(player.cursor_position(), 0);
    assert_eq!(player.player_state(), PlayerState::Stopped);
}

fn make_track_from_samples(samples: Vec<f32>, channels: Channels) -> Track {
    match channels {
        Channels::Mono => Track {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                channels: Channels::Mono,
                sample_rate: 1_f32,
                samples: samples.clone(),
            },
        },
        Channels::Stereo => Track {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                samples: samples.clone(),
                sample_rate: 1_f32,
                channels: Channels::Stereo,
            },
        },
    }
}
