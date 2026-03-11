use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Mirrors the PostgreSQL enum `rate_limit_config_type`.
/// Variant names are PascalCase to match both the DB enum values
/// and the Rust `RateLimitConfig` serde serialization (rename_all = "PascalCase").
#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[sqlx(type_name = "rate_limit_config_type")]
pub enum RateLimitConfigType {
    TokenBucket,
    TokenWindow,
}
