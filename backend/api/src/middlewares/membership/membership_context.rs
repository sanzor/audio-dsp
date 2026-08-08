use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use std::{future::{ready, Ready}, ops::Deref};

use domain::project_role::ProjectRole;

#[derive(Debug, Clone)]
pub struct RoleContext(pub ProjectRole);

impl Deref for RoleContext {
    type Target = ProjectRole;
    fn deref(&self) -> &ProjectRole {
        &self.0
    }
}

impl FromRequest for RoleContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<RoleContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized("No role context"))),
        }
    }
}

/// The validated project_id for this request, inserted by the membership middleware.
#[derive(Debug, Clone, Copy)]
pub struct ProjectContext(pub i32);

impl FromRequest for ProjectContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<ProjectContext>() {
            Some(ctx) => ready(Ok(*ctx)),
            None => ready(Err(actix_web::error::ErrorBadRequest("No project context"))),
        }
    }
}
