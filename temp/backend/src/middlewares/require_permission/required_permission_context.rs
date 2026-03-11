use actix_web::{FromRequest, HttpMessage};
use futures_util::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct RequiredPermissionContext {
    pub permission: &'static str,
}

impl FromRequest for RequiredPermissionContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<RequiredPermissionContext>() {
            Some(ctx) => ready(Ok(ctx.clone())),
            None => ready(Err(actix_web::error::ErrorForbidden(
                "Required permission context missing",
            ))),
        }
    }
}
