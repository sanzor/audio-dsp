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

use crate::organizations::organization_provider::OrganizationProvider;

use super::organization_context_middleware_service::OrganizationContextMiddlewareService;

#[derive(Clone)]
pub struct OrganizationContextMiddleware {
    organization_provider: Arc<dyn OrganizationProvider>,
}

impl OrganizationContextMiddleware {
    pub fn new(organization_provider: Arc<dyn OrganizationProvider>) -> Self {
        Self {
            organization_provider,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for OrganizationContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = OrganizationContextMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OrganizationContextMiddlewareService::new(
            Rc::new(service),
            self.organization_provider.clone(),
        )))
    }
}
