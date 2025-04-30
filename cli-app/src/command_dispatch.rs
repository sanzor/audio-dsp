use crate::command::{Command, CommandResult};

pub trait CommandDispatch{
    fn dispatch(self,command:Command)->Result<CommandResult,String>;
}