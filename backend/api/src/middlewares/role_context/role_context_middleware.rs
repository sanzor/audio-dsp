use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use std::{future::{ready, Ready}, sync::Arc};

use crate::memberships::memberships_provider::MembershipsProvider;

use super::role_context_middleware_service::RoleContextMiddlewareService;

#[derive(Clone)]
pub struct RoleContextMiddleware {
    pub memberships: Arc<dyn MembershipsProvider>,
}

impl<S, B> Transform<S, ServiceRequest> for RoleContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Transform = RoleContextMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleContextMiddlewareService {
            service: std::rc::Rc::new(service),
            memberships: Arc::clone(&self.memberships),
        }))
    }
}
