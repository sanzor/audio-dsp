use std::sync::Arc;

use deadpool_redis::redis::aio::MultiplexedConnection;

use crate::usage::data_provider::usage_data_provider::UsageDataProvider;

pub struct UsageWorkerParams {
    pub conn: MultiplexedConnection,
    pub usage_data_provider: Arc<dyn UsageDataProvider>,
}
