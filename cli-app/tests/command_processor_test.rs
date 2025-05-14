use cli_app::{
    command::{Command, CommandResult},
    command_processor::CommandProcessor,
    dispatch_provider::DispatchProvider,
    state::create_shared_state,
    track::TrackInfo,
};
use rstest::rstest;
mod common;
#[rstest]
pub fn can_run_load_command() -> Result<(), String> {
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let mut processor = CommandProcessor::new(DispatchProvider::new(), create_shared_state());
    let command = Command::Load {
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
    let mut processor = CommandProcessor::new(DispatchProvider::new(), create_shared_state());
    load_command(&mut processor, name).unwrap();
    let info_command = Command::Info {
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
    let mut processor = CommandProcessor::new(DispatchProvider::new(), create_shared_state());
    load_command(&mut processor, name).unwrap();
    let info_command = Command::Ls {
    };
    let ls_result = processor.process_command(info_command)?.output;
    let track_list: Vec<TrackInfo> = serde_json::from_str(&ls_result).unwrap();
    assert!(track_list.len()==1);
    assert!(track_list[0].name==name);
    Ok(())
}

#[rstest]
pub fn can_run_save_command() -> Result<(), String> {
    let name = "my-track";
    let mut processor = CommandProcessor::new(DispatchProvider::new(), create_shared_state());
    load_command(&mut processor, name).unwrap();
    let info_command = Command::Ls {
    };
    let ls_result = processor.process_command(info_command)?.output;
    let track_list: Vec<TrackInfo> = serde_json::from_str(&ls_result).unwrap();
    assert!(track_list.len()==1);
    assert!(track_list[0].name==name);
    Ok(())
}

fn load_command(processor: &mut CommandProcessor, name: &str) -> Result<CommandResult, String> {
    let path = common::test_data("dragons.wav");
    let path_str = path.to_str().ok_or_else(|| "Invalid file".to_string())?;
    let command = Command::Load {
        name: Some(name.to_string()),
        filename: Some(path_str.to_string()),
    };
    let _result = processor.process_command(command);
    _result
}
