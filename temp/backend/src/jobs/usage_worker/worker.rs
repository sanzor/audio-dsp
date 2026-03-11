use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::jobs::usage_worker::{
    usage_worker_config::UsageWorkerConfig, usage_worker_params::UsageWorkerParams,
    usage_worker_processor::UsageWorkerProcessor,
    usage_worker_processor_params::UsageWorkerProcessorParams,
};

pub struct UsageWorker {
    update_interval_duration: Duration,
    processor: UsageWorkerProcessor,
}

impl UsageWorker {
    pub fn spawn_worker(
        params: UsageWorkerParams,
        config: UsageWorkerConfig,
        token: CancellationToken,
    ) -> JoinHandle<Result<(), String>> {
        let worker = Self::new(config, params);
        let handle = tokio::spawn(async move { worker.run_loop(token).await });
        handle
    }
    fn new(config: UsageWorkerConfig, params: UsageWorkerParams) -> Self {
        let processor = UsageWorkerProcessor::new(UsageWorkerProcessorParams {
            conn: params.conn,
            usage_data_provider: params.usage_data_provider,
        });
        Self {
            update_interval_duration: config.update_every,
            processor,
        }
    }
    async fn run_loop(&self, token: CancellationToken) -> Result<(), String> {
        let mut interval = tokio::time::interval(self.update_interval_duration);
        loop {
            tokio::select! {
                _tick=interval.tick()=>{
                    if let Err(e) = self.processor.update().await {
                        tracing::error!("Usage worker update failed: {}", e);
                    } else {
                        tracing::info!("Usage worker updated successfully");
                    }
                }
                _cancel=token.cancelled()=>{
                    tracing::info!("Usage worker shutting down gracefully...");
                    return Ok(());
                }
            }
        }
    }
}
