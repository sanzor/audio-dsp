use std::collections::VecDeque;
use std::sync::Arc;

use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::{self, AsyncCommands};
use rust_shared::rate_limit_config::RateLimitConfig;

use crate::domain::db::db_organization::OrganizationId;
use crate::jobs::usage_worker::usage_worker_processor_params::UsageWorkerProcessorParams;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;

pub struct UsageWorkerProcessor {
    conn: MultiplexedConnection,
    usage_data_provider: Arc<dyn UsageDataProvider>,
}

impl UsageWorkerProcessor {
    pub fn new(params: UsageWorkerProcessorParams) -> Self {
        Self {
            conn: params.conn,
            usage_data_provider: params.usage_data_provider,
        }
    }

    pub async fn update(&self) -> Result<(), String> {
        let mut stream = self.get_stream();
        let mut data_conn = self.conn.clone();

        while let Some(key) = stream.next().await? {
            if let Err(e) = self.process_key(&mut data_conn, &key).await {
                tracing::warn!("Usage worker failed to process redis key {}: {}", key, e);
            }
        }
        Ok(())
    }

    fn get_stream(&self) -> OrganizationKeyStream {
        OrganizationKeyStream::new(self.conn.clone(), "organization:*".to_string(), 250)
    }

    async fn process_key(&self, conn: &mut MultiplexedConnection, key: &str) -> Result<(), String> {
        let org_id = match parse_org_id_from_key(key) {
            Some(id) => id,
            None => return Ok(()),
        };

        let Some(config) = self.fetch_config(conn, key).await? else {
            return Ok(());
        };

        let snapshot = fetch_usage_snapshot(conn, key, config).await?;

        self.usage_data_provider.upsert_subscription_usage(org_id, snapshot).await
    }

    async fn fetch_config(
        &self,
        conn: &mut MultiplexedConnection,
        key: &str,
    ) -> Result<Option<RateLimitConfig>, String> {
        let config_raw: Option<String> =
            conn.hget(key, "config").await.map_err(|e| e.to_string())?;
        let Some(config_raw) = config_raw else {
            return Ok(None);
        };
        let config: RateLimitConfig =
            serde_json::from_str(&config_raw).map_err(|e| e.to_string())?;
        Ok(Some(config))
    }

}

fn parse_org_id_from_key(key: &str) -> Option<OrganizationId> {
    let id_str = key.strip_prefix("organization:")?;
    id_str.parse::<OrganizationId>().ok()
}

#[derive(Clone, Copy, Debug)]
pub enum RateLimitUsageSnapshot {
    TokenBucket {
        limit: u64,
        bucket_tokens: Option<i64>,
    },
    TokenWindow {
        limit: u64,
        window_size_secs: u64,
        window_counter: Option<i64>,
        window_start: Option<i64>,
    },
    Default,
}

async fn fetch_usage_snapshot(
    conn: &mut MultiplexedConnection,
    key: &str,
    config: RateLimitConfig,
) -> Result<RateLimitUsageSnapshot, String> {
    match config {
        RateLimitConfig::TokenBucket { limit } => {
            // `tokens` = remaining tokens in the Redis bank.
            // The scoring API atomically decrements this on each in-memory refill,
            // so the value here always reflects real-time availability.
            let bucket_tokens: Option<i64> = hget_i64(conn, key, "tokens").await?;
            Ok(RateLimitUsageSnapshot::TokenBucket { limit, bucket_tokens })
        }
        RateLimitConfig::TokenWindow {
            window_size_secs,
            limit,
        } => {
            let (raw_counter, raw_start): (Option<String>, Option<String>) = conn
                .hmget(key, ("window_counter", "window_start"))
                .await
                .map_err(|e| e.to_string())?;

            Ok(RateLimitUsageSnapshot::TokenWindow {
                limit,
                window_size_secs,
                window_counter: raw_counter.and_then(|v| v.parse::<i64>().ok()),
                window_start: raw_start.and_then(|v| v.parse::<i64>().ok()),
            })
        }
        RateLimitConfig::Default => Ok(RateLimitUsageSnapshot::Default),
    }
}

async fn hget_i64(
    conn: &mut MultiplexedConnection,
    key: &str,
    field: &str,
) -> Result<Option<i64>, String> {
    let raw: Option<String> = conn.hget(key, field).await.map_err(|e| e.to_string())?;
    Ok(raw.and_then(|v| v.parse::<i64>().ok()))
}

struct OrganizationKeyStream {
    conn: MultiplexedConnection,
    cursor: u64,
    pattern: String,
    count: u32,
    buffer: VecDeque<String>,
    started: bool,
}

impl OrganizationKeyStream {
    fn new(conn: MultiplexedConnection, pattern: String, count: u32) -> Self {
        Self {
            conn,
            cursor: 0,
            pattern,
            count,
            buffer: VecDeque::new(),
            started: false,
        }
    }

    async fn next(&mut self) -> Result<Option<String>, String> {
        loop {
            if let Some(item) = self.buffer.pop_front() {
                return Ok(Some(item));
            }

            if self.started && self.cursor == 0 {
                return Ok(None);
            }

            self.started = true;
            let (next_cursor, keys) = redis::cmd("SCAN")
                .arg(self.cursor)
                .arg("MATCH")
                .arg(&self.pattern)
                .arg("COUNT")
                .arg(self.count)
                .query_async::<(u64, Vec<String>)>(&mut self.conn)
                .await
                .map_err(|e| e.to_string())?;

            self.cursor = next_cursor;
            self.buffer.extend(keys);
        }
    }
}
