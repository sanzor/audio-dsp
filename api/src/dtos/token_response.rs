use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    pub _id_token: Option<String>,
    pub _expires_in: u64,
    pub _token_type: String,
    pub _refresh_token: Option<String>,
}
