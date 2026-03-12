#[derive(Clone, Debug)]
pub struct CreateDomainUserParams {
    pub google_sub_id: Option<String>,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub password_hash: Option<String>,
}
