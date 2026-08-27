use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use std::{
    future::{ready, Ready},
    sync::Arc,
};

use crate::transforms::transforms_provider::TransformsProvider;

use super::transform_access_middleware_service::TransformAccessMiddlewareService;

/// Loads the caller's `TransformAccessContext` once per request — wrap on
/// any scope whose handlers call `transform_authz::require_access` or
/// `transform_draft_authz::require_access` (both drafts and published
/// transforms share the same underlying row id, so one context serves both).
/// Must be wrapped inside `JwtAuthMiddleware` (i.e. added after it in the
/// `.wrap()` chain) since it reads the `JwtContext` that deposits.
#[derive(Clone)]
pub struct TransformAccessMiddleware {
    pub transforms_service: Arc<dyn TransformsProvider>,
}

impl<S, B> Transform<S, ServiceRequest> for TransformAccessMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Transform = TransformAccessMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TransformAccessMiddlewareService {
            service: std::rc::Rc::new(service),
            transforms_service: Arc::clone(&self.transforms_service),
        }))
    }
}
