use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMode {
    Subscription,
    TokenBucket,
}

impl BillingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMode::Subscription => "subscription",
            BillingMode::TokenBucket => "token_bucket",
        }
    }
}

impl TryFrom<&str> for BillingMode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "subscription" => Ok(BillingMode::Subscription),
            "token_bucket" => Ok(BillingMode::TokenBucket),
            other => Err(format!("unknown billing mode: {}", other)),
        }
    }
}
