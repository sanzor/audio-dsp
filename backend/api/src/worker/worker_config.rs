use std::time::Duration;

pub struct WorkerConfig {
    pub throttle_duration: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            throttle_duration: Duration::from_secs(1),
        }
    }
}
