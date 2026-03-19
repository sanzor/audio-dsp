use crate::dtos::claims::Claims;

pub trait JwtProvider: Send + Sync {
    fn issue_user_token(&self, user_id: i64, name: Option<&str>, email: Option<&str>, is_admin: bool) -> Result<String, String>;
    fn issue_verification_token(&self, user_id: i64) -> Result<String, String>;
    fn issue_invite_token(&self, invitee_email: &str, project_id: i64, role: &str) -> Result<String, String>;
    fn verify(&self, token: &str) -> Result<Claims, String>;
}
