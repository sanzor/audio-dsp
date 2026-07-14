use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::{
    consumer::consumer::Consumer,
    events::ticket_created_event::TicketCreatedEvent,
    processor::{process_params::ProcessParams, processor::Processor},
    worker_config::WorkerConfig,
    worker_params::WorkerParams,
};

pub struct Worker {
    consumer: Box<dyn Consumer<TicketCreatedEvent>>,
    processor: Processor,
    config: WorkerConfig,
    token: CancellationToken,
}


impl Worker {
    pub fn new(params: WorkerParams, config: WorkerConfig, token: CancellationToken) -> Self {
        Self {
            consumer: params.consumer,
            processor: params.processor,
            config,
            token,
        }
    }

    pub fn spawn(
        params: WorkerParams,
        config: WorkerConfig,
        token: CancellationToken,
    ) -> JoinHandle<Result<(), String>> {
        let worker = Self::new(params, config, token);
        tokio::spawn(async move { worker.run_loop().await })
    }

    async fn run_loop(mut self) -> Result<(), String> {
        loop {
            tokio::select! {
                _ = self.token.cancelled() => {
                    tracing::info!("worker shutting down");
                    return Ok(());
                }
                res = self.consumer.consume() => {
                    match res {
                        Ok(event) => {
                            if let Err(e) = self.processor.process(ProcessParams { event }).await {
                                tracing::error!(error = %e, "processor error");
                            }
                        }
                        Err(e) => {
                            tracing::error!(error = %e, "consume error");
                            tokio::time::sleep(self.config.throttle_duration).await;
                        }
                    }
                }
            }
        }
    }
}
