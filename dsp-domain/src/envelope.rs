use std::sync::mpsc::Sender;

use crate::{dsp_command::DspCommand, dsp_command_result::DspCommandResult};

pub struct Caller(Sender<DspCommandResult>);
pub struct Envelope {
    pub command: DspCommand,
    pub from: Option<Caller>,
}
