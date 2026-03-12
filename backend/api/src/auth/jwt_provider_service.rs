use crate::{
    dtos::claims::Claims,
    token::token_utils::{
        create_access_token, create_invite_token, create_project_token,
        create_verification_token, verify_token,
    },
};
use super::jwt_provider::JwtProvider;

pub struct JwtProviderService;

impl JwtProvider for JwtProviderService {
    fn issue_user_token(&self, user_id: &str, name: Option<&str>, email: Option<&str>, is_admin: bool) -> Result<String, String> {
        Ok(create_access_token(user_id, name, email, is_admin))
    }

    fn issue_verification_token(&self, user_id: &str) -> Result<String, String> {
        Ok(create_verification_token(user_id))
    }

    fn issue_invite_token(&self, user_id: &str, project_id: &str, role: &str) -> Result<String, String> {
        Ok(create_invite_token(user_id, project_id, role))
    }

    fn issue_project_token(&self, user_id: &str, name: Option<&str>, email: Option<&str>, is_admin: bool, project_id: &str) -> Result<String, String> {
        Ok(create_project_token(user_id, name, email, is_admin, project_id))
    }

    fn verify(&self, token: &str) -> Result<Claims, String> {
        verify_token(token).map_err(|e| e.to_string())
    }
}
