use std::sync::Arc;
use async_trait::async_trait;
use crate::types::sample::Sample;
use super::mixer::MixerProvider;
use super::sink::SinkProvider;

#[async_trait]
pub trait PlayerProvider {
    async fn play(&self, sample: Sample, with_effects: bool) -> Result<(), String>;
}

pub struct Player {
    mixer: Arc<dyn MixerProvider + Send + Sync>,
    sink: Arc<dyn SinkProvider + Send + Sync>,
}

impl Player {
    pub fn new(
        mixer: Arc<dyn MixerProvider + Send + Sync>,
        sink: Arc<dyn SinkProvider + Send + Sync>,
    ) -> Self {
        Self { mixer, sink }
    }
}

#[async_trait]
impl PlayerProvider for Player {
    async fn play(&self, sample: Sample, with_effects: bool) -> Result<(), String> {
        let out = if with_effects {
            self.mixer.mix(sample)?
        } else {
            sample
        };
        self.sink.send(out).await
    }
}
