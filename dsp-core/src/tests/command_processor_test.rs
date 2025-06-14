use audiolib::{audio_buffer::AudioBuffer, Channels};
use maplit::hashmap;
use std::{collections::HashMap, sync::Arc};

fn create_state(tracks: HashMap<String, Track>) -> Arc<Mutex<SharedState>> {
    Arc::new(Mutex::new(SharedState { tracks }))
}

use dsp_domain::{
    dsp_message::DspMessage,
    track::{Track, TrackInfo},
    tracks_message_result::TracksMessageResult,
};
use rstest::rstest;
use tokio::sync::Mutex;

use crate::{
    command_processor::CommandProcessor, command_processor_test::common,
    dispatchers_provider::DispatchersProvider, state::SharedState,
};

#[rstest]
#[actix_rt::test]
pub async fn can_run_load_command() -> Result<(), String> {
    let user_name = "some_user";
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;

    let state = create_state(hashmap! {});
    let processor = CommandProcessor::new(DispatchersProvider::new());
    let command = DspMessage::Load {
        user_name: Some(user_name.to_string()),
        track_name: Some("dragons.wav".to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_crud_command(command, state).await?;
    assert!(_result.output.contains("Loaded"));
    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_insert_command() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let state = create_state(hashmap! {});
    let processor = CommandProcessor::new(DispatchersProvider::new());
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
    let _result = processor.process_crud_command(command, state).await?;
    assert!(_result.output.contains("Inserted"));

    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_info_command() -> Result<(), String> {
    let user_name = "some_user";
    let track_name = "my-track";
    let state = create_state(hashmap! {});
    let mut processor = CommandProcessor::new(DispatchersProvider::new());
    load_command(&mut processor, track_name, Arc::clone(&state)).await?;
    let info_command = DspMessage::Info {
        user_name: Some(user_name.to_string()),
        track_name: Some(track_name.to_string()),
    };
    let info_result_str = processor
        .process_crud_command(info_command, state)
        .await?
        .output;
    let info: TrackInfo = serde_json::from_str(&info_result_str).unwrap();
    assert!(info.name == track_name);
    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_list_command() -> Result<(), String> {
    let name = "my-track";
    let state = create_state(hashmap! {});
    let mut processor = CommandProcessor::new(DispatchersProvider::new());
    load_command(&mut processor, name, Arc::clone(&state)).await?;
    let info_command = DspMessage::Ls {
        user_name: Some(name.to_string()),
    };

    let ls_result = processor
        .process_crud_command(info_command, state)
        .await?
        .output;
    let track_list: Vec<TrackInfo> = serde_json::from_str(&ls_result).unwrap();
    assert!(track_list.len() == 1);
    assert!(track_list[0].name == name);
    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_upload_command() -> Result<(), String> {
    let user_name = "my-my_user";
    let track_name = "my-track";
    let filename = "dragons2.wav";
    let mut processor = CommandProcessor::new(DispatchersProvider::new());
    let state = create_state(hashmap! {});
    let c = load_command(&mut processor, track_name, Arc::clone(&state)).await?;
    let upload_command = DspMessage::Upload {
        user_name: Some(user_name.to_string()),
        track_name: Some(track_name.to_string()),
        filename: Some(filename.to_string()),
    };
    let upload_result = processor
        .process_crud_command(upload_command, state)
        .await?
        .output;
    assert!(upload_result.contains("successfully"));
    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_delete_command() -> Result<(), String> {
    let name = "my-track";
    let user_name = "my-my_user";
    let state = create_state(hashmap! {});
    let mut processor = CommandProcessor::new(DispatchersProvider::new());
    load_command(&mut processor, name, Arc::clone(&state)).await?;

    let track_list_before_delete =
        get_track_list(&mut processor, &user_name, Arc::clone(&state)).await?;
    assert!(track_list_before_delete.len() == 1);

    let delete_command = DspMessage::Delete {
        user_name: Some(user_name.to_string()),
        track_name: Some(name.to_string()),
    };
    let delete_command_result = processor
        .process_crud_command(delete_command, Arc::clone(&state))
        .await?;
    assert!(delete_command_result.output.contains("succesful"));

    let track_list_after_delete =
        get_track_list(&mut processor, &user_name, Arc::clone(&state)).await?;
    assert!(track_list_after_delete.len() == 0);

    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_copy_command() -> Result<(), String> {
    let user_name = "my-my_user";
    let name = "my-track";
    let copy_name = "my-track2";
    let mut processor = CommandProcessor::new(DispatchersProvider::new());
    let state = create_state(hashmap! {});
    load_command(&mut processor, name, Arc::clone(&state)).await?;

    let copy_result_string = &processor
        .process_crud_command(
            DspMessage::Copy {
                user_name: Some(user_name.to_string()),
                track_name: Some(name.to_string()),
                copy_name: Some(copy_name.to_string()),
            },
            Arc::clone(&state),
        )
        .await?
        .output;
    assert!(copy_result_string.contains("Copied successfully"));
    let track_list_after_copy =
        get_track_list(&mut processor, &user_name, Arc::clone(&state)).await?;
    assert!(track_list_after_copy.len() == 2);
    assert!(track_list_after_copy[1].name == copy_name);

    Ok(())
}

#[rstest]
#[actix_rt::test]
pub async fn can_run_exit_command() -> Result<(), String> {
    let user_name = "my-track";
    let track_name = "my-track";
    let mut command_processor = CommandProcessor::new(DispatchersProvider::new());
    let state = create_state(hashmap! {});
    load_command(&mut command_processor, track_name, Arc::clone(&state))
        .await
        .unwrap();
    let exit_command = DspMessage::Exit {
        user_name: Some(user_name.to_string()),
    };
    let upload_result = command_processor
        .process_crud_command(exit_command, Arc::clone(&state))
        .await;
    assert!(upload_result.is_err());
    let error = upload_result.unwrap_err();
    assert!(error.contains("Could not find dispatcher"));
    Ok(())
}

async fn load_command(
    processor: &mut CommandProcessor,
    name: &str,
    state: Arc<Mutex<SharedState>>,
) -> Result<TracksMessageResult, String> {
    let user_name = "my-my_user";
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let command = DspMessage::Load {
        user_name: Some(user_name.to_string()),
        track_name: Some(name.to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_crud_command(command, state).await;
    _result
}

async fn get_track_list(
    processor: &mut CommandProcessor,
    user_name: &str,
    state: Arc<Mutex<SharedState>>,
) -> Result<Vec<TrackInfo>, String> {
    serde_json::from_str(
        &processor
            .process_crud_command(
                DspMessage::Ls {
                    user_name: Some(user_name.to_string()),
                },
                state,
            )
            .await?
            .output,
    )
    .map_err(|e| e.to_string())
}
