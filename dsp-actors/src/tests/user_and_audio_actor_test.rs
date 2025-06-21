use std::collections::HashMap;

use audiolib::{audio_buffer::AudioBuffer, Channels};
use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use dsp_domain::{audio_player_message::AudioPlayerMessage, dsp_message::DspMessage, track::{Track, TrackInfo}};
use kameo::{actor::ActorRef, spawn};

use crate::{audio_player_actor::{audio_player_actor::AudioPlayerState, state_reply::AudioPlayerActorStateResult}, user_actor::user_actor::UserActor};

fn create_user_actor() -> ActorRef<UserActor> {
    let processor = CommandProcessor::create_processor();
    let tracks = TracksState::new();
    let players = HashMap::new();
    let actor = spawn(UserActor::new(processor, tracks, players));
    let g = kameo::registry::ActorRegistry::new();

    actor
}

#[tokio::test]
async fn can_create_player_and_play() -> Result<(), String> {
    let track_name="some_track";
    let track=sample_track(track_name);
    let user_name="my_user".to_string();
    let user_actor=create_user_actor();
    let insert_result=user_actor
            .ask(DspMessage::Insert { user_name: Some(user_name), track_payload: Some(to_string(&track)) })
            .await.map_err(|e|e.to_string());
    let play=user_actor
            .tell( AudioPlayerMessage::Play { track_id: Some(track_name.to_string()) }).await;
    let state=get_state(&user_actor, track_name).await?;
    assert!(matches!(state.state,AudioPlayerState::Playing));

    
    Ok(())
}

fn sample_track(track_name:&str)->Track{
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = Track {
        info: TrackInfo { name: track_name.to_string() },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    track
}
fn to_string(track:&Track)->String{
    serde_json::to_string(track).unwrap()
}

async fn get_state(user_actor:&ActorRef<UserActor>,track_id:&str)->Result<AudioPlayerActorStateResult,String>{
    let rez=user_actor.ask(AudioPlayerMessage::State { track_id: Some(track_id.to_string()) }).await.map_err(|e|e.to_string())?;
    let state:AudioPlayerActorStateResult=serde_json::from_str(&rez.output).unwrap();
    Ok(state)
}