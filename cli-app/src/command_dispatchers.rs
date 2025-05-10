

use std::path::PathBuf;
use std::str::FromStr;

use audiolib::audio_parse;
use audiolib::audio_transform::AudioTransformMut;

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

pub struct DeleteDispatcher{}
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

impl CommandDispatch for UploadDispatcher{
    fn dispatch(&self,envelope:Envelope,state:SharedState)->Result<CommandResult,String>{
         let result=state
         .try_write()
         .map_err(|e|e.to_string())
         .and_then(|mut guard|{
            let  state_ref=&mut *guard;
            match envelope.command{
                Command::Upload { name, filename }=>self.internal_dispatch(name, filename,state_ref),
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

impl CommandDispatch for DeleteDispatcher{
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
                Command::Copy { name, copy_name }=>self.internal_dispatch(name, copy_name, s),
                _=>Err("Could not perform copy".to_owned())
            }
        });
        result
    }
}


impl CommandDispatch for GainDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let rez=state.try_write()
            .map_err(|e|e.to_string())
            .and_then(|mut guard|{
                 let s=&mut *guard;
                 match envelope.command{
                    Command::Gain { name, gain, mode }=>self.internal_dispatch(name, gain,s),
                    _ =>Err("err".to_string())
                 }
                });
        return rez;
    }   
}

impl CommandDispatch for NormalizeDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let rez=state.try_write()
            .map_err(|e|e.to_string())
            .and_then(|mut guard|{
                 let s=&mut *guard;
                 match envelope.command{
                    Command::Normalize { name,mode: _}=>self.internal_dispatch(name,s),
                    _ =>Err("err".to_string())
                 }
                });
        return rez;
    }   
}

impl CommandDispatch for LowPassDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let rez=state.try_write()
            .map_err(|e|e.to_string())
            .and_then(|mut guard|{
                 let s=&mut *guard;
                 match envelope.command{
                    Command::LowPass { name,cutoff}=>self.internal_dispatch(name,cutoff,s),
                    _ =>Err("err".to_string())
                 }
                });
        return rez;
    }   
}

impl CommandDispatch for HighPassDispatcher{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
        let rez=state.try_write()
            .map_err(|e|e.to_string())
            .and_then(|mut guard|{
                 let s=&mut *guard;
                 match envelope.command{
                    Command::HighPass { name,cutoff}=>self.internal_dispatch(name,cutoff,s),
                    _ =>Err("err".to_string())
                 }
                });
        return rez;
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
    fn internal_dispatch(&self,name:Option<String>,filename:String,state:&State)->Result<CommandResult,String>{
        let result=name
            .ok_or_else(||"Invalid name".to_string())
            .and_then(|name|
                state.get_track_ref(name.as_str())
                     .ok_or_else(||"Track not found".to_string()))
            .and_then(|track_ref| match PathBuf::from_str(&filename){
                        Ok(path)=>Ok((track_ref,path)),
                        _=>Err("Could not read path".to_owned())
            })
            .and_then(|(track_ref,path)|
                audio_parse::write_wav_file(&track_ref.inner.data, &path))
            .map(|()| CommandResult{});
        return result;
    }
}

impl CopyDispatcher{
    fn internal_dispatch(&self,name:Option<String>,copy_name:Option<String>,state:&mut State)->Result<CommandResult,String>{
       
       let result=
                name
                .ok_or_else(||"invalid name".to_string())
                .and_then(|n| 
                    match state.get_track_copy(n.as_str()){
                        Some(track)=>Ok(track),
                        _ => Err("Could not find track".to_string())
                })
                .map(|mut new_track|{
                    new_track.info.name=copy_name.ok_or(new_track.info.name+"v2").unwrap();
                    new_track
                })
                .and_then(|new_track|state.upsert_track(new_track))
              
                .map(|()| CommandResult {  });
            return result;
    
    }
}
impl DeleteDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
            let result=
                    name.ok_or_else(||"Invalid name for deleted track".to_string())
                    .and_then(|n|
                        state.delete_track(n.as_str())
                    )
                    .map(|()|CommandResult{});
            return result;
    }
}

impl GainDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
        let result=
                    name.ok_or_else(||"Invalid name for deleted track".to_string())
                        .and_then(|n| 
                            state.get_track_ref_mut(n.as_str())
                             .ok_or_else(||"could not find ref mut".to_string())
                             .map(|track_ref| track_ref.inner.data.gain_mut(cutoff))
                        )
                        .map(|x|CommandResult{});
        return result;
    }
}

impl NormalizeDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
          let result=
                    name.ok_or_else(||"Invalid name for deleted track".to_string())
                        .and_then(|n| 
                            state.get_track_ref_mut(n.as_str())
                             .ok_or_else(||"could not find ref mut".to_string())
                             .map(|track_ref| track_ref.inner.data.normalize_mut())
                        )
                        .map(|_|CommandResult{});
        return result;
    }
}

impl LowPassDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
         let result=
                    name.ok_or_else(||"Invalid name for deleted track".to_string())
                        .and_then(|n| 
                            state.get_track_ref_mut(n.as_str())
                             .ok_or_else(||"could not find ref mut".to_string())
                             .map(|track_ref| track_ref.inner.data.low_pass_mut(cutoff))
                        )
                        .map(|x|CommandResult{});
        return result;
    }
}

impl HighPassDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
         let result=
                    name.ok_or_else(||"Invalid name for deleted track".to_string())
                        .and_then(|n| 
                            state.get_track_ref_mut(n.as_str())
                             .ok_or_else(||"could not find ref mut".to_string())
                             .map(|track_ref| track_ref.inner.data.high_pass_mut(cutoff))
                        )
                        .map(|x|CommandResult{});
        return result;
    }
}

