use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse},
};
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc};

use domain::workspace_role::WorkspaceRole;
use tracing::warn;

use crate::{
    memberships::memberships_provider::MembershipsProvider,
    middlewares::{
        jwt::jwt_context::JwtContext,
        membership::membership_context::{WorkspaceContext, RoleContext},
    },
};

pub const PROJECT_ID_HEADER: &str = "x-project-id";

pub struct MembershipMiddlewareService<S> {
    pub service: Rc<S>,
    pub memberships: Arc<dyn MembershipsProvider>,
}

impl<S, B> Service<ServiceRequest> for MembershipMiddlewareService<S>
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

        Box::pin(async move {
            if req.method() == actix_web::http::Method::OPTIONS {
                return srv.call(req).await.map(|r| r.map_into_left_body());
            }

            // 1. Identity from JWT (deposited by JwtAuthMiddleware)
            let jwt_ctx = req.extensions().get::<JwtContext>().cloned();
            let jwt_ctx = match jwt_ctx {
                Some(ctx) => ctx,
                None => {
                    let res = req.into_response(actix_web::HttpResponse::Unauthorized().body("Unauthorized"));
                    return Ok(res.map_into_right_body());
                }
            };

            // 2. Workspace id comes from a workspace-scoped route (`/{workspace_id}/...`)
            // whenever one is present. The header fallback keeps routes not yet
            // migrated to a workspace-scoped path on their existing contract.
            let workspace_id = req
                .match_info()
                .get("workspace_id")
                .and_then(|value| value.parse::<i32>().ok())
                .or_else(|| {
                    req.headers()
                        .get(PROJECT_ID_HEADER)
                        .and_then(|value| value.to_str().ok())
                        .and_then(|value| value.parse::<i32>().ok())
                });

            // 3. Resolve role
            let role_ctx = if jwt_ctx.is_admin {
                RoleContext(WorkspaceRole::SuperAdmin)
            } else {
                match workspace_id {
                    None => {
                        warn!(path = %req.path(), "request rejected: missing workspace_id (path or X-Project-ID header)");
                        let res = req.into_response(actix_web::HttpResponse::BadRequest().body("Missing X-Project-ID header"));
                        return Ok(res.map_into_right_body());
                    }
                    Some(wid) => {
                        let role = match memberships.get_role(wid, jwt_ctx.user_id).await {
                            Ok(role) => role,
                            Err(error) => {
                                warn!(
                                    path = %req.path(),
                                    workspace_id = wid,
                                    user_id = jwt_ctx.user_id,
                                    %error,
                                    "request rejected: failed to resolve workspace role"
                                );
                                let res = req.into_response(
                                    actix_web::HttpResponse::InternalServerError()
                                        .body("Failed to resolve workspace role"),
                                );
                                return Ok(res.map_into_right_body());
                            }
                        };
                        match role {
                            None => {
                                warn!(path = %req.path(), workspace_id = wid, user_id = jwt_ctx.user_id, "request rejected: access denied to workspace");
                                let res = req.into_response(actix_web::HttpResponse::Forbidden().body("Access denied to this workspace"));
                                return Ok(res.map_into_right_body());
                            }
                            Some(r) => RoleContext(r),
                        }
                    }
                }
            };

            // 4. Attach to request
            req.extensions_mut().insert(role_ctx);
            if let Some(wid) = workspace_id {
                req.extensions_mut().insert(WorkspaceContext(wid));
            }
            let path = req.path().to_owned();
            match srv.call(req).await {
                Ok(res) => {
                    if res.status().is_server_error() {
                        warn!(path = %path, status = %res.status(), "downstream service returned 5xx after membership check");
                    }
                    Ok(res.map_into_left_body())
                }
                Err(error) => {
                    warn!(path = %path, %error, "downstream service returned error after membership check");
                    Err(error)
                }
            }
        })
    }
}
