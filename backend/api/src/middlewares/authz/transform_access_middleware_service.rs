use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse},
};
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc};

use domain::domain_user::UserId;
use tracing::warn;

use crate::{middlewares::jwt::jwt_context::JwtContext, transforms::transforms_provider::TransformsProvider};

use super::transform_access_context::TransformAccessContext;

pub struct TransformAccessMiddlewareService<S> {
    pub service: Rc<S>,
    pub transforms_service: Arc<dyn TransformsProvider>,
}

impl<S, B> Service<ServiceRequest> for TransformAccessMiddlewareService<S>
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
        let transforms_service = Arc::clone(&self.transforms_service);

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

            let access_ctx = if jwt_ctx.is_admin {
                TransformAccessContext::default()
            } else {
                match transforms_service.list_accessible_transform_ids(jwt_ctx.user_id).await {
                    Ok(ids) => TransformAccessContext::new(ids.into_iter().collect()),
                    Err(error) => {
                        warn!(path = %req.path(), user_id = jwt_ctx.user_id, %error, "failed to resolve transform access set");
                        let res = req.into_response(
                            actix_web::HttpResponse::InternalServerError().body("failed to resolve transform access"),
                        );
                        return Ok(res.map_into_right_body());
                    }
                }
            };

            req.extensions_mut().insert(access_ctx);

            let path = req.path().to_owned();
            match srv.call(req).await {
                Ok(res) => Ok(res.map_into_left_body()),
                Err(error) => {
                    warn!(path = %path, %error, "downstream service returned error after transform access check");
                    Err(error)
                }
            }
        })
    }
}
