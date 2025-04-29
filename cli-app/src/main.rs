mod command;

use command::{Command, CommandResult, RunMode};
fn main() {
   loop{
        let user_input=read_line();
        loop{
            let command=parse_command(user_input.as_str()).expect("Could not parse command {command}");
            let result=dispatch(command);
        }
   }
}

fn read_line()->String{
    use std::io::{self,Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}

fn parse_command(input:&str)->Result<Command,String>{
    let parts:Vec<&str>=input.trim().split_whitespace().collect();
    let parts_slice=parts.as_slice();
    let rez=match parts_slice{
        ["load", filename,name]=>Ok(Command::Load { filname: filename.to_string(),name:name.to_string()}),
        ["save", filename,name]=>Ok(Command::Save { filname: filename.to_string(),name:Some(name.to_string())}),
        ["unload",name]=>Ok(Command::Delete { name:name.to_string()}),
        ["list"]=> Ok(Command::Ls),
        ["gain",factor]=>factor.parse::<f32>().map(|f|Command::Gain { gain:f,mode:Some(RunMode::Simple)}).map_err(|e|e.to_string()),
        ["normalize"]=> Ok(Command::Normalize { mode: Some(RunMode::Simple) }),
        ["low_pass",cutoff]=> cutoff.parse::<f32>().map(|f| Command::LowPass { cutoff: f }).map_err(|e|e.to_string()),
        ["high_pass",cutoff]=>cutoff.parse::<f32>().map(|f| Command::HighPass{ cutoff:  f}).map_err(|e|e.to_string()),
        _ => Err("Could not process comand".to_string())
    };
    rez
   
    
}

fn dispatch(command:Command)->CommandResult{
    
}
