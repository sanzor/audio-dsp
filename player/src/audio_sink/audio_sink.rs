use std::{future::Future, pin::Pin};

use crate::AudioFrame;
#[async_trait::async_trait]
pub trait AudioSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: &'a AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
}
