use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{
    domain::db::{db_organization::OrganizationId, db_user::UserId},
    middlewares::jwt::{
        jwt_provider::JwtProvider,
        portal_jwt_claims::{PortalJwtClaims, TokenPurpose},
    },
};

pub struct Hs256JwtProvider {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: Option<String>,
    audience: Option<String>,
    access_token_ttl_seconds: i64,
    verification_token_ttl_seconds: i64,
}

impl Hs256JwtProvider {
    pub fn new(
        jwt_secret: &str,
        access_token_ttl_seconds: u64,
        verification_token_ttl_seconds: u64,
        issuer: Option<String>,
        audience: Option<String>,
    ) -> Result<Self, String> {
        let secret = jwt_secret.trim();
        if secret.is_empty() {
            return Err("jwt_secret is required".to_string());
        }

        Ok(Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            issuer,
            audience,
            access_token_ttl_seconds: i64::try_from(access_token_ttl_seconds)
                .map_err(|_| "access_token_ttl_seconds out of range".to_string())?,
            verification_token_ttl_seconds: i64::try_from(verification_token_ttl_seconds)
                .map_err(|_| "verification_token_ttl_seconds out of range".to_string())?,
        })
    }

    fn issue(
        &self,
        user_id: UserId,
        org_id: Option<OrganizationId>,
        purpose: TokenPurpose,
        ttl: i64,
    ) -> Result<String, String> {
        let now = Utc::now().timestamp();
        let exp = now + ttl;

        let claims = PortalJwtClaims {
            user_id,
            org_id,
            purpose,
            iat: now,
            exp,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };

        jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| e.to_string())
    }
}

impl JwtProvider for Hs256JwtProvider {
    fn issue_user_token(&self, user_id: UserId) -> Result<String, String> {
        self.issue(
            user_id,
            None,
            TokenPurpose::Access,
            self.access_token_ttl_seconds,
        )
    }

    fn issue_org_token(&self, user_id: UserId, org_id: OrganizationId) -> Result<String, String> {
        self.issue(
            user_id,
            Some(org_id),
            TokenPurpose::Access,
            self.access_token_ttl_seconds,
        )
    }

    fn issue_verification_token(&self, user_id: UserId) -> Result<String, String> {
        self.issue(
            user_id,
            None,
            TokenPurpose::Verification,
            self.verification_token_ttl_seconds,
        )
    }

    fn verify(&self, token: &str) -> Result<PortalJwtClaims, String> {
        let mut validation = Validation::new(Algorithm::HS256);

        if let Some(iss) = &self.issuer {
            validation.set_issuer(&[iss]);
        }
        if let Some(aud) = &self.audience {
            validation.set_audience(&[aud]);
        }

        jsonwebtoken::decode::<PortalJwtClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| e.to_string())
    }
}
