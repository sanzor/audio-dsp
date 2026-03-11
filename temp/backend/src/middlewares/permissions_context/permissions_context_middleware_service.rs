use std::{rc::Rc, sync::Arc};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    http::Method,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use tracing::warn;

use crate::access_control::access_control_provider::AccessControlProvider;
use crate::middlewares::jwt::jwt_context::JWTContext;
use crate::users::user_provider::UserProvider;

use super::permissions_context::PermissionsContext;

pub struct PermissionsContextMiddlewareService<S> {
    service: Rc<S>,
    access_control: Arc<dyn AccessControlProvider>,
    user_provider: Arc<dyn UserProvider>,
}

impl<S> PermissionsContextMiddlewareService<S> {
    pub fn new(
        service: Rc<S>,
        access_control: Arc<dyn AccessControlProvider>,
        user_provider: Arc<dyn UserProvider>,
    ) -> Self {
        Self {
            service,
            access_control,
            user_provider,
        }
    }
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
        let service = self.service.clone();
        let access_control = self.access_control.clone();
        let user_provider = self.user_provider.clone();

        Box::pin(async move {
            if req.method() == Method::OPTIONS {
                return service.call(req).await.map(|res| res.map_into_left_body());
            }

            // JWTContext was already deposited by the JWT middleware upstream
            let jwt_ctx = req.extensions().get::<JWTContext>().copied();

            let Some(jwt_ctx) = jwt_ctx else {
                return Ok(req
                    .into_response(HttpResponse::Unauthorized().body("JWT context missing"))
                    .map_into_right_body());
            };

            // Check if user is active
            match user_provider.get_user(jwt_ctx.user_id).await {
                Ok(Some(user)) if !user.is_active => {
                    return Ok(req
                        .into_response(
                            HttpResponse::Forbidden().body("user account is deactivated"),
                        )
                        .map_into_right_body());
                }
                Ok(None) => {
                    return Ok(req
                        .into_response(HttpResponse::Unauthorized().body("user not found"))
                        .map_into_right_body());
                }
                Err(e) => {
                    warn!("failed to check user status: {e}");
                    return Ok(req
                        .into_response(
                            HttpResponse::InternalServerError()
                                .body("failed to verify user status"),
                        )
                        .map_into_right_body());
                }
                _ => {} // user is active, continue
            }
            // If we have an org-scoped token, fetch permissions; otherwise empty set
            let permissions = match jwt_ctx.org_id {
                Some(org_id) => {
                    match access_control
                        .permissions_for_user(jwt_ctx.user_id, org_id)
                        .await
                    {
                        Ok(perms) => perms.into_iter().collect(),
                        Err(e) => {
                            warn!("failed to load permissions: {e}");
                            return Ok(req
                                .into_response(
                                    HttpResponse::InternalServerError()
                                        .body("failed to load permissions"),
                                )
                                .map_into_right_body());
                        }
                    }
                }
                None => Default::default(),
            };

            req.extensions_mut()
                .insert(PermissionsContext { permissions });

            service.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
