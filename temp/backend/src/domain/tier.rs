use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Free,
    Standard,
    Premium,
    BankLevel,
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tier::Free => write!(f, "free"),
            Tier::Standard => write!(f, "standard"),
            Tier::Premium => write!(f, "premium"),
            Tier::BankLevel => write!(f, "bank_level"),
        }
    }
}

impl FromStr for Tier {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free" => Ok(Tier::Free),
            "standard" => Ok(Tier::Standard),
            "premium" => Ok(Tier::Premium),
            "bank_level" => Ok(Tier::BankLevel),
            _ => Err(format!("invalid tier: '{s}'")),
        }
    }
}

// ── sqlx support (tier stored as TEXT in Postgres) ────────────────────────────

impl sqlx::Type<sqlx::Postgres> for Tier {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Tier {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        s.parse::<Tier>().map_err(|e| e.into())
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Tier {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <String as sqlx::Encode<sqlx::Postgres>>::encode(self.to_string(), buf)
    }
}
