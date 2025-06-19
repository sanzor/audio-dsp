
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use audiolib::audio_buffer::AudioBuffer;
use audiolib::Channels;
use dsp_domain::track::Track;
use dsp_domain::track::TrackInfo;
use kameo::actor::spawn;
use kameo::actor::ActorRef;
use player::audio_sink::AudioSink;
use player::AudioFrame;
use tokio::sync::Mutex;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;
use crate::audio_player_actor::audio_player_actor::AudioPlayerState;
use crate::audio_player_actor::audio_player_actor_params::AudioPlayerActorParams;
use crate::audio_player_actor::state_reply::StateReply;
use crate::audio_player_actor::state_request::StateRequest;
use crate::AudioPlayerMessage;
struct TestSink{
     pub written: Arc<Mutex<Vec<AudioFrame>>>,
}
impl AudioSink for TestSink{
    fn write_frame<'a>(&'a mut self,frame: &'a AudioFrame,) -> Pin<Box<dyn Future<Output = Result<(),String> > +Send+'a> >  {
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
async fn test_can_write()->Result<(),String>{
    let track= make_track_from_samples(vec![1_f32;500], Channels::Mono);
    let written=Arc::new(Mutex::new(vec![]));
    let sink=Box::new(TestSink{written:Arc::clone(&written)});
    let audio_player_actor=create_actor(track,sink);
    let _=audio_player_actor.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(10)).await;
    let val=& *written.lock().await;
    assert!(val.len()>0);
    
    Ok(())
}

#[tokio::test]
async fn test_can_pause()->Result<(),String>{
    let track= make_track_from_samples(vec![1_f32;500], Channels::Mono);
    let written=Arc::new(Mutex::new(vec![]));
    let sink=Box::new(TestSink{written:Arc::clone(&written)});
    let actor_ref=create_actor(track,sink);
    let _=actor_ref.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state,AudioPlayerState::Playing)==true);
    let _=actor_ref.tell(AudioPlayerMessage::Pause{}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(matches!(state_query_result.state,AudioPlayerState::Paused)==true);
    Ok(())
}

#[tokio::test]
async fn test_can_pause_and_resume()->Result<(),String>{
    let track= make_track_from_samples(vec![1_f32;500], Channels::Mono);
    let written=Arc::new(Mutex::new(vec![]));
    let sink=Box::new(TestSink{written:Arc::clone(&written)});
    let actor_ref=create_actor(track,sink);
    let _=actor_ref.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state,AudioPlayerState::Playing)==true);
    let _=actor_ref.tell(AudioPlayerMessage::Pause{}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(matches!(state_query_result.state,AudioPlayerState::Paused)==true);
    let _=actor_ref.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    assert!(matches!(state_query_result.state,AudioPlayerState::Playing)==true);
    Ok(())
}

#[tokio::test]
async fn test_cursor_still_moves_after_pause()->Result<(),String>{
    let track= make_track_from_samples(vec![1_f32;500], Channels::Mono);
    let written=Arc::new(Mutex::new(vec![]));
    let sink=Box::new(TestSink{written:Arc::clone(&written)});
    let actor_ref=create_actor(track,sink);
    let _=actor_ref.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let _=actor_ref.tell(AudioPlayerMessage::Pause{}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    let cursor_after_pause=state_query_result.cursor;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let _=actor_ref.tell(AudioPlayerMessage::Play{}).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let _=actor_ref.tell(AudioPlayerMessage::Pause{}).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let state_query_result=get_state(&actor_ref).await?;
    let cursor_after_second_pause=state_query_result.cursor;
    assert!(cursor_after_second_pause>cursor_after_pause);
    Ok(())
}

fn create_actor(track:Track,sink:Box<dyn AudioSink+Send+Sync+'static>) -> ActorRef<AudioPlayerActor> {
    let audio_player_actor_params=AudioPlayerActorParams{sink:sink,track:track,cursor:0};
    let audio_player_actor = spawn(AudioPlayerActor::new(audio_player_actor_params));
     
    let g = kameo::registry::ActorRegistry::new();

    audio_player_actor
}
fn create_sink(written:Arc<Mutex<Vec<Vec<f32>>>>)->Box<dyn AudioSink>{
    let sink=Box::new(TestSink{written:Arc::clone(&written)});
    sink
}

async fn get_state(actor_ref:&ActorRef<AudioPlayerActor>)->Result<StateReply,String>{
    let state_query_result=actor_ref.ask(StateRequest{}).await.map_err(|e| e.to_string());
    state_query_result
}