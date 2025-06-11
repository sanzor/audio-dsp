use std::{collections::HashMap, sync::Arc};

use crate::user_actor::user_actor::UserActor;
use actix::{Actor, Addr};
use audiolib::{self, audio_buffer::AudioBuffer, Channels};
use dsp_core::{command_processor::CommandProcessor, state::SharedState};
use dsp_domain::{dsp_message::DspMessage, track::{Track, TrackInfo}, tracks_message_result::TracksMessageResult, user::User};
use tokio::sync::Mutex;

fn create_actor() -> UserActor {
    let processor = Arc::new(CommandProcessor::create_processor());
    let tracks = Arc::new(Mutex::new(SharedState::new()));
    let players = Arc::new(Mutex::new(HashMap::new()));
    UserActor::new(processor, tracks, players)
}
#[actix::test]
async fn can_run_insert()->Result<(),String> {
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
    let addr = create_actor().start();
    let rez=addr.send(command).await.map_err(|e| e.to_string())??;
    
    assert!(rez.output.contains("Inserted"));

    Ok(())
}

#[actix::test]
async fn can_run_list()->Result<(),String> {
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
    let addr = create_actor().start();
    let initial_list=list_command(&addr,  &user_name.clone()).await?;
    assert_eq!(initial_list.len(),0);
    let insert_result=insert_command(&addr, &user_name, track).await?;
    
    
    assert!(insert_result.output.contains("Inserted"));
    let after_list=list_command(&addr,  &user_name.clone()).await?;
    assert_eq!(after_list.len(),1);
    Ok(())
}


async fn insert_command(
    addr:&Addr<UserActor>,
    user_name:&str,
    track:Track
) -> Result<TracksMessageResult, String> {

    let command = DspMessage::Insert {
        user_name: Some(user_name.to_string()),
        track_payload:Some(serde_json::to_string(&track).unwrap())
    };
    let rez=addr.send(command).await.map_err(|e| e.to_string())?;
    rez
}

async fn list_command(
    addr:&Addr<UserActor>,
    user_name:&str,
) -> Result<Vec<TrackInfo>, String> {

    let command = DspMessage::Ls {
        user_name: Some(user_name.to_string())
    };
    let rez=addr.send(command).await.map_err(|e| e.to_string())??;
    let ls:Vec<TrackInfo>=serde_json::from_str(&rez.output).unwrap();
    Ok(ls)
}
