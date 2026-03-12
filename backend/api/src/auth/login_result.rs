use super::user::AuthUser;

#[derive(Clone, Debug)]
pub struct LoginResult {
    pub user: AuthUser,
    pub token: String,
}
