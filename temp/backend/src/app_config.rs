use common::{
    common_config::{HasRedisConfig, RedisSettings},
    telemetry::{has_telemetry_config::HasTelemetryConfig, telemetry_config::TelemetryConfig},
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub address: SocketAddr,
    pub worker_count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UsageWorkerSettings {
    pub update_every_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SyncWorkerSettings {
    pub update_every_seconds: u64,
}

fn default_smtp_port() -> u16 {
    587
}

fn default_smtp_enabled() -> bool {
    false
}

fn default_smtp_use_starttls() -> bool {
    true
}

fn default_access_token_ttl_seconds() -> u64 {
    60 * 60
}

fn default_verification_token_ttl_seconds() -> u64 {
    24 * 60 * 60
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_secret: String,
    #[serde(default = "default_access_token_ttl_seconds")]
    pub access_token_ttl_seconds: u64,
    #[serde(default = "default_verification_token_ttl_seconds")]
    pub verification_token_ttl_seconds: u64,
    pub issuer: Option<String>,
    pub audience: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpSettings {
    #[serde(default = "default_smtp_enabled")]
    pub enabled: bool,
    pub host: String,
    #[serde(default = "default_smtp_port")]
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from_address: String,
    pub verification_base_url: String,
    #[serde(default = "default_smtp_use_starttls")]
    pub use_starttls: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StripeSettings {
    pub secret_key: String,
    pub webhook_secret: String,
    /// Base URL of this portal (e.g. "https://app.example.com").
    /// Used to build success/cancel redirect URLs.
    pub portal_url: String,
}

fn default_token_bucket_size() -> u64 {
    20
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrganizationSettings {
    #[serde(default = "default_token_bucket_size")]
    pub default_token_bucket_size: u64,
}

impl Default for OrganizationSettings {
    fn default() -> Self {
        Self {
            default_token_bucket_size: default_token_bucket_size(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerSettings,
    pub redis: RedisSettings,
    pub database: DatabaseSettings,
    pub usage_worker: UsageWorkerSettings,
    pub sync_worker: SyncWorkerSettings,
    pub auth: AuthSettings,
    pub smtp: SmtpSettings,
    pub stripe: StripeSettings,
    pub telemetry_config: TelemetryConfig,
    #[serde(default)]
    pub organization: OrganizationSettings,
}

impl HasTelemetryConfig for AppConfig {
    fn telemetry_config(&self) -> &TelemetryConfig {
        &self.telemetry_config
    }
}

impl HasRedisConfig for AppConfig {
    fn redis_config(&self) -> &RedisSettings {
        &self.redis
    }
}
