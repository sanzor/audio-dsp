use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}
