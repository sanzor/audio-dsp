use serde::Deserialize;

fn default_upload_limit_mb() -> usize {
    100
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_upload_limit_mb")]
    pub upload_limit_mb: usize,
}

impl AppConfig {
    pub fn upload_limit_bytes(&self) -> usize {
        self.upload_limit_mb * 1024 * 1024
    }
}

pub fn load_app_config() -> Result<AppConfig, config::ConfigError> {
    let env_name = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    let base_path = std::env::var("APP_CONFIG_PATH")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
        .unwrap_or_else(|_| ".".into());

    tracing::info!("Loading config from base path: {base_path}, env: {env_name}");

    let settings = config::Config::builder()
        .add_source(
            config::File::with_name(&format!("{base_path}/config/{env_name}/settings"))
                .required(true),
        )
        .add_source(
            config::File::with_name(&format!("{base_path}/config/local")).required(false),
        )
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    settings.try_deserialize()
}
