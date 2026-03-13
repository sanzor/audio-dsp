use actix_web::{Error, FromRequest, HttpMessage as _, HttpRequest, dev::Payload};
use domain::project_role::ProjectRole;
use std::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct JwtContext {
    pub user_id: String,
    pub project_id: Option<String>,
    pub is_admin: bool,
    pub role: Option<ProjectRole>,
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
