
use crate::{command::{Command, CommandResult}, command_dispatch::CommandDispatch, envelope::Envelope, state::{SharedState, State}};

pub struct CopyDispatcher{}


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
