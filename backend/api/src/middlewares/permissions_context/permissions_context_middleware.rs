use std::{
    collections::HashSet,
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use super::permissions_context::PermissionsContext;

/// Stub middleware — grants all listed permissions to every request.
/// Replace with a real ACL-backed implementation when access control is needed.
#[derive(Clone)]
pub struct PermissionsContextMiddleware {
    pub permissions: HashSet<String>,
}

impl PermissionsContextMiddleware {
    pub fn allow_all() -> Self {
        Self {
            permissions: HashSet::from([
                "users:create".to_string(),
                "users:read".to_string(),
                "users:update".to_string(),
                "users:delete".to_string(),
            ]),
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
        ready(Ok(PermissionsContextMiddlewareService {
            service: Rc::new(service),
            permissions: self.permissions.clone(),
        }))
    }
}

pub struct PermissionsContextMiddlewareService<S> {
    service: Rc<S>,
    permissions: HashSet<String>,
}

impl<S, B> Service<ServiceRequest> for PermissionsContextMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.extensions_mut().insert(PermissionsContext {
            permissions: self.permissions.clone(),
        });
        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
    }
}
