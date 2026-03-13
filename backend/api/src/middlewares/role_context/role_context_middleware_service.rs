use actix_web::{
    Error, HttpMessage, dev::{Service, ServiceRequest, ServiceResponse}
};
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc};

use crate::{
    middlewares::{
        jwt::jwt_context::JwtContext,
        role_context::role_context::RoleContext,
    },
    users::user_provider::UserProvider,
};

pub struct RoleContextMiddlewareService<S> {
    pub service: Rc<S>,
    pub user_provider: Arc<dyn UserProvider>,
}

impl<S, B> Service<ServiceRequest> for RoleContextMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let user_provider = Arc::clone(&self.user_provider);

        Box::pin(async move {
            if req.method() == actix_web::http::Method::OPTIONS {
                return srv.call(req).await;
            }

            let jwt_ctx = req.extensions().get::<JwtContext>().cloned();
            let jwt_ctx = match jwt_ctx {
                Some(ctx) => ctx,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
            };

            let user = user_provider.get_user_by_id(&jwt_ctx.user_id).await;
            match user {
                None => return Err(actix_web::error::ErrorUnauthorized("User not found")),
                Some(u) if !u.is_active => {
                    return Err(actix_web::error::ErrorForbidden("Account inactive"))
                }
                _ => {}
            }

            let role_ctx = if jwt_ctx.is_admin {
                RoleContext { role: None, is_admin: true }
            } else {
                RoleContext { role: jwt_ctx.role, is_admin: false }
            };

            req.extensions_mut().insert(role_ctx);
            srv.call(req).await
        })
    }
}
