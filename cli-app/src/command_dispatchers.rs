

use std::path::PathBuf;

use audiolib::audio_parse;

use crate::command::{Command, CommandResult};
use crate::command_dispatch::CommandDispatch;
use crate::envelope::Envelope;
use crate::state::{SharedState, State};
use crate::track::{Track, TrackInfo};


pub struct LoadDispatcher{}
pub struct InfoDispatcher{}
pub struct ListDispatcher{}
pub struct UploadDispatcher{}

pub struct CopyDispatcher{}

pub struct UnloadDispatcher{}
pub struct GainDispatcher{}
pub struct NormalizeDispatcher{}
pub struct LowPassDispatcher{}
pub struct HighPassDispatcher{}


impl CommandDispatch for LoadDispatcher{
    fn dispatch(&self,envelope:Envelope,state:SharedState)->Result<CommandResult,String>{
         let result=state
         .try_write()
         .map_err(|e|e.to_string())
         .and_then(|mut guard|{
            let  state_ref=&mut *guard;
            match envelope.command{
                Command::Load { name, filename }=>self.internal_dispatch(name, filename,state_ref),
                _=> Err("".to_owned())
            }
         });
        
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
        let result=state.try_read()
        .map_err(|e|e.to_string())
            .and_then(|guard|{
                let state_ref=&*guard;
                match envelope.command {
                    Command::Info { name }=>self.internal_dispatch(name, state_ref),
                    _ =>Err("".to_owned())
                }
            });
        return result;
    }
}

impl CommandDispatch for UnloadDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let result=state.try_write()
            .map_err(|e|e.to_string())
            .and_then(|mut guard|{
                let state=&mut *guard;
                match envelope.command{
                    Command::Delete { name }=>self.internal_dispatch(name,state),
                    _ => Err("".to_string())
                }
            });
        return result;
    }
}

impl CommandDispatch for CopyDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let result=state.try_write()
        .map_err(|e|e.to_string())
        .and_then(|mut guard|{
            let s=&mut *guard;
            match envelope.command{
                Command::Copy { name, copy_name }=>self.internal_dispatch(name, copy_name, state),
                _=>Err("Could not perform copy".to_owned())
            }
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
        let rez=name.map(|n|state.get_track_info(n.as_str()));
        Ok(CommandResult{})
    }
}

impl ListDispatcher{
    fn internal_dispatch(&self,state:&State)->Result<CommandResult,String>{
        let _=state.tracks();
        Ok(CommandResult {  })
    }
}

impl UploadDispatcher{
    fn internal_dispatch(self,name:Option<String>,filename:String,state:&State)->Result<CommandResult,String>{
        let result=name.map(|n|state.get_track(n.as_str())
            .and_then(|track|{
                audio_parse::write_wav_file(&track.data, filename)
            }
            );
    }
}

impl CopyDispatcher{
    fn internal_dispatch(self,name:Option<String>,copy_name:Option<String>,state:&State)->Result<CommandResult,String>{
        state.get_track(name)
            .and_then(|track|{
                
            })

    
    }
}
impl UnloadDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
        
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

