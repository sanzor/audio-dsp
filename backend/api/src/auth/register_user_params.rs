#[derive(Clone, Debug)]
pub struct RegisterUserParams {
    pub email: String,
    pub password_hash: String,
    pub name: String,
}
