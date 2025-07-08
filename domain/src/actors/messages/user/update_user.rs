use serde::Serialize;

pub struct UpdateUser {
    pub id: String,
    pub email: String,
    pub name: String,
}
#[derive(Serialize)]
pub struct UpdateUserResult {}
