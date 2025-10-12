use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}
