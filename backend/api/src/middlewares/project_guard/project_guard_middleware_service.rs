use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse},
};
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc};

use domain::project_role::ProjectRole;
use tracing::warn;

use crate::{
    memberships::memberships_provider::MembershipsProvider,
    middlewares::{
        jwt::jwt_context::JwtContext,
        role_context::role_context::RoleContext,
    },
    users::user_provider::UserProvider,
};

pub const PROJECT_ID_HEADER: &str = "x-project-id";

pub struct ProjectGuardMiddlewareService<S> {
    pub service: Rc<S>,
    pub memberships: Arc<dyn MembershipsProvider>,
    pub user_provider: Arc<dyn UserProvider>,
}

impl<S, B> Service<ServiceRequest> for ProjectGuardMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let memberships = Arc::clone(&self.memberships);
        let user_provider = Arc::clone(&self.user_provider);

        Box::pin(async move {
            if req.method() == actix_web::http::Method::OPTIONS {
                return srv.call(req).await.map(|r| r.map_into_left_body());
            }

            let jwt_ctx = req.extensions().get::<JwtContext>().cloned();
            let jwt_ctx = match jwt_ctx {
                Some(ctx) => ctx,
                None => {
                    let res = req.into_response(actix_web::HttpResponse::Unauthorized().body("Unauthorized"));
                    return Ok(res.map_into_right_body());
                }
            };

            // is_active check
            let user = match user_provider.get_user(jwt_ctx.user_id).await {
                Ok(u) => u,
                Err(e) => {
                    let res = req.into_response(actix_web::HttpResponse::InternalServerError().body(e.to_string()));
                    return Ok(res.map_into_right_body());
                }
            };
            match user {
                None => {
                    let res = req.into_response(actix_web::HttpResponse::Unauthorized().body("User not found"));
                    return Ok(res.map_into_right_body());
                }
                Some(u) if !u.is_active => {
                    let res = req.into_response(actix_web::HttpResponse::Forbidden().body("Account inactive"));
                    return Ok(res.map_into_right_body());
                }
                _ => {}
            }

            let role_ctx = if jwt_ctx.is_admin {
                RoleContext(ProjectRole::SuperAdmin)
            } else {
                let project_id: Option<i64> = req
                    .headers()
                    .get(PROJECT_ID_HEADER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<i64>().ok());

                match project_id {
                    None => {
                        let res = req.into_response(actix_web::HttpResponse::BadRequest().body("Missing X-Project-ID header"));
                        return Ok(res.map_into_right_body());
                    }
                    Some(pid) => {
                        let role = match memberships.get_role(pid, jwt_ctx.user_id).await {
                            Ok(role) => role,
                            Err(error) => {
                                warn!(
                                    path = %req.path(),
                                    project_id = pid,
                                    user_id = jwt_ctx.user_id,
                                    %error,
                                    "request rejected: failed to resolve project role"
                                );
                                let res = req.into_response(
                                    actix_web::HttpResponse::InternalServerError()
                                        .body("Failed to resolve project role"),
                                );
                                return Ok(res.map_into_right_body());
                            }
                        };
                        match role {
                            None => {
                                let res = req.into_response(actix_web::HttpResponse::Forbidden().body("Access denied to this project"));
                                return Ok(res.map_into_right_body());
                            }
                            Some(r) => RoleContext(r),
                        }
                    }
                }
            };

            req.extensions_mut().insert(role_ctx);
            srv.call(req).await.map(|r| r.map_into_left_body())
        })
    }
}
