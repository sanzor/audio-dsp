use actix::fut::{ready, Ready};
use actix_web::{Error, FromRequest};

use crate::token::token_utils::verify_token;

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
        let result = req
            .cookie("auth_token")
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("No auth token"))
            .and_then(|cookie| {
                verify_token(cookie.value())
                    .map_err(|e| actix_web::error::ErrorUnauthorized("Invalid token"))
            })
            .map(|claims| AuthenticatedUser {
                email: claims.email,
                roles: claims.roles,
                user_id: claims.user_id,
            });
        ready(result)
    }
}
