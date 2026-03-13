use crate::dtos::claims::Claims;

pub trait JwtProvider: Send + Sync {
    fn issue_user_token(&self, user_id: &str, name: Option<&str>, email: Option<&str>, is_admin: bool) -> Result<String, String>;
    fn issue_verification_token(&self, user_id: &str) -> Result<String, String>;
    fn issue_invite_token(&self, user_id: &str, project_id: &str, role: &str) -> Result<String, String>;
    fn issue_project_token(&self, user_id: &str, name: Option<&str>, email: Option<&str>, is_admin: bool, project_id: &str, role: &str) -> Result<String, String>;
    fn verify(&self, token: &str) -> Result<Claims, String>;
}
