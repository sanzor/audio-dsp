use super::user::AuthUser;

#[derive(Clone, Debug)]
pub struct RegisterUserResult {
    pub user: AuthUser,
    pub token: String,
    pub email_sent_note: Option<String>,
}
