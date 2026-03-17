#[derive(Clone, Debug)]
pub struct CreateUserParams {
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}
