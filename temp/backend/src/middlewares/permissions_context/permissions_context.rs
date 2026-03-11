use std::collections::HashSet;

use actix_web::{FromRequest, HttpMessage};
use futures_util::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct PermissionsContext {
    pub permissions: HashSet<String>,
}

impl PermissionsContext {
    pub fn has(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }
}

impl FromRequest for PermissionsContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<PermissionsContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorForbidden(
                "Permissions context missing",
            ))),
        }
    }
}
