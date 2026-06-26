pub struct Worker{
    config:WorkerConfig,
    consumer: Box<dyn Consumer<TicketCreatedEvent>>,
}
#[async_trait]
impl Worker{
    pub fn new(params:WorkerParams,token:CancellationToken)->Self{

    }
    pub async fn spawn_worker(
        &self,
        params:WorkerParams,
        token:CancellationToken,
        config:WorkerConfig
    )->JoinHandle<Result<(),String>>{
        let worker=Self::new(config);
        tokio::spawn(async move{ worker.run_loop(token,params,config)})
    }
    pub async fn run_loop(&self,token:CancellationToken,params:WorkerParams,)->Result<(),String>{
        loop{
            let message=tokio::select!{
                _=tokio::time::sleep(duration)=>Ok(())
                cancel=token.cancel()=>{

                },
                res = self.consumer.consume() => match res {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::error!("Consume error: {}", e);
                        self.wait_for_cancellation(self.throttle_duration_seconds, &cancellation_token).await?;
                        continue;
                    }
                }

            };

        }
    }
}