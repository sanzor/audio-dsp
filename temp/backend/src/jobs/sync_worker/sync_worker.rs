use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::jobs::sync_worker::{
    sync_worker_config::SyncWorkerConfig, sync_worker_params::SyncWorkerParams,
    sync_worker_processor::SyncWorkerProcessor,
    sync_worker_processor_params::SyncWorkerProcessorParams,
    sync_worker_processor_service::SyncWorkerProcessorService,
};

pub struct SyncWorker {
    update_interval_duration: Duration,
    processor: Arc<dyn SyncWorkerProcessor>,
}

impl SyncWorker {
    pub fn spawn_worker(
        params: SyncWorkerParams,
        config: SyncWorkerConfig,
        token: CancellationToken,
    ) -> JoinHandle<Result<(), String>> {
        let worker = Self::new(config, params);
        tokio::spawn(async move { worker.run_loop(token).await })
    }

    fn new(config: SyncWorkerConfig, params: SyncWorkerParams) -> Self {
        let processor: Arc<dyn SyncWorkerProcessor> =
            Arc::new(SyncWorkerProcessorService::new(SyncWorkerProcessorParams {
                redis: params.redis,
                db_conn: params.db_conn,
            }));

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
                    if let Err(e) = self.processor.sync().await {
                        tracing::error!("Sync worker failed: {}", e);
                    } else {
                        tracing::info!("Sync worker completed successfully");
                    }
                }
                _cancel=token.cancelled()=>{
                    tracing::info!("Sync worker shutting down gracefully...");
                    return Ok(());
                }
            }
        }
    }
}
