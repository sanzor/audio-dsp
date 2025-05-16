use dsp_domain::command::{DspCommand, RunMode};

pub fn parse_load(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [filename, name] => Ok(DspCommand::Load {
            filename: Some(filename.to_string()),
            name: Some(name.to_string()),
        }),
        [filename] => Ok(DspCommand::Load {
            filename: Some(filename.to_string()),
            name: Some(filename.to_string()),
        }),
        _ => Err("Invalid load command".to_owned()),
    }
}
pub fn parse_upload(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [filename, name] => Ok(DspCommand::Upload {
            filename: Some(filename.to_string()),
            name: Some(name.to_string()),
        }),
        [name] => Ok(DspCommand::Upload {
            filename: Some(name.to_string()),
            name: Some(name.to_string()),
        }),
        _ => Err("Invalid save command".to_owned()),
    }
}

pub fn parse_unload(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [name] => Ok(DspCommand::Delete {
            name: Some(name.to_string()),
        }),
        _ => Err("Invalid unload command".to_owned()),
    }
}

pub fn parse_info(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [name] => Ok(DspCommand::Info {
            name: Some(name.to_string()),
        }),
        _ => Err("Invalid info command".to_owned()),
    }
}

pub fn parse_ls(value: &[&str]) -> Result<DspCommand, String> {
    let _ = value;
    Ok(DspCommand::Ls)
}
pub fn parse_gain(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [name, factor] => factor
            .parse::<f32>()
            .map(|f| DspCommand::Gain {
                name: Some(name.to_string()),
                gain: f,
                mode: Some(RunMode::Simple),
            })
            .map_err(|e| e.to_string()),
        _ => Err("Invalid gain command".to_owned()),
    }
}
pub fn parse_normalize(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [name] => Ok(DspCommand::Normalize {
            name: Some(name.to_string()),
            mode: Some(RunMode::Simple),
        }),
        _ => Err("Invalid load command".to_owned()),
    }
}
pub fn parse_low_pass(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [cutoff, name] => cutoff
            .parse::<f32>()
            .map(|f| DspCommand::LowPass {
                name: Some(name.to_string()),
                cutoff: f,
            })
            .map_err(|e| e.to_string()),
        _ => Err("Invalid low_pass command".to_owned()),
    }
}
pub fn parse_high_pass(value: &[&str]) -> Result<DspCommand, String> {
    match value {
        [cutoff, name] => cutoff
            .parse::<f32>()
            .map(|f| DspCommand::HighPass {
                name: Some(name.to_string()),
                cutoff: f,
            })
            .map_err(|e| e.to_string()),
        _ => Err("Invalid high_pass command".to_owned()),
    }
}

pub fn parse_play(value: &[&str])->Result<DspCommand,String>{
    match value{
        [name] => Ok(DspCommand::Play { name:Some(name.to_string())}),
        _ => Err("Invalid run command".to_string())
    }
}
