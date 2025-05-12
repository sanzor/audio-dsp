use crate::{command::{Command, CommandResult}, command_parser::*, command_processor::CommandProcessor};

pub struct Processor{
    command_processor:CommandProcessor,
}

impl Processor{
    pub fn new(command_processor:CommandProcessor)->Processor{
        Processor { command_processor: command_processor }
    }

    pub fn process(&mut self,input:&str)->Result<CommandResult,String>{
        let command=Processor::parse_command(input);
        let result=command.and_then(|com| self.command_processor.process_command(com));
        result
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
}

