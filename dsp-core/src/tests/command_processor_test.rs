use dsp_domain::{dsp_command_result::DspCommandResult, message::Message, track::TrackInfo};
use rstest::rstest;

use crate::{
    command_processor::CommandProcessor, command_processor_test::common,
    dispatchers_provider::DispatchersProvider, state::create_state,
};

#[rstest]
pub fn can_run_load_command() -> Result<(), String> {
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    let command = Message::Load {
        name: Some("dragons.wav".to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_command(command)?;
    assert!(_result.output.contains(""));
    Ok(())
}

#[rstest]
pub fn can_run_info_command() -> Result<(), String> {
    let name = "my-track";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut processor, name).unwrap();
    let info_command = Message::Info {
        name: Some(name.to_string()),
    };
    let info_result_str = processor.process_command(info_command)?.output;
    let info: TrackInfo = serde_json::from_str(&info_result_str).unwrap();
    assert!(info.name == name);
    Ok(())
}

#[rstest]
pub fn can_run_list_command() -> Result<(), String> {
    let name = "my-track";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut processor, name).unwrap();
    let info_command = Message::Ls {};
    let ls_result = processor.process_command(info_command)?.output;
    let track_list: Vec<TrackInfo> = serde_json::from_str(&ls_result).unwrap();
    assert!(track_list.len() == 1);
    assert!(track_list[0].name == name);
    Ok(())
}

#[rstest]
pub fn can_run_upload_command() -> Result<(), String> {
    let name = "my-track";
    let filename = "dragons2.wav";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut processor, name).unwrap();
    let upload_command = Message::Upload {
        name: Some(name.to_string()),
        filename: Some(filename.to_string()),
    };
    let upload_result = processor.process_command(upload_command)?.output;
    assert!(upload_result.contains("successfully"));
    Ok(())
}

#[rstest]
pub fn can_run_delete_command() -> Result<(), String> {
    let name = "my-track";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut processor, name).unwrap();

    let track_list_before_delete = get_track_list(&mut processor)?;
    assert!(track_list_before_delete.len() == 1);

    let delete_command = Message::Delete {
        name: Some(name.to_string()),
    };
    let delete_command_result = processor.process_command(delete_command)?.output;
    assert!(delete_command_result.contains("succesful"));

    let track_list_after_delete = get_track_list(&mut processor)?;
    assert!(track_list_after_delete.len() == 0);

    Ok(())
}

#[rstest]
pub fn can_run_copy_command() -> Result<(), String> {
    let name = "my-track";
    let copy_name = "my-track2";
    let mut processor = CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut processor, name).unwrap();

    let copy_result_string = &processor
        .process_command(Message::Copy {
            name: Some(name.to_string()),
            copy_name: Some(copy_name.to_string()),
        })?
        .output;
    assert!(copy_result_string.contains("Copied successfully"));
    let track_list_after_copy = get_track_list(&mut processor)?;
    assert!(track_list_after_copy.len() == 2);
    assert!(track_list_after_copy[1].name == copy_name);

    Ok(())
}

#[rstest]
pub fn can_run_exit_command() -> Result<(), String> {
    let name = "my-track";
    let mut command_processor =
        CommandProcessor::new(DispatchersProvider::new(), create_state());
    load_command(&mut command_processor, name).unwrap();
    let exit_command = Message::Exit {};
    let upload_result = command_processor.process_command(exit_command);
    assert!(upload_result.is_err());
    let error = upload_result.unwrap_err();
    assert!(error.contains("Could not find dispatcher"));
    Ok(())
}

fn load_command(processor: &mut CommandProcessor, name: &str) -> Result<DspCommandResult, String> {
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let command = Message::Load {
        name: Some(name.to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_command(command);
    _result
}

fn get_track_list(processor: &mut CommandProcessor) -> Result<Vec<TrackInfo>, String> {
    serde_json::from_str(&processor.process_command(Message::Ls {})?.output)
        .map_err(|e| e.to_string())
}
