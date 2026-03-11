use actix_web::{FromRequest, HttpMessage};
use futures_util::future::{ready, Ready};

use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Copy, Debug)]
pub struct JWTContext {
    pub user_id: UserId,
    pub org_id: Option<OrganizationId>,
}

impl FromRequest for JWTContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<JWTContext>() {
            Some(&auth) => ready(Ok(auth.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "Auth context missing",
            ))),
        }
    }
}
