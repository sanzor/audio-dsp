use actix_web::{body::EitherBody, dev::{Service, ServiceRequest, ServiceResponse, Transform}};
use std::future::{ready, Ready};

use super::jwt_middleware_service::JwtAuthMiddlewareService;

#[derive(Clone)]
pub struct JwtAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Transform = JwtAuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddlewareService {
            service: std::rc::Rc::new(service),
        }))
    }
}
