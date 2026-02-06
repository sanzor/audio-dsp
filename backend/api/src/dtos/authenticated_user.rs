use actix::fut::{ready, Ready};
use actix_web::{Error, FromRequest};

use crate::token::token_utils::verify_token;

#[derive(utoipa::ToSchema)]
pub struct AuthenticatedUser {
    pub user_id: String,
    // Optionally:
    pub email: Option<String>,      // if included in token
    pub roles: Option<Vec<String>>, // for RBAC
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let cookie = match req.cookie("auth_token") {
            Some(c) => c,
            None => return ready(Err(actix_web::error::ErrorUnauthorized("No auth token"))),
        };
        let claims = match verify_token(cookie.value()) {
            Err(_e) => return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
            Ok(tk) => tk,
        };

        ready(Ok(AuthenticatedUser {
            email: claims.email,
            roles: claims.roles,
            user_id: claims.user_id,
        }))
    }
}
