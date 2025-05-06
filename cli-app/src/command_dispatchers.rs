
use std::env;
use std::path::PathBuf;

use crate::command::{Command, CommandResult};
use crate::command_dispatch::CommandDispatch;
use crate::envelope::Envelope;
use crate::state::{SharedState, State};
use crate::track::{Track, TrackInfo};


pub struct LoadDispatcher{}
pub struct InfoDispatcher{}
pub struct ListDispatcher{}
pub struct SaveDispatcher{}

pub struct UnloadDispatcher{}
pub struct GainDispatcher{}
pub struct NormalizeDispatcher{}
pub struct LowPassDispatcher{}
pub struct HighPassDispatcher{}


impl CommandDispatch for LoadDispatcher{
    fn dispatch(&self,envelope:Envelope,state:SharedState)->Result<CommandResult,String>{
         let mut guard=state.write().unwrap();
         let result=match envelope.command{
            Command::Load { name, filename }=>self.internal_dispatch(name, filename,&mut *guard),
            _=> Err("".to_owned())
         };
         return result;
    }
}
impl CommandDispatch for ListDispatcher{
    fn dispatch(&self,envelope:Envelope,state:SharedState)->Result<CommandResult,String>{
        let guard=state.read().unwrap();
        let result=match envelope.command{
           Command::Info { name:_ }=>self.internal_dispatch(&*guard),
           _=> Err("".to_owned())
        };
        return result;
   }
}

impl CommandDispatch for InfoDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let result=state.try_read().map_err(|e|e.to_string())?;
        let guard=&*result;
        let rez=
            .map_err(|err| err.to_string())
            .and_then(|state|{
                let result=match envelope.command {
                    Command::Info { name }=>self.internal_dispatch(name, state),
                    _ =>Err("".to_owned())
                };
                result
            });
        result
    }
}



impl LoadDispatcher{
    fn internal_dispatch(&self,name:Option<String>,filename:String,state:&mut State)->Result<CommandResult,String>{
        let name=PathBuf::from(name.ok_or(filename)?);
        let result=audiolib::audio_parse::read_wav_file(&name)
            .map(|f| Track{info:TrackInfo{name:name.to_str().unwrap().to_string()},data:f})
            .and_then(|new_track|state.upsert_track(new_track))
            .map(|()|CommandResult{});
        result
    }   
}
impl InfoDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&State)->Result<CommandResult,String>{
        name.map(|n|state.get_track_info(name))
    }
}

impl ListDispatcher{
    fn internal_dispatch(&self,state:&State)->Result<CommandResult,String>{
        
    }
}

impl SaveDispatcher{
    fn internal_dispatch(self,name:Option<String>,filename:String)->Result<CommandResult,String>{
        
    }
}

impl UnloadDispatcher{
    fn internal_dispatch(self,name:Option<String>)->Result<CommandResult,String>{
        
    }
}

impl GainDispatcher{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

impl NormalizeDispatcher{
    fn internal_dispatch(self,name:Option<String>)->Result<CommandResult,String>{
        
    }
}

impl LowPassDispatcher{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

impl HighPassDispatcher{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

