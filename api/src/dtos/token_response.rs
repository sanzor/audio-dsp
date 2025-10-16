use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    #[serde(rename = "id_token")]
    pub _id_token: Option<String>,
    #[serde(rename = "expires_in")]
    pub _expires_in: u64,
    #[serde(rename = "token_type")]
    pub _token_type: String,
    #[serde(rename = "refresh_token")]
    pub _refresh_token: Option<String>,
}
