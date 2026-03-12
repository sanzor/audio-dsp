#[derive(Clone, Debug)]
pub struct LoginParams {
    pub email: String,
    pub password_hash: String,
}
