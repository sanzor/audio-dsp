use std::{future::Future, pin::Pin};

use crate::AudioFrame;

use super::AudioSink;

pub struct StdSink {}
#[async_trait::async_trait]
impl AudioSink for StdSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            println!("{frame:?}");
            Ok(())
        })
    }
}
