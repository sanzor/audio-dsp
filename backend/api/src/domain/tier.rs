use serde::{Deserialize, Serialize};
use sqlx::{Type, postgres::{PgTypeInfo, PgValueRef}, Decode, Encode};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Tier {
    Free,
    Standard,
    Premium,
    BankLevel,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Free => write!(f, "Free"),
            Tier::Standard => write!(f, "Standard"),
            Tier::Premium => write!(f, "Premium"),
            Tier::BankLevel => write!(f, "BankLevel"),
        }
    }
}

impl std::str::FromStr for Tier {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Free" => Ok(Tier::Free),
            "Standard" => Ok(Tier::Standard),
            "Premium" => Ok(Tier::Premium),
            "BankLevel" => Ok(Tier::BankLevel),
            other => Err(format!("unknown tier: {other}")),
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for Tier {
    fn type_info() -> PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Tier {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = <String as Decode<sqlx::Postgres>>::decode(value)?;
        s.parse::<Tier>().map_err(|e| Box::new(sqlx::Error::Decode(e.into())) as Box<dyn std::error::Error + Send + Sync>)
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Tier {
    fn encode_by_ref(&self, buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        <String as Encode<sqlx::Postgres>>::encode_by_ref(&self.to_string(), buf)
    }
}
