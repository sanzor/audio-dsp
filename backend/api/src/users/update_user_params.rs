#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}
