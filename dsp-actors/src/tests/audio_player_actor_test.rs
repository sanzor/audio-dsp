use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use audiolib::audio_buffer::AudioBuffer;
use audiolib::Channels;
use dsp_domain::actors::player_command::PlayerCommand;
use dsp_domain::actors::player_state::AudioPlayerState;
use dsp_domain::track::Track;
use dsp_domain::track::TrackInfo;

use player::audio_sink::AudioSink;
use player::AudioFrame;
use tokio::sync::Mutex;

use crate::user_actor_test::utils::create_user_actor;
use crate::user_actor_test::utils::get_player_actor_state;
struct TestSink {
    pub written: Arc<Mutex<Vec<AudioFrame>>>,
}
impl AudioSink for TestSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: &'a AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let collection = &mut *self.written.try_lock().map_err(|e| e.to_string())?;
            collection.push(frame.clone());
            Ok(())
        })
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

#[tokio::test]
async fn test_can_write() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let audio_player_actor = create_user_actor(track, sink);
    let _ = audio_player_actor.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(10)).await;
    let val = &*written.lock().await;
    assert!(val.len() > 0);

    Ok(())
}

#[tokio::test]
async fn test_can_pause() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state, AudioPlayerState::Playing) == true);
    let _ = actor_ref.tell(PlayerCommand::Pause {}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(matches!(state_query_result.state, AudioPlayerState::Paused) == true);
    Ok(())
}

#[tokio::test]
async fn test_can_pause_and_resume() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state, AudioPlayerState::Playing) == true);
    let _ = actor_ref.tell(PlayerCommand::Pause {}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(matches!(state_query_result.state, AudioPlayerState::Paused) == true);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state, AudioPlayerState::Playing) == true);
    Ok(())
}

#[tokio::test]
async fn test_cursor_still_moves_after_pause() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let _ = actor_ref.tell(PlayerCommand::Pause {}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    let cursor_after_pause = state_query_result.cursor;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let _ = actor_ref.tell(PlayerCommand::Pause {}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    let cursor_after_second_pause = state_query_result.cursor;
    assert!(cursor_after_second_pause > cursor_after_pause);
    Ok(())
}

#[tokio::test]
async fn test_state_is_paused_at_end() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 2], Channels::Mono);
    let track_len = track.data.samples.len();
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    loop {
        tokio::time::sleep(Duration::from_millis(3)).await;
        let state_query_result = get_player_actor_state(&actor_ref).await?;
        if state_query_result.written == track_len && matches!(state_query_result.state,AudioPlayerState::Paused){
            // assert!(matches!(state_query_result.state, AudioPlayerState::Paused));
            // assert!(matches!(state_query_result.cursor, 0));
            break;
        }
    }
    let state_query_result = get_player_actor_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state,AudioPlayerState::Paused));
    Ok(())
}

#[tokio::test]
async fn test_can_seek_while_paused() -> Result<(), String> {
    let sample_count = 100;
    let seek_position = sample_count / 2;
    let track = make_track_from_samples(vec![1_f32; sample_count], Channels::Mono);
    let track_len = track.data.samples.len();
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref
        .tell(PlayerCommand::Seek {
            position: seek_position as u32,
        })
        .await;
    let state = get_player_actor_state(&actor_ref).await?;
    assert!(state.cursor == seek_position);
    assert!(matches!(state.state, AudioPlayerState::Paused));

    Ok(())
}

#[tokio::test]
async fn test_pauses_when_seek() -> Result<(), String> {
    let sample_count = 100;
    let seek_position = sample_count / 2;
    let track = make_track_from_samples(vec![1_f32; sample_count], Channels::Mono);
    let track_len = track.data.samples.len();
    let written = Arc::new(Mutex::new(vec![]));
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    let actor_ref = create_user_actor(track, sink);
    let _ = actor_ref.tell(PlayerCommand::Play {}).await;
    let _ = actor_ref
        .tell(PlayerCommand::Seek {
            position: seek_position as u32,
        })
        .await;
    let state = get_player_actor_state(&actor_ref).await?;
    assert!(state.cursor == seek_position);
    assert!(matches!(state.state, AudioPlayerState::Paused));
    Ok(())
}

fn create_sink(written: Arc<Mutex<Vec<Vec<f32>>>>) -> Box<dyn AudioSink> {
    let sink = Box::new(TestSink {
        written: Arc::clone(&written),
    });
    sink
}
