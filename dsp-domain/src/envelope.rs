use std::sync::mpsc::Sender;

use crate::command::{CommandResult, DspCommand};

pub struct Caller(Sender<CommandResult>);
pub struct Envelope {
    pub command: DspCommand,
    pub from: Option<Caller>,
}
