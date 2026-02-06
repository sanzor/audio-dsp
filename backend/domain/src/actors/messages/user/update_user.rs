use serde::{Deserialize, Serialize};

pub struct UpdateUser {
    pub id: String,
    pub email: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateUserResult {
    pub id: String,
    pub new_email: String,
    pub new_name: String,
}
