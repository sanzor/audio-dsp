use crate::user::User;

pub struct DomainUser {
    id: String,
    name: String,
}

impl User for DomainUser {
    fn id(&self) -> String {
        self.id.to_string()
    }
    fn name(&self) -> String {
        self.name.to_string()
    }
}
