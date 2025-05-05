use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::command::{Command, CommandResult};
use crate::command_dispatch::CommandDispatch;
use crate::state::{State, Track, TrackInfo, TrackStore};

pub type SharedState=Arc<Mutex<State>>;
pub struct Load{
    state:SharedState
}
pub struct Info{
    state:SharedState
}
pub struct List{
    state:SharedState
}
pub struct Save{
    state:SharedState
}

pub struct Unload{
    state:SharedState
}
pub struct Gain{
    state:SharedState
}
pub struct Normalize{
    state:SharedState
}
pub struct LowPass{
    state:SharedState
}
pub struct HighPass{
    state:SharedState
}

impl Load{
    fn internal_dispatch(&self,name:Option<String>,filename:String)->Result<CommandResult,String>{
        let name=PathBuf::from(name.ok_or(filename)?);
        let mut v=self.state.lock().unwrap();
        let result=audiolib::audio_parse::read_wav_file(&name)
            .map(|f| Track{info:TrackInfo{name:name.to_str().unwrap().to_string()},data:f})
            .and_then(|new_track|v.upsert_track(new_track))
            .map(|()|CommandResult{});
        result
    }   
}
impl Info{
    fn internal_dispatch(&self,name:Option<String>)->Result<CommandResult,String>{
        if let Some(info)=self
    }
}

impl List{
    fn internal_dispatch(&self)->Result<CommandResult,String>{
        
    }
}

impl Save{
    fn internal_dispatch(self,name:Option<String>,filename:String)->Result<CommandResult,String>{
        
    }
}

impl Unload{
    fn internal_dispatch(self,name:Option<String>)->Result<CommandResult,String>{
        
    }
}

impl Gain{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

impl Normalize{
    fn internal_dispatch(self,name:Option<String>)->Result<CommandResult,String>{
        
    }
}

impl LowPass{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

impl HighPass{
    fn internal_dispatch(self,name:Option<String>,cutoff:f32)->Result<CommandResult,String>{
        
    }
}

impl CommandDispatch for Load{
    fn dispatch(&self,command:Command)->Result<CommandResult,String>{
         let result=match command{
            Command::Load { name, filename }=>self.internal_dispatch(name, filename),
            _=> Err("".to_owned())
         };
         return result;
    }
}
impl CommandDispatch for List{
    fn dispatch(self,command:Command)->Result<CommandResult,String>{
        let result=match command{
           Command::Info { name }=>self.internal_dispatch(),
           _=> Err("".to_owned())
        };
        return result;
   }
}