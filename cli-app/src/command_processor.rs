use crate::{command::{Command, CommandResult}, command_dispatch::CommandDispatch, dispatch_provider::DispatchProvider, envelope::Envelope, state::SharedState};



pub struct CommandProcessor{ 
    dispatch_provider:DispatchProvider,
    state:SharedState
}

impl CommandProcessor{
    pub fn new(dispatch_provider:DispatchProvider,state:SharedState)->CommandProcessor{
        CommandProcessor{dispatch_provider,state}
    }

    pub fn process_command(&mut self,input:Command)->Result<CommandResult,String>{
        let dispatcher_name=self.get_dispatcher_name(&input);
        let dispatcher=self.dispatch_provider.get_dispatcher_by_name(dispatcher_name)
                    .ok_or_else(||"Could not find dispatcher".to_string())?;
        let result=dispatcher.dispatch(Envelope{command:input,from:None}, self.state.clone());
        result
    }



    fn get_dispatcher_name(&self,command:&Command)->&'static str{
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
            Command::Normalize { name: _, mode }=>"normalize",
            Command::Upload { name, filename }=>"upload",
       }
    }
    
}