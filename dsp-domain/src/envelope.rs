use std::sync::mpsc::Sender;

use crate::{dsp_message_result::DspMessageResult, dsp_message::DspMessage};

pub struct Caller(Sender<DspMessageResult>);
pub struct Envelope {
    pub command: DspMessage,
    pub from: Option<Caller>,
}
