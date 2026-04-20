use async_trait::async_trait;
use crate::types::sample::Sample;

#[async_trait]
pub trait SinkProvider {
    async fn send(&self, sample: Sample) -> Result<(), String>;
}

pub struct Sink;

#[async_trait]
impl SinkProvider for Sink {
    async fn send(&self, _sample: Sample) -> Result<(), String> {
        // write to audio output device / speaker
        Ok(())
    }
}
