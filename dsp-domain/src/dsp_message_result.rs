use actix::Message;
use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct DspMessageResult {
    pub output: String,
    pub should_exit: bool,
}
