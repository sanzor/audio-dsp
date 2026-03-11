use crate::{
    auth::user::User,
    domain::db::db_user::{self, UserId},
};

#[derive(Clone, Debug)]
pub struct LoginResult {
    pub user: User,
    pub token: String,
    pub mfa_required: bool,
}
