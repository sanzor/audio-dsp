use std::sync::Arc;
use crate::types::sample::Sample;
use super::graph_executor::GraphExecutor;

pub trait MixerProvider {
    fn mix(&self, input: Sample) -> Result<Sample, String>;
}

pub struct Mixer {
    executor: Arc<GraphExecutor>,
}

impl Mixer {
    pub fn new(executor: Arc<GraphExecutor>) -> Self {
        Self { executor }
    }
}

impl MixerProvider for Mixer {
    fn mix(&self, input: Sample) -> Result<Sample, String> {
        let mut out = Sample { data: [0.0; 128] };
        self.executor.process(&input.data, &mut out.data);
        Ok(out)
    }
}
