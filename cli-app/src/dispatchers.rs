use crate::command::{Command, CommandResult};
use crate::command_dispatch::CommandDispatch;
pub struct Load{}

impl Load{
    fn internal_dispatch(self,name:Option<String>,filename:String)->Result<CommandResult,String>{
        
    }
}
impl CommandDispatch for Load{
    fn dispatch(self,command:Command)->Result<CommandResult,String>{
         let result=match command{
            Command::Load { name, filename }=>self.internal_dispatch(name, filename),
            _=> Err("".to_owned())
         };
         return result;
    }
}