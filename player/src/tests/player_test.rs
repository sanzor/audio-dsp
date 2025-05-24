use std::io::sink;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use audiolib::audio_buffer::AudioBuffer;
use audiolib::Channels;
use dsp_domain::track::Track;
use dsp_domain::track::TrackInfo;
use rstest::rstest;

use crate::command_receiver::LocalReceiver;
use crate::player_command::PlayerCommand;
use crate::player_command::PlayerMessage;
use crate::player_command::QueryMessage;
use crate::player_command::QueryResult;
use crate::player_core::player_state::PlayerState;
use crate::player_core::player_states::PlayerStates;
use crate::player_core::Player;
use crate::player_params::PlayerParams;
use crate::player_ref::local_player_ref::LocalPlayerRef;
use crate::player_ref::PlayerRef;
use crate::player_test::test_receiver::TestReceiver;
use crate::player_test::test_sink::TestConcurrentSink;
use crate::player_test::test_sink::TestSink;

#[rstest]
fn can_run_player()->Result<(),String> {
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams { track: track };

    let audio_sink = TestSink { written: vec![] };
    let receiver = TestReceiver::new(vec![
        PlayerMessage::Command {
            command: PlayerCommand::Play,
        },
        PlayerMessage::Command {
            command: PlayerCommand::Stop,
        },
    ]);
    let mut player = Player::new(player_params, audio_sink, Box::new(receiver));
    let result = player.run();
    assert!(result.is_ok());
    let state= get_state(player.query(QueryMessage::GetState { to: None })?)?;

    assert_eq!(state.cursor, 0);
    assert_eq!(state.current_state, PlayerStates::Stopped);
    Ok(())
}

  
#[rstest]
fn can_run_and_write_frame()->Result<(),String> {
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams { track: track };
    let (query_tx, query_rx) = channel();
    let (command_tx, _) = channel();
    let player_handle=LocalPlayerRef{tx:command_tx};
    let written = Arc::new(Mutex::new(vec![]));
    let audio_sink = TestConcurrentSink {
        written: written.clone(),
    };
    let receiver = Box::new(LocalReceiver { receiver: sink_rx });
    let mut player = Player::new(player_params, audio_sink, receiver);
    let result = thread::spawn(move || {
        player.run();
    });
    let state= get_state(player.query(QueryMessage::GetState { to: None })?)?;

    assert_eq!(state.cursor, 0);
    assert_eq!(state.current_state, PlayerStates::Stopped);
     Ok(())
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

fn get_state(query_result:QueryResult)->Result<PlayerState,String>{
    match query_result{
        QueryResult::State { state:state }=>Ok(state),
        _ =>Err("Not query state".to_string())
    }
}

fn query_state(handle:impl PlayerRef)->Result<PlayerState,String>{
    let (tx,rx)=channel();
    handle.send_message(PlayerMessage::Query { query: QueryMessage::GetState { to: Some(tx) }});
    let result=rx.try_recv().map_err(|e|e.to_string())?;
    result.and_then(get_state)
}
