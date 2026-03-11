use serde::{Deserialize, Serialize};

use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenPurpose {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "verification")]
    Verification,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortalJwtClaims {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganizationId>,
    #[serde(default = "default_purpose")]
    pub purpose: TokenPurpose,
    pub iat: i64,
    pub exp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
}

fn default_purpose() -> TokenPurpose {
    TokenPurpose::Access
}
