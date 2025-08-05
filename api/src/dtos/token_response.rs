use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    pub id_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: Option<String>,
}
