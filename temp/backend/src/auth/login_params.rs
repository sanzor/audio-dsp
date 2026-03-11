#[derive(Clone, Debug)]
pub struct LoginParams {
    pub email: String,
    pub password_hash: String,
    pub mfa_code: Option<String>,
}
