use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

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
use crate::player_ref::AudioPlayerRef;
use crate::player_test::test_sink::TestSink;

#[rstest]
fn can_run_and_change_state() -> Result<(), String> {
    let id = "some_id";
    let player_throttle = 50;
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams { track: track };
    let (message_tx, message_rx) = channel::<PlayerMessage>();

    let player_handle = LocalPlayerRef {
        tx: message_tx,
        id: id.to_string(),
    };
    let written = Arc::new(Mutex::new(vec![]));
    let audio_sink = TestSink {
        written: written.clone(),
    };
    let receiver = Box::new(LocalReceiver {
        receiver: message_rx,
    });
    let mut player = Player::new(player_params, audio_sink, receiver, Some(player_throttle));
    let state_ref = player.state_ref();
    let thread_handle = thread::spawn(move || {
        let _ = player.run();
    });
    let _ = send_command(&player_handle, PlayerCommand::Play);
    thread::sleep(Duration::from_millis(player_throttle * 2));
    let state_val = state_ref.lock().unwrap().clone();
    assert!(state_val.frames_written > 0);
    let _ = send_command(&player_handle, PlayerCommand::Close);
    thread_handle.join().expect("Player thread panicked");
    Ok(())
}

#[rstest]
fn can_run_and_and_write_to_sink() -> Result<(), String> {
    let id = "some_id".to_string();
    let player_throttle = 50;
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let player_params = PlayerParams {
        track: track.clone(),
    };
    let (message_tx, message_rx) = channel::<PlayerMessage>();

    let player_handle = LocalPlayerRef {
        tx: message_tx,
        id: id,
    };
    let written = Arc::new(Mutex::new(vec![]));
    let audio_sink = TestSink {
        written: written.clone(),
    };
    let receiver = Box::new(LocalReceiver {
        receiver: message_rx,
    });
    let mut player = Player::new(player_params, audio_sink, receiver, Some(player_throttle));
    let state_ref = player.state_ref();
    let thread_handle = thread::spawn(move || {
        let _ = player.run();
    });
    let _ = send_command(&player_handle, PlayerCommand::Play);

    do_until(
        || state_ref.lock().unwrap().clone(),
        |state_val| {
            // let matches=matches!(state_val.current_state,PlayerStates::Playing);
            println!("Frames written: {:?}", state_val.frames_written);
            let finished_writing = state_val.frames_written == track.data.samples.len();
            finished_writing
        },
        player_throttle * 2,
    );

    let _ = send_command(&player_handle, PlayerCommand::Close);
    thread_handle.join().expect("Player thread panicked");
    Ok(())
}

#[rstest]
fn can_stop_after_frames_written() -> Result<(), String> {
    let id = "some_id".to_string();
    let player_throttle = 50;
    let track = make_track_from_samples(vec![1.0, 2.0, 3.0], Channels::Mono);
    let (message_tx, message_rx) = channel();
    let player_params = PlayerParams {
        track: track.clone(),
    };
    let written_stream = Arc::new(Mutex::new(vec![]));
    let stream_sink = TestSink {
        written: written_stream,
    };
    let message_receiver = Box::new(LocalReceiver {
        receiver: message_rx,
    });
    let mut player = Player::new(
        player_params,
        stream_sink,
        message_receiver,
        Some(player_throttle),
    );
    let state_ref = player.state_ref();
    let _thread_handle = thread::spawn(move || {
        let _ = player.run();
    });
    let player_handle = LocalPlayerRef {
        tx: message_tx,
        id: id,
    };
    let _ = send_command(&player_handle, PlayerCommand::Play);

    do_until(
        || state_ref.lock().unwrap().clone(),
        |state_val| {
            let written_all = state_val.frames_written == track.data.samples.len();
            let state_matches = matches!(state_val.current_state, PlayerStates::Stopped);
            written_all && state_matches
        },
        player_throttle * 2,
    );
    Ok(())
}

#[rstest]
fn can_pause() -> Result<(), String> {
    let id = "some_id".to_string();
    let player_throttle = 50;
    let track = make_track_from_samples(vec![1.0, 2.0], Channels::Mono);
    let (message_tx, message_rx) = channel();
    let player_params = PlayerParams {
        track: track.clone(),
    };
    let written_stream = Arc::new(Mutex::new(vec![]));
    let stream_sink = TestSink {
        written: written_stream,
    };
    let message_receiver = Box::new(LocalReceiver {
        receiver: message_rx,
    });
    let mut player = Player::new(
        player_params,
        stream_sink,
        message_receiver,
        Some(player_throttle),
    );
    let state_ref = player.state_ref();
    let _thread_handle = thread::spawn(move || {
        let _ = player.run();
    });
    let player_handle = LocalPlayerRef {
        tx: message_tx,
        id: id,
    };
    let _ = send_command(&player_handle, PlayerCommand::Play);
    let _ = send_command(&player_handle, PlayerCommand::Pause);
    do_until(
        || state_ref.lock().unwrap().clone(),
        |state_val| matches!(state_val.current_state, PlayerStates::Paused),
        player_throttle * 2,
    );
    Ok(())
}

// #[rstest]
// pub fn can_run_cpal()->Result<(),String>{
//     let host=cpal::default_host();
//     let device=host.default_output_device()?;
//     let devices=host.devices().map_err(|e|e.to_string())?;
//     device.build_output_stream(device.default_output_config(), data_callback, error_callback, timeout);
//     Ok(())
// }
fn do_until<T>(retriever: impl Fn() -> T, predicate: impl Fn(&T) -> bool, throttle: u64) -> T {
    let start = Instant::now();
    let timeout = Duration::from_secs(20);
    loop {
        let val = retriever();
        if predicate(&val) {
            return val;
        }
        if start.elapsed() > timeout {
            panic!("Timeout waiting for condition")
        }
        thread::sleep(Duration::from_millis(throttle));
    }
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

fn get_state(query_result: QueryResult) -> Result<PlayerState, String> {
    match query_result {
        QueryResult::State { state } => Ok(state),
        _ => Err("Not query state".to_string()),
    }
}

async fn send_command(sender: &impl AudioPlayerRef, command: PlayerCommand) -> Result<(), String> {
    sender
        .send_message(PlayerMessage::Command { command: command })
        .await
        .map_err(|e| format!("Failed to send command {:?}", e))
}

fn query_state(handle: &impl AudioPlayerRef) -> Result<PlayerState, String> {
    let (send_back_tx, rx) = channel();
    let _ = handle.send_message(PlayerMessage::Query {
        query: QueryMessage::GetState {
            to: Some(send_back_tx),
        },
    });
    let result = rx
        .recv()
        .map_err(|e| format!("Could not receive result with error :{:?}", e))?;
    result.and_then(get_state)
}
