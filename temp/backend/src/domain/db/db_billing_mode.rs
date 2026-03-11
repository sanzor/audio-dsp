use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct DbBillingModeConfig {
    pub org_id: i64,
    pub mode: String,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
