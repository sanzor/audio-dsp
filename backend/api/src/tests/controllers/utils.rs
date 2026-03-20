use std::sync::Arc;

#[cfg(test)]
use actix_web::cookie::Cookie;
use crate::users::user_provider::UserProvider;
use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};
use jsonwebtoken::encode;
use kameo::actor::ActorRef;
#[cfg(test)]
pub fn make_test_auth_cookie(secret: String, user_id: i64) -> Cookie<'static> {
    use jsonwebtoken::{EncodingKey, Header};

    use crate::dtos::claims::Claims;

    let claims = Claims {
        user_id,
        name: Some("tester".into()),
        email: Some("test@example.com".into()),
        is_admin: false,
        project_id: None,
        purpose: Some("access".into()),
        role: None,
        exp: 2_000_000_000, // far future expiration
    };

    // ⚠️ Use the same key you use in prod .env (or a fixed test one)
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    Cookie::build("auth_token", token)
        .path("/")
        .http_only(false)
        .finish()
}

static INIT: std::sync::Once = std::sync::Once::new();
pub fn init_env() {
    INIT.call_once(|| {
        dotenvy::from_filename(concat!(env!("CARGO_MANIFEST_DIR"), "/dev.env")).ok();
    });
}
