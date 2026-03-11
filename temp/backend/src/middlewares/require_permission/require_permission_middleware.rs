use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use super::require_permission_middleware_service::RequirePermissionMiddlewareService;

/// Wrap individual routes to enforce a specific permission.
///
/// ```rust
/// web::resource("/")
///     .route(web::post().to(create_role))
///     .wrap(RequirePermission::new("roles:create"))
/// ```
#[derive(Clone)]
pub struct RequirePermission {
    permission: &'static str,
}

impl RequirePermission {
    pub fn new(permission: &'static str) -> Self {
        Self { permission }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequirePermission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequirePermissionMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequirePermissionMiddlewareService::new(
            Rc::new(service),
            self.permission,
        )))
    }
}
