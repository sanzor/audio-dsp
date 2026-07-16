use std::sync::Arc;

use crate::transforms::data_provider::transforms_data_provider::TransformsDataProvider;

use super::build_job::BuildJobConfig;

pub struct ProcessorParams {
    pub data_provider: Arc<dyn TransformsDataProvider>,
    pub build_job_config: BuildJobConfig,
    /// Fuel budget for the wasmtime instance used to call a compiled
    /// module's metadata export — bounds a malicious/broken `metadata()`.
    pub metadata_fuel_limit: u64,
}
