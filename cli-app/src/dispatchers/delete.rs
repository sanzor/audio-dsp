use crate::{command::{Command, CommandResult}, command_dispatch::CommandDispatch, envelope::Envelope, state::{SharedState, State}};
pub struct DeleteDispatcher{}

impl DeleteDispatcher{
    fn internal_dispatch(&self,name:Option<String>,state:&mut State)->Result<CommandResult,String>{
            let name=name.ok_or_else(||"Invalid name for deleted track".to_string())?;
            let _=state.delete_track(&name)?;
            Ok(CommandResult{ output: format!("Delete track {}",name)})
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
