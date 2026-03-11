use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use crate::middlewares::jwt::jwt_middleware_service::JwtAuthMiddlewareService;
use crate::middlewares::jwt::jwt_provider::JwtProvider;

#[derive(Clone)]
pub struct JwtAuthMiddleware {
    jwt: Arc<dyn JwtProvider>,
}

impl JwtAuthMiddleware {
    pub fn new(jwt: Arc<dyn JwtProvider>) -> Self {
        Self { jwt }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddlewareService::new(
            Rc::new(service),
            self.jwt.clone(),
        )))
    }
}
