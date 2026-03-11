use crate::{
    domain::db::{db_organization::OrganizationId, db_user::UserId},
    middlewares::jwt::portal_jwt_claims::PortalJwtClaims,
};

pub trait JwtProvider: Send + Sync {
    fn issue_user_token(&self, user_id: UserId) -> Result<String, String>;
    fn issue_org_token(&self, user_id: UserId, org_id: OrganizationId) -> Result<String, String>;
    fn issue_verification_token(&self, user_id: UserId) -> Result<String, String>;
    fn verify(&self, token: &str) -> Result<PortalJwtClaims, String>;
}
