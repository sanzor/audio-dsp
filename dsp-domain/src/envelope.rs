use std::sync::mpsc::Sender;

use crate::{message_result::MessageResult, message::Message};

pub struct Caller(Sender<MessageResult>);
pub struct Envelope {
    pub command: Message,
    pub from: Option<Caller>,
}
