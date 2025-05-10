

use crate::{command::CommandResult, dispatcher_enum::DispatcherEnum, envelope::Envelope, scheduler::{self, Scheduler}};
use crate::command::Command;
pub struct Processor{ 
    scheduler:Scheduler
}

impl Processor{
    pub fn new(scheduler:Scheduler){
        Processor{scheduler:scheduler}
    }

    pub fn process_command(&self,input:&str)->CommandResult{
         let _result=
                parse_command(user_input.as_str())
                .map(dispatch)
                .map(|r|println!("{}",r));
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
    fn dispatch(&self,command:Command)->CommandResult{
        let dispatcher_name=self.get_dispatcher_name(command);
        let dispatcher=self.scheduler.dispatch_map.get(dispatcher).unwrap();
        CommandResult{}
    }
    fn get_dispatcher_name(&self,command:Command)->&'static str{
       match command{
            Command::Copy { name, copy_name }=>"copy",
            Command::Exit=>"exit",
            Command::HighPass { name, cutoff }=>"high_pass",
            Command::LowPass { name, cutoff }=>"low_pass",
            Command::Delete { name }=>"delete",
            Command::Info { name }=>"info",
            Command::Load { name, filename }=>"load",
            Command::Ls=>"ls",
            Command::Gain { name, gain, mode }=>"gain",
            Command::Normalize { name, mode }=>"normalize",
            Command::Upload { name, filename }=>"upload",
       }
    }
    
}