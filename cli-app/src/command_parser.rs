use crate::command::{Command, RunMode};

pub fn parse_load(value:&[&str])->Result<Command,String>{
    match value {
        [filename,name]=>Ok(Command::Load { filename: filename.to_string(),name:Some(name.to_string())}),
        [filename]=>Ok(Command::Load { filename: filename.to_string(),name:Some(filename.to_string())}),
        _ =>Err("Invalid load command".to_owned())
    }
}
pub fn parse_save(value:&[&str])->Result<Command,String>{
    match value{
        [filename,name]=>Ok(Command::Save { filename: filename.to_string(),name:Some(name.to_string())}),
        [name]=>Ok(Command::Save { filename: name.to_string(),name:Some(name.to_string())}),
        _ =>Err("Invalid save command".to_owned())

    } 
}

pub fn parse_unload(value:&[&str])->Result<Command,String>{
    match value{
        [name]=>Ok(Command::Delete { name:Some(name.to_string())}),
        _ =>Err("Invalid unload command".to_owned())
    }
}

pub fn parse_info(value:&[&str])->Result<Command,String>{
    match value{
        [name]=>Ok(Command::Info { name: Some(name.to_string())}),
        _ =>Err("Invalid info command".to_owned())
    }
}

pub fn parse_ls(value:&[&str])->Result<Command,String>{
    let _ = value;
    Ok(Command::Ls)
}
pub fn parse_gain(value:&[&str])->Result<Command,String>{
    match value{
        [name,factor]=>factor.parse::<f32>()
                                            .map(|f|Command::Gain { name:Some(name.to_string()),gain:f,mode:Some(RunMode::Simple)})
                                            .map_err(|e|e.to_string()),
        _ =>Err("Invalid gain command".to_owned())
    }
}
pub fn parse_normalize(value:&[&str])->Result<Command,String>{
    match value{
        [name]=> Ok(Command::Normalize { name:Some(name.to_string()),mode: Some(RunMode::Simple) }),
        _ =>Err("Invalid load command".to_owned())
    }
}
pub fn parse_low_pass(value:&[&str])->Result<Command,String>{
    match value{
        [cutoff,name]=> 
                            cutoff.parse::<f32>()
                            .map(|f| Command::LowPass { name:Some(name.to_string()),cutoff: f })
                            .map_err(|e|e.to_string()),
        _ =>Err("Invalid low_pass command".to_owned())
    }
}
pub fn parse_high_pass(value:&[&str])->Result<Command,String>{
    match value{
        [cutoff,name]=>
                            cutoff.parse::<f32>()
                            .map(|f| Command::HighPass{ name:Some(name.to_string()),cutoff:  f})
                            .map_err(|e|e.to_string()),
        _ =>Err("Invalid high_pass command".to_owned())
    }
}