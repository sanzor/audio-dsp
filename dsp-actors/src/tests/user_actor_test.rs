use std::collections::HashMap;

use crate::user_actor::user_actor::UserActor;
use audiolib::{self, audio_buffer::AudioBuffer, Channels};
use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use dsp_domain::{
    dsp_message::DspMessage,
    track::{Track, TrackInfo},
    tracks_message_result::TracksMessageResult,
};
use kameo::actor::spawn;
use kameo::actor::ActorRef;

fn create_actor() -> ActorRef<UserActor> {
    let processor = CommandProcessor::create_processor();
    let tracks = TracksState::new();
    let players = HashMap::new();
    let actor=  spawn(UserActor::new(processor, tracks, players));
    let g=kameo::registry::ActorRegistry::new();

    actor
}
#[actix::test]
async fn can_run_insert() -> Result<(), String> {
    
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = Track {
        info: TrackInfo { name: track_name },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let command = DspMessage::Insert {
        user_name: Some(user_name),
        track_payload: Some(serde_json::to_string(&track).unwrap()),
    };
    let addr = create_actor();
    let rez = addr.ask(command).await.map_err(|e| e.to_string())?;

    assert!(rez.output.contains("Inserted"));

    Ok(())
}

#[actix::test]
async fn can_run_copy() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let copy_track_name = "some_other_track".to_string();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = Track {
        info: TrackInfo {
            name: track_name.clone(),
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let addr = create_actor();
    let _ = insert_command(&addr, &user_name, track).await?;
    let after_insert_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(after_insert_list.len(), 1);

    let copy_command = DspMessage::Copy {
        user_name: Some(user_name.clone()),
        track_name: Some(track_name),
        copy_name: Some(copy_track_name),
    };
    let copy_result = addr.ask(copy_command).await.map_err(|e| e.to_string())?;
    assert!(copy_result.output.contains("Copied"));
    let after_copy_list = list_command(&addr, &user_name).await?;
    assert_eq!(after_copy_list.len(), 2);
    Ok(())
}

#[actix::test]
async fn can_run_list() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = Track {
        info: TrackInfo { name: track_name },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let addr = create_actor();
    let initial_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(initial_list.len(), 0);
    let insert_result = insert_command(&addr, &user_name, track).await?;

    assert!(insert_result.output.contains("Inserted"));
    let after_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(after_list.len(), 1);
    Ok(())
}

async fn insert_command(
    addr: &ActorRef<UserActor>,
    user_name: &str,
    track: Track,
) -> Result<TracksMessageResult, String> {
    let command = DspMessage::Insert {
        user_name: Some(user_name.to_string()),
        track_payload: Some(serde_json::to_string(&track).unwrap()),
    };
    let rez = addr.ask(command).await.map_err(|e| e.to_string());
    rez
}

async fn list_command(addr: &ActorRef<UserActor>, user_name: &str) -> Result<Vec<TrackInfo>, String> {
    let command = DspMessage::Ls {
        user_name: Some(user_name.to_string()),
    };
    let rez = addr.ask(command).await.map_err(|e| e.to_string())?;
    let ls: Vec<TrackInfo> = serde_json::from_str(&rez.output).unwrap();
    Ok(ls)
}
