mod command;
mod command_parser;
mod command_dispatch;
mod command_dispatchers;
pub(crate) mod state;
mod scheduler;
use command::{Command, CommandResult};
use command_parser::*;


fn main() {
   loop{
        let user_input=read_line();
        loop{
            let _result=
                parse_command(user_input.as_str())
                .map(dispatch)
                .map(|r|println!("{}",r));
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
    if parts.len()<1{
        return Err("No command provided".to_string())
    }
    let command=parts.get(0).map(|v|v.to_ascii_lowercase());
    let payload=&parts[1..];
    let rez=match command.as_deref(){
        Some("load")=>parse_load(payload),
        Some("save")=>parse_save(payload),
        Some("info")=>parse_info(payload),
        Some("unload")=>parse_unload(payload),
        Some("list")=> parse_ls(payload),
        Some("gain")=>parse_gain(payload),
        Some("normalize")=> parse_normalize(payload),
        Some("low_pass")=> parse_low_pass(payload),
        Some("high_pass")=> parse_high_pass(payload),
        _ => Err("Could not process comand".to_string())
    };
    rez
}

fn dispatch(command:Command)->CommandResult{
    match command{
        Command::Exit=>return CommandResult{},
        Command::Info { name }=>command_dispatchers::
    }
}
