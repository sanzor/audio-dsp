use crate::user::User;

#[derive(Clone)]
pub struct DomainUser {
    pub id: i64,
    pub google_sub_id: Option<String>,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub password_hash: Option<String>,
    pub is_verified: bool,
}

impl User for DomainUser {
    fn id(&self) -> i64 { self.id }
    fn name(&self) -> &str { &self.name }
    fn email(&self) -> &str { &self.email }
}
