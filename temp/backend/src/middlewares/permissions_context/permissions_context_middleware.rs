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

use crate::access_control::access_control_provider::AccessControlProvider;
use crate::users::user_provider::UserProvider;

use super::permissions_context_middleware_service::PermissionsContextMiddlewareService;

#[derive(Clone)]
pub struct PermissionsContextMiddleware {
    access_control: Arc<dyn AccessControlProvider>,
    user_provider: Arc<dyn UserProvider>,
}

impl PermissionsContextMiddleware {
    pub fn new(
        access_control: Arc<dyn AccessControlProvider>,
        user_provider: Arc<dyn UserProvider>,
    ) -> Self {
        Self {
            access_control,
            user_provider,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PermissionsContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionsContextMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionsContextMiddlewareService::new(
            Rc::new(service),
            self.access_control.clone(),
            self.user_provider.clone(),
        )))
    }
}
