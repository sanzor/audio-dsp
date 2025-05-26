use std::sync::mpsc::Sender;

use crate::{dsp_command_result::DspCommandResult, message::Message};

pub struct Caller(Sender<DspCommandResult>);
pub struct Envelope {
    pub command: Message,
    pub from: Option<Caller>,
}
