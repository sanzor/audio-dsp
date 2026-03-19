use std::env;

use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;

use crate::dtos::claims::Claims;

pub(crate) fn create_access_token(
    user_id: i64,
    name: Option<&str>,
    email: Option<&str>,
    is_admin: bool,
) -> String {
    create_token(user_id, email, name, is_admin, None, None, None, 15)
}


pub(crate) fn create_refresh_token(user_id: i64) -> String {
    create_token(user_id, None, None, false, None, Some("refresh"), None, 60 * 24 * 7)
}

pub(crate) fn create_verification_token(user_id: i64) -> String {
    create_token(user_id, None, None, false, None, Some("verification"), None, 60 * 24)
}

/// Invite token embeds the invitee's email in the `email` field.
/// The `user_id` field is set to 0 because the invitee may not have an account yet.
pub(crate) fn create_invite_token(invitee_email: &str, project_id: i64, role: &str) -> String {
    create_token(0, Some(invitee_email), None, false, Some(project_id), Some("invite"), Some(role), 60 * 24 * 7)
}

pub(crate) fn generate_csrf_token() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    general_purpose::STANDARD_NO_PAD.encode(bytes)
}

pub(crate) fn create_token(
    user_id: i64,
    email: Option<&str>,
    name: Option<&str>,
    is_admin: bool,
    project_id: Option<i64>,
    purpose: Option<&str>,
    role: Option<&str>,
    minutes: i64,
) -> String {
    let secret = std::env::var("JWT_SECRET").expect("Missing JWT_SECRET in env");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(minutes))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        exp: expiration,
        name: name.map(|e| e.to_owned()),
        email: email.map(|e| e.to_owned()),
        is_admin,
        project_id,
        purpose: purpose.map(|p| p.to_owned()),
        role: role.map(|r| r.to_owned()),
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

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            println!("JWT decode error: {:?}", e);
            Err(e)
        }
    }
}
