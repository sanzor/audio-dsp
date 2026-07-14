use std::sync::Arc;

use crate::transforms::{data_provider::transforms_data_provider::TransformsDataProvider, storage_provider::transform_storage_provider::TransformStorageProvider};

pub struct ProcessorParams{
    pub data_provider: Arc<dyn TransformsDataProvider>,
    pub storage:Box<dyn TransformStorageProvider>
}