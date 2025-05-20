use super::CommandChannel;

pub struct SimpleChannel{

}

impl<C:Send+'static> CommandChannel<C> for std::sync::mpsc::Sender<C>{
    fn send(&self,command:C)->Result<(),String> {
        self.send(command).map_err(|e|e.to_string())
    }
}