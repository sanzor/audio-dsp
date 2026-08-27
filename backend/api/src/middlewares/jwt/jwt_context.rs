use actix_web::{Error, FromRequest, HttpMessage as _, HttpRequest, dev::Payload};
use domain::domain_user::UserId;
use std::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct JwtContext {
    pub user_id: UserId,
    pub is_admin: bool,
}

impl FromRequest for JwtContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<JwtContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}
