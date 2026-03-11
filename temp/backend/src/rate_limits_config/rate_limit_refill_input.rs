use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RateLimitRefillInput {
    BucketAdd { amount: u64 },
    WindowReset,
    WindowRefund { amount: u64 },
    BucketReset
}
