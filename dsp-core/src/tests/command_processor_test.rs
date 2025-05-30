use dsp_domain::{dsp_command_result::DspCommandResult, message::Message, track::TrackInfo, user};
use rstest::rstest;
use serde_json::to_string;

use crate::{
    command_processor::CommandProcessor, command_processor_test::common,
    dispatchers_provider::DispatchersProvider, state::create_shared_state,
};

#[rstest]
pub async fn can_run_load_command() -> Result<(), String> {
    let user_name = "some_user";
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    let command = Message::Load {
        user_name: Some(user_name.to_string()),
        track_name: Some("dragons.wav".to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_command(command).await?;
    assert!(_result.output.contains(""));
    Ok(())
}

#[rstest]
pub async fn can_run_info_command() -> Result<(), String> {
    let user_name = "some_user";
    let track_name = "my-track";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    load_command(&mut processor, track_name).await?;
    let info_command = Message::Info {
        user_name: Some(user_name.to_string()),
        track_name: Some(track_name.to_string()),
    };
    let info_result_str = processor.process_command(info_command).await?.output;
    let info: TrackInfo = serde_json::from_str(&info_result_str).unwrap();
    assert!(info.name == track_name);
    Ok(())
}

#[rstest]
pub async fn can_run_list_command() -> Result<(), String> {
    let name = "my-track";

    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    load_command(&mut processor, name).await?;
    let info_command = Message::Ls {
        user_name: Some(name.to_string()),
    };
    let ls_result = processor.process_command(info_command).await?.output;
    let track_list: Vec<TrackInfo> = serde_json::from_str(&ls_result).unwrap();
    assert!(track_list.len() == 1);
    assert!(track_list[0].name == name);
    Ok(())
}

#[rstest]
pub async fn can_run_upload_command() -> Result<(), String> {
    let user_name = "my-my_user";
    let track_name = "my-track";
    let filename = "dragons2.wav";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    let c = load_command(&mut processor, track_name).await?;
    let upload_command = Message::Upload {
        user_name: Some(user_name.to_string()),
        track_name: Some(track_name.to_string()),
        filename: Some(filename.to_string()),
    };
    let upload_result = processor.process_command(upload_command).await?.output;
    assert!(upload_result.contains("successfully"));
    Ok(())
}

#[rstest]
pub async fn can_run_delete_command() -> Result<(), String> {
    let name = "my-track";
    let user_name = "my-my_user";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    load_command(&mut processor, name).await?;

    let track_list_before_delete = get_track_list(&mut processor, &user_name).await?;
    assert!(track_list_before_delete.len() == 1);

    let delete_command = Message::Delete {
        user_name: Some(user_name.to_string()),
        track_name: Some(name.to_string()),
    };
    let delete_command_result = processor.process_command(delete_command).await?;
    assert!(delete_command_result.output.contains("succesful"));

    let track_list_after_delete = get_track_list(&mut processor, &user_name).await?;
    assert!(track_list_after_delete.len() == 0);

    Ok(())
}

#[rstest]
pub async fn can_run_copy_command() -> Result<(), String> {
    let user_name = "my-my_user";
    let name = "my-track";
    let copy_name = "my-track2";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    load_command(&mut processor, name).await?;

    let copy_result_string = &processor
        .process_command(Message::Copy {
            user_name: Some(user_name.to_string()),
            track_name: Some(name.to_string()),
            copy_name: Some(copy_name.to_string()),
        })
        .await?
        .output;
    assert!(copy_result_string.contains("Copied successfully"));
    let track_list_after_copy = get_track_list(&mut processor, &user_name).await?;
    assert!(track_list_after_copy.len() == 2);
    assert!(track_list_after_copy[1].name == copy_name);

    Ok(())
}

#[rstest]
pub async fn can_run_exit_command() -> Result<(), String> {
    let name = "my-track";
    let mut command_processor =
        CommandProcessor::new(DispatchersProvider::new(), create_shared_state());
    load_command(&mut command_processor, name).await.unwrap();
    let exit_command = Message::Exit {};
    let upload_result = command_processor.process_command(exit_command).await;
    assert!(upload_result.is_err());
    let error = upload_result.unwrap_err();
    assert!(error.contains("Could not find dispatcher"));
    Ok(())
}

async fn load_command(
    processor: &mut CommandProcessor,
    name: &str,
) -> Result<DspCommandResult, String> {
    let user_name = "my-my_user";
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let command = Message::Load {
        user_name: Some(user_name.to_string()),
        track_name: Some(name.to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_command(command).await;
    _result
}

async fn get_track_list(
    processor: &mut CommandProcessor,
    user_name: &str,
) -> Result<Vec<TrackInfo>, String> {
    serde_json::from_str(
        &processor
            .process_command(Message::Ls {
                user_name: Some(user_name.to_string()),
            })
            .await?
            .output,
    )
    .map_err(|e| e.to_string())
}
