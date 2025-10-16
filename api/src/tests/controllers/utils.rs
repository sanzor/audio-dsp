#[cfg(test)]
use actix_web::cookie::Cookie;
use jsonwebtoken::encode;
#[cfg(test)]
pub fn make_test_auth_cookie() -> Cookie<'static> {
    use jsonwebtoken::{EncodingKey, Header};

    use crate::dtos::claims::Claims;

    let claims = Claims {
        user_id: "test-user-id".into(),
        name: Some("tester".into()),
        email: Some("test@example.com".into()),
        roles: Some(vec!["tester".into()]),
        exp: 2_000_000_000, // far future expiration
    };

    // ⚠️ Use the same key you use in prod .env (or a fixed test one)
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"test-secret"),
    )
    .unwrap();

    Cookie::build("auth_token", token)
        .path("/")
        .http_only(false)
        .finish()
}
