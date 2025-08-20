use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub roles: Option<Vec<String>>,
    pub exp: usize,
}
