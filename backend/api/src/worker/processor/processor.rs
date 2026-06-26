use std::sync::Arc;

use crate::transforms::data_provider::transforms_data_provider::TransformsDataProvider;

use super::{process_params::ProcessParams, process_result::ProcessResult};

pub struct Processor {
    data_provider: Arc<dyn TransformsDataProvider>,
}

impl Processor {
    pub fn new(data_provider: Arc<dyn TransformsDataProvider>) -> Self {
        Self { data_provider }
    }

    pub async fn process(&self, _params: ProcessParams) -> Result<ProcessResult, String> {
        todo!("implement: self.data_provider.update_ticket(...)")
    }
}
