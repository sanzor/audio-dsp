use crate::dtos::claims::Claims;

pub trait JwtProvider: Send + Sync {
    fn issue_user_token(&self, user_id: domain::domain_user::UserId, name: Option<&str>, email: Option<&str>, is_admin: bool) -> Result<String, String>;
    fn issue_verification_token(&self, user_id: domain::domain_user::UserId) -> Result<String, String>;
    fn issue_invite_token(&self, invitee_email: &str, workspace_id: i32, role: &str) -> Result<String, String>;
    fn verify(&self, token: &str) -> Result<Claims, String>;
}
