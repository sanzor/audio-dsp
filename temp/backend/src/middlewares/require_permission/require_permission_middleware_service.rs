use std::rc::Rc;

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

use super::required_permission_context::RequiredPermissionContext;

pub struct RequirePermissionMiddlewareService<S> {
    service: Rc<S>,
    permission: &'static str,
}

impl<S> RequirePermissionMiddlewareService<S> {
    pub fn new(service: Rc<S>, permission: &'static str) -> Self {
        Self {
            service,
            permission,
        }
    }
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMiddlewareService<S>
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
        let service = self.service.clone();
        let permission = self.permission;

        Box::pin(async move {
            let has_permission = req
                .extensions()
                .get::<PermissionsContext>()
                .map(|ctx| ctx.has(permission))
                .unwrap_or(false);

            if !has_permission {
                return Ok(req
                    .into_response(
                        HttpResponse::Forbidden()
                            .body(format!("missing required permission: {permission}")),
                    )
                    .map_into_right_body());
            }

            req.extensions_mut()
                .insert(RequiredPermissionContext { permission });

            service.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
