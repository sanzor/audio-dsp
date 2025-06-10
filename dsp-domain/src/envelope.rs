use std::sync::mpsc::Sender;

use crate::{dsp_message::DspMessage, tracks_message_result::TracksMessageResult};

pub struct Caller(Sender<TracksMessageResult>);
pub struct Envelope {
    pub command: DspMessage,
    pub from: Option<Caller>,
}
