
use std::path::PathBuf;
use std::str::FromStr;

use audiolib::audio_parse;
use audiolib::audio_transform::AudioTransformMut;

use crate::command::{Command, CommandResult};
use crate::command_dispatch::CommandDispatch;

use crate::envelope::Envelope;
use crate::state::{SharedState, State};
pub struct ListDispatcher{}
pub struct UploadDispatcher{}

pub struct CopyDispatcher{}

pub struct DeleteDispatcher{}
pub struct GainDispatcher{}
pub struct NormalizeDispatcher{}
pub struct LowPassDispatcher{}
pub struct HighPassDispatcher{}




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



impl ListDispatcher{
    fn internal_dispatch(&self,state:&State)->Result<CommandResult,String>{
        let tracks=state.tracks();
        Ok(CommandResult {  output:serde_json::to_string_pretty(&tracks).unwrap()})
    }
}

impl UploadDispatcher{
    fn internal_dispatch(&self,name:Option<String>,filename:Option<String>,state:&State)->Result<CommandResult,String>{
        let name=name.ok_or_else(||"Invalid name to upload track")?;
        let track_ref=state.get_track_ref(&name).ok_or_else(||"Could not find track_ref")?;
        let file_path_str = filename.unwrap_or_else(|| name.clone());
        let path = PathBuf::from_str(&file_path_str).map_err(|err| err.to_string())?;
        let _=audio_parse::write_wav_file(&track_ref.inner.data, &path)?;
        Ok(CommandResult{ output:format!("Upload file successfully: {}",path.to_str().ok_or("invalid path")?.to_string())})
    }
}

impl CopyDispatcher{
    fn internal_dispatch(&self,name:Option<String>,copy_name:Option<String>,state:&mut State)->Result<CommandResult,String>{
        let fname=name.ok_or("Invalid name for copy")?;
        let mut new_track= state.get_track_copy(&fname.clone()).ok_or("Could not find track")?;

        let copy_name=copy_name.unwrap_or_else(||new_track.info.name.clone()+"v2");
        new_track.info.name=copy_name.clone();
        let _=state.upsert_track(new_track);
        Ok(CommandResult { output: format!("Copied successfully track:{} to {}",fname,copy_name)})
    }
}
impl DeleteDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
            let name=name.ok_or_else(||"Invalid name for deleted track".to_string())?;
            let _=state.delete_track(&name)?;
            Ok(CommandResult{ output: format!("Delete track {}",name)})
    }
}

impl GainDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
        let name=name.ok_or("Invalid name for track to perform gain on")?;
        let track_ref=state.get_track_ref_mut(&name).ok_or("Could not find track ref")?;
        let _=track_ref.inner.data.gain_mut(cutoff);
        Ok(CommandResult { output: format!("Updated gain for track {} succesful",name) })
    }
}

impl NormalizeDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
        let name=name.ok_or("Invalid name for track to perform normalize on")?;
        let track_ref=state.get_track_ref_mut(&name).ok_or_else(||"Could not find track ref")?;
        let _=track_ref.inner.data.normalize_mut();
        Ok(CommandResult { output: format!("Normalize track {} succesful",name) })
    }
}

impl LowPassDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
        let name=name.ok_or("Invalid name for track to low_pass on")?;
        let track_ref=state.get_track_ref_mut(&name).ok_or_else(||"Could not find track ref")?;
        let _=track_ref.inner.data.low_pass_mut(cutoff);
        Ok(CommandResult { output: format!("Normalize track {} succesful",name) })
    }
}

impl HighPassDispatcher{
    fn internal_dispatch(&self,name:Option<String>,cutoff:f32,state:&mut State)->Result<CommandResult,String>{
        let name=name.ok_or("Invalid name for track to high_pass on")?;
        let track_ref=state.get_track_ref_mut(&name).ok_or_else(||"Could not find track ref")?;
        let _=track_ref.inner.data.high_pass_mut(cutoff);
        Ok(CommandResult { output: format!("Normalize track {} succesful",name) })
    }
}

