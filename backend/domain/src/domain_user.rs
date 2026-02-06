use crate::user::User;

#[derive(Clone)]
pub struct DomainUser {
    pub id: String,
    pub google_sub_id: Option<String>,
    pub name: String,
    pub email: String,
    pub picture: String,
}

impl User for DomainUser {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }
}
