use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use std::{future::{ready, Ready}, ops::Deref};

use domain::workspace_role::WorkspaceRole;

#[derive(Debug, Clone)]
pub struct RoleContext(pub WorkspaceRole);

impl Deref for RoleContext {
    type Target = WorkspaceRole;
    fn deref(&self) -> &WorkspaceRole {
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

/// The validated workspace_id for this request, inserted by the membership middleware.
#[derive(Debug, Clone, Copy)]
pub struct WorkspaceContext(pub i32);

impl FromRequest for WorkspaceContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<WorkspaceContext>() {
            Some(ctx) => ready(Ok(*ctx)),
            None => ready(Err(actix_web::error::ErrorBadRequest("No workspace context"))),
        }
    }
}
