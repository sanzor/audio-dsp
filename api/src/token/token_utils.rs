use std::env;


use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use base64::{engine::general_purpose, Engine};

use crate::dtos::claims::Claims;

pub(crate) fn create_access_token(
    user_id: &str,
    name:Option<&str>,
    email: Option<&str>,
    roles: Option<Vec<String>>,
) -> String {
    create_token(user_id, email,name, roles, 15)
}

pub(crate) fn create_refresh_token(user_id: &str) -> String {
    create_token(user_id, None,None, None, 60 * 24 * 7)
}

pub(crate) fn generate_csrf_token() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    general_purpose::STANDARD_NO_PAD.encode(&bytes)
}
pub(crate) fn create_token(
    user_id: &str,
    email: Option<&str>,
    name:Option<&str>,
    roles: Option<Vec<String>>,
    minutes: i64,
) -> String {
    let secret = std::env::var("JWT_SECRET").expect("Missing JWT_SECRET in env");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(minutes))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_owned(),
        exp: expiration,
        name: name.map(|e|e.to_owned()),
        email: email.map(|e| e.to_owned()),
        roles: roles,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to encode JWT")
}

pub(crate) fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
