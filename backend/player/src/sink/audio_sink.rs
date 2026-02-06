use std::{future::Future, pin::Pin};

use crate::AudioFrame;

pub trait AudioSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
}
