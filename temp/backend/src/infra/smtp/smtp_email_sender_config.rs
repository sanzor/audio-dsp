use serde::Deserialize;

fn default_smtp_port() -> u16 {
    587
}

fn default_smtp_enabled() -> bool {
    false
}

fn default_use_starttls() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpEmailSenderConfig {
    #[serde(default = "default_smtp_enabled")]
    pub enabled: bool,
    pub host: String,
    #[serde(default = "default_smtp_port")]
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from_address: String,
    pub verification_base_url: String,
    #[serde(default = "default_use_starttls")]
    pub use_starttls: bool,
}
