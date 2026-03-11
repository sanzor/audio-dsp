use crate::auth::user::User;

#[derive(Clone, Debug)]
pub struct RegisterUserResult {
    pub user: User,
    pub token: String,
    pub mfa_required: bool,
    pub mfa_secret: Option<String>,
    pub email_sent_note: Option<String>,
}
