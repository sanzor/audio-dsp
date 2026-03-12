use super::user::AuthUser;

#[derive(Clone, Debug)]
pub struct VerifyUserResult {
    pub user: AuthUser,
}
