use std::sync::mpsc::Sender;

use crate::command::{Command, CommandResult};


pub struct Caller(Sender<CommandResult>);
pub struct Envelope{
    pub command:Command,
    pub from:Option<Caller>
}